# Timber Harbor assets

- `harbor-atlas.png`: original transparent RGBA artwork created for this example
  with the built-in ImageGen tool on 2026-10-02. Actual output is 1254Ã—1254;
  source rectangles isolate the authored silhouettes in `presentation::region`.
  Contains grass, stone road, water, shoreline, sawmill, workshop, cottage, dock,
  pine grove, sailboat, two worker frames, logs, crates, and placement indicators.
- `music.wav`: original 24-second C-major mallet/bass harbor theme with a gentle
  procedural tide bed, mono PCM16 at 22,050 Hz.
- `pickup.wav`: original one-second ascending delivery chime, same PCM format.
- `generate_audio.py`: reproducible standard-library synthesis for both WAV files.
  Run `python assets/generate_audio.py` only when regenerating audio; Python is
  not required to play.
- `skin.txt`: raw asset source explicitly depending on all three atlases, used to expose
  dependency propagation and atomic development reload in the game.

## Image generation prompt

Built-in tool mode with `transparent_background=true`; no reference images.

```text
Use case: stylized-concept. Asset type: production game sprite atlas for a cozy isometric harbor tycoon named Timber Harbor. Generate a square 1024x1024 RGBA transparent atlas, EXACT 4 columns by 4 rows of equal 256x256 cells. Each cell contains ONE centered isolated sprite with generous transparent padding, no text, no grid lines, no overlap. Consistent orthographic isometric 2:1 view (diamond horizontal width twice height), sunlight upper left, charming detailed hand-painted pixel-art aesthetic, teal/pine/ochre/coral palette. Every ground diamond has width 220 pixels and height 110 pixels, centered at x128 y184; buildings sit on identical diamond footprints, bottom footprint tip at y239. Row 1 left to right: grass diamond tile with tiny flowers; cobblestone road diamond tile; shallow turquoise water diamond tile; sandy shoreline diamond tile. Row 2: small timber sawmill with red roof and stacked logs; blue-roof carpenter workshop with chimney and crates; cozy cream cottage with coral roof; wooden dock warehouse with teal roof and shipping crates. Row 3: clustered pine trees on grass diamond; small shipping sailboat with cream sail; worker in ochre clothes facing southeast; same worker walking southeast alternate step. Row 4: timber log pile; stacked finished wooden crates; translucent emerald green diamond placement outline; translucent coral red diamond placement outline. Sprite assets only. Whole background fully transparent. No labels or lettering. Crisp silhouette, detailed roofs and timber materials, inviting polished indie game art.
```

## Workforce expansion and UI artwork

Both additions were generated with the built-in ImageGen tool on 2026-10-02,
`transparent_background=true`, and copied into the example without editing the
source outputs. Both are 1774×887 RGBA. Authored rectangles in `presentation::skin`
and `presentation::world` isolate sprites without requiring runtime generation.

- `harbor-expansion.png`: log/plank warehouses and two walking poses each for
  lumberjack, carpenter and porter. Reference: the existing `harbor-atlas.png`.
  Generation output: `exec-27789c74-d0fa-4247-ad4c-df1690eb48eb.png`.
- `harbor-ui.png`: menu panel, normal/selected/hover frames, gold/log/plank/housing
  icons. Generated without image references.
  Generation output: `exec-027f39f4-0fd2-41b4-8312-41c7c5587e1d.png`.

Expansion prompt:

```text
Generate an expansion sprite atlas for this isometric harbor game, matching the attached painterly detailed medieval cozy game sprites. Transparent background. Strict uniform grid 4 columns by 2 rows, each isolated asset centered within its cell with generous margins, no text, no labels, no grid lines. Top row: raw log warehouse with stacks of round logs and brown roof; processed plank warehouse with orderly sawn boards and blue roof; lumberjack in green tunic with axe walking pose A; lumberjack same character walking pose B. Bottom row: carpenter wearing blue apron carrying finished boards walking pose A; same carpenter pose B; porter wearing burgundy vest carrying a crate walking pose A; same porter pose B. Buildings entire isometric diamond bases and roof visible. Workers full body, consistent scale, facing lower right. Every sprite entirely within its exact cell, no overlap. Output landscape atlas aspect ratio 2:1.
```

UI prompt:

```text
A polished game UI sprite atlas for a cozy painterly medieval timber harbor management game. Transparent background, no words no letters no numerals. 4 columns by 2 rows, strict separate uniformly sized cells with padding, landscape 2:1 aspect. Top row: ornate dark navy and teal wood menu panel with brass corner brackets and blank dark center, rectangular framed button normal state, matching button illuminated gold selected state, matching button teal hover state. Bottom row: icon stack of gold coins; icon bundle of raw round logs; icon bundle of sawn light wood planks; icon small cream plaster red-roof workers house with burgundy worker silhouette badge. Keep panels centered and all graphic elements completely within each cell, edges readable and clean, richly textured painted materials consistent with isometric fantasy town sprites. Panel center blank and very dark for cream text.
```
