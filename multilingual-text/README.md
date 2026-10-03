# Multilingual text

Provisional public SDK example: game-owned font assets, script fallback, Unicode
shaping, mixed bidi, combining marks, ligatures, measurement, wrapping and DPI.

```console
cargo run -p gridthorn_example_multilingual_text
cargo run -p gridthorn_example_multilingual_text -- --headless
cargo run -p gridthorn_example_multilingual_text -- --smoke
```

The native window displays Cyrillic, Arabic/Latin/numbers and Japanese. Resize it
to exercise logical wrapping; move between monitors with different scaling to
exercise `WindowScaleFactor` and raster rebuilding. Escape exits. `--smoke` exits
after 120 presentation frames. `--headless` validates four raster scales without
window/GPU creation. IME input is demonstrated separately in `text-input`.

Fonts are unmodified Noto Sans Regular and Noto Sans Arabic Regular from
https://github.com/notofonts/noto-fonts/tree/main/hinted/ttf, and Noto Sans JP Regular
from https://github.com/notofonts/noto-cjk/tree/main/Sans/SubsetOTF/JP. Their SIL Open
Font Licenses are in `assets/fonts/OFL.txt` and `assets/fonts/OFL-CJK.txt`.

See the engine [text contract](../../gridthorn-engine/docs/TEXT.md) for tested
coverage, budgets and native/performance deferrals. Fonts stay in this example;
the engine does not embed default runtime fonts.
