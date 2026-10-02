use super::parse_arguments;

#[test]
fn accepts_zero_ticks_and_maximum_seed() {
    let args = [
        "--scenario",
        "economy",
        "--ticks",
        "0",
        "--seed",
        "18446744073709551615",
    ]
    .map(str::to_owned);
    let request = parse_arguments(&args).unwrap();
    assert_eq!(request.scenario, "economy");
    assert_eq!(request.ticks, 0);
    assert_eq!(request.seed, u64::MAX);
}

#[test]
fn rejects_missing_extra_malformed_and_overflowing_arguments() {
    for args in [
        vec![],
        vec!["--scenario", "economy"],
        vec!["--scenario", "economy", "--ticks", "-1", "--seed", "42"],
        vec![
            "--scenario",
            "economy",
            "--ticks",
            "18446744073709551616",
            "--seed",
            "42",
        ],
        vec![
            "--scenario",
            "economy",
            "--ticks",
            "10",
            "--seed",
            "invalid",
        ],
        vec!["--scenario", "economy", "--other", "10", "--seed", "42"],
        vec![
            "--scenario",
            "economy",
            "--ticks",
            "10",
            "--seed",
            "42",
            "extra",
        ],
    ] {
        assert!(parse_arguments(&args.into_iter().map(str::to_owned).collect::<Vec<_>>()).is_err());
    }
}
