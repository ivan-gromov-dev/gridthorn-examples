use super::{ArgumentError, HeadlessArguments};

#[test]
fn uses_default_tick_count_without_arguments() {
    let arguments = HeadlessArguments::parse(Vec::new()).expect("default arguments should parse");

    assert_eq!(arguments.fixed_steps, 3);
}

#[test]
fn parses_explicit_tick_count() {
    let arguments = HeadlessArguments::parse(["--ticks".to_owned(), "12".to_owned()])
        .expect("explicit tick count should parse");

    assert_eq!(arguments.fixed_steps, 12);
}

#[test]
fn rejects_invalid_tick_count() {
    let result = HeadlessArguments::parse(["--ticks".to_owned(), "many".to_owned()]);

    assert_eq!(
        result,
        Err(ArgumentError::InvalidTickCount("many".to_owned()))
    );
}
