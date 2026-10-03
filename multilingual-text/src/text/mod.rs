use gridthorn::{
    ApplicationRuntime, Color, ExitRequest, FontAsset, InputState, KeyCode, RasterText,
    RenderFrame, ScheduleBuilder, ScheduleStage, TextError, TextStyle, TextSystem,
    WindowScaleFactor, WindowViewport,
};

const CONTENT: &str = "Font assets · Unicode · DPI\n\nПривет, мир! Кириллица: Ёжик и съёмка.\n\nArabic + Latin: العربية 123 — مرحبا بالعالم\n\n日本語: 日本の文字、かなとカタカナ。\n\nCombining marks: e\u{301} a\u{308} · Ligatures: office ffi\n\nWrapping: длинная строка переносится по границам слов; extraordinaryextraordinaryextraordinary тоже помещается.\n\nResize the window or move between monitors. Escape exits.";

struct Presentation {
    text: TextSystem,
    snapshot: Option<RasterText>,
    extent: [u32; 2],
    dpi: f64,
    frames: u32,
}

fn service() -> Result<TextSystem, Box<dyn std::error::Error>> {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/fonts");
    let fonts = [
        "NotoSans-Regular.ttf",
        "NotoSansArabic-Regular.ttf",
        "NotoSansJP-Regular.otf",
    ]
    .into_iter()
    .map(|name| FontAsset::load(directory.join(name)))
    .collect::<Result<Vec<_>, _>>()?;
    Ok(TextSystem::new("en-US", &fonts)?)
}

impl Presentation {
    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        reason = "window pixels and DPI are validated by the text service"
    )]
    fn prepare(&mut self, viewport: WindowViewport, dpi: f64) -> Result<(), TextError> {
        if self.extent == [viewport.width, viewport.height] && self.dpi.to_bits() == dpi.to_bits() {
            return Ok(());
        }
        let mut style = TextStyle::new("Noto Sans", 23.0);
        style.width = Some((viewport.width as f32 / dpi as f32 - 48.0).max(1.0));
        let layout = self.text.layout(CONTENT, &style)?;
        if layout.missing_glyphs() != 0 {
            eprintln!("Missing glyphs: {}", layout.missing_glyphs());
        }
        self.snapshot = Some(
            self.text
                .rasterize(&layout, dpi as f32, Color::rgb(0.9, 0.95, 1.0))?
                .at([24.0, 24.0])?,
        );
        self.extent = [viewport.width, viewport.height];
        self.dpi = dpi;
        println!(
            "Layout: {} lines, {:.1}×{:.1} logical pixels; DPI {dpi}",
            layout.lines().len(),
            layout.measurement().width,
            layout.measurement().height
        );
        Ok(())
    }
}

/// Compose multilingual presentation using only public SDK services.
pub(crate) fn runtime(smoke: bool) -> Result<ApplicationRuntime, Box<dyn std::error::Error>> {
    let presentation = Presentation {
        text: service()?,
        snapshot: None,
        extent: [0, 0],
        dpi: 0.0,
        frames: 0,
    };
    let mut schedules = ScheduleBuilder::new();
    let mut initial = Some(presentation);
    schedules.add_system(ScheduleStage::Startup, move |world| {
        world.insert_resource(initial.take().expect("startup once"));
    });
    schedules.add_system(ScheduleStage::Update, move |world| {
        let viewport = world
            .read_resource(|value: &WindowViewport| *value)
            .unwrap_or_default();
        let dpi = world
            .read_resource(|value: &WindowScaleFactor| value.0)
            .unwrap_or(1.0);
        let result = world.update_resource_with(|state: &mut Presentation| {
            state.frames += 1;
            state.prepare(viewport, dpi)
        });
        if let Some(Err(error)) = result {
            eprintln!("Text preparation failed: {error}");
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
        let finished = smoke
            && world
                .read_resource(|state: &Presentation| state.frames >= 120)
                .unwrap_or(false);
        let escape = world
            .read_resource(|input: &InputState| input.key_just_pressed(KeyCode::Escape))
            .unwrap_or(false);
        if finished || escape {
            world.update_resource(|exit: &mut ExitRequest| exit.request());
        }
    });
    schedules.add_system(ScheduleStage::Render, |world| {
        let snapshot = world
            .read_resource(|state: &Presentation| state.snapshot.clone())
            .flatten();
        world.insert_resource(
            RenderFrame::default().with_ui(snapshot.into_iter().map(Into::into).collect()),
        );
    });
    Ok(ApplicationRuntime::new(schedules.build()))
}

/// Exercise fallback, measurement, wrapping and repeated DPI rasterization without GPU.
pub(crate) fn headless() -> Result<(), Box<dyn std::error::Error>> {
    let mut text = service()?;
    let mut style = TextStyle::new("Noto Sans", 23.0);
    style.width = Some(520.0);
    let layout = text.layout(CONTENT, &style)?;
    assert_eq!(layout.missing_glyphs(), 0);
    assert!(layout.lines().len() > 12);
    for scale in [1.0, 1.25, 1.5, 2.0] {
        let raster = text
            .rasterize(&layout, scale, Color::default())?
            .at([24.0, 24.0])?;
        assert_eq!(raster.measurement(), layout.measurement());
        println!(
            "DPI {scale}: {} antialiased samples, {} lines",
            raster.pixel_count(),
            layout.lines().len()
        );
    }
    Ok(())
}
