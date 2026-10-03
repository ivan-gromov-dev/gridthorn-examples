# Provisional localization runtime example

Run without a window, GPU, system fonts or operating-system language detection:

```console
cargo run -p gridthorn_example_localization --locked
cargo test -p gridthorn_example_localization --locked
```

The game loads validated UTF-8 Fluent assets through `gridthorn::localization`,
switches between English, Russian, Egyptian Arabic and Japanese, supplies text
and numeric parameters, formats plural/select messages and localized numbers,
and resolves a missing translation through the explicit English fallback.
The output includes the locale that supplied each message and preserves bidi
isolation around parameters. The last step checks rejected malformed catalog
data, explicit replacement and missing-message diagnostics.

This validates the headless runtime API. It does not validate native font
rendering or a language menu. See the engine's `docs/LOCALIZATION.md` for the
supported Fluent subset, precision bounds and formatting exclusions.
