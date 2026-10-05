mod interface;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|argument| argument == "--performance") {
        return interface::performance();
    }
    if std::env::args().any(|argument| argument == "--headless") {
        return interface::headless();
    }
    let mut workload = None;
    for (flag, mode) in [
        ("--smoke", interface::SmokeWorkload::Mixed),
        ("--idle-smoke", interface::SmokeWorkload::Idle),
        ("--editing-smoke", interface::SmokeWorkload::Editing),
        ("--slider-smoke", interface::SmokeWorkload::Slider),
        ("--locale-smoke", interface::SmokeWorkload::Locale),
        ("--scroll-smoke", interface::SmokeWorkload::Scroll),
        ("--windows-smoke", interface::SmokeWorkload::Windows),
        ("--animation-smoke", interface::SmokeWorkload::Animation),
        ("--selection-smoke", interface::SmokeWorkload::Selection),
        ("--preedit-smoke", interface::SmokeWorkload::Preedit),
    ] {
        if std::env::args().any(|argument| argument == flag) {
            if workload.is_some() {
                return Err("choose one smoke workload".into());
            }
            workload = Some(mode);
        }
    }
    let locale = match std::env::args()
        .find_map(|argument| argument.strip_prefix("--locale=").map(str::to_owned))
        .as_deref()
    {
        None | Some("en-US") => 0,
        Some("ru") => 1,
        Some("ar-EG") => 2,
        Some("ja") => 3,
        Some(_) => return Err("locale must be en-US, ru, ar-EG or ja".into()),
    };
    let long = std::env::args().any(|argument| argument == "--long-smoke");
    if long
        && !matches!(
            workload,
            Some(
                interface::SmokeWorkload::Idle
                    | interface::SmokeWorkload::Editing
                    | interface::SmokeWorkload::Windows
                    | interface::SmokeWorkload::Animation
            )
        )
    {
        return Err("long smoke requires idle, editing, windows or animation workload".into());
    }
    gridthorn::WindowedApplication::new(
        gridthorn::WindowConfig {
            title: "Gridthorn · Multilingual workbench".into(),
            width: 1000,
            height: if workload == Some(interface::SmokeWorkload::Scroll) {
                400
            } else {
                800
            },
        },
        interface::runtime(workload, locale, if long { 1200 } else { 120 })?,
    )
    .run()?;
    Ok(())
}
