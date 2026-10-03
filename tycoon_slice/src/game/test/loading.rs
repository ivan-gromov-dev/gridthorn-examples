use crate::game::session::Session;

#[test]
fn missing_market_stream_is_rejected_with_full_world_rollback() {
    let mut bad = Session::new("harbor").unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../target");
    std::fs::create_dir_all(&root).unwrap();
    bad.save_path = root.join(format!("harbor-missing-rng-{}.toml", std::process::id()));
    bad.runner
        .world()
        .update_resource(|root: &mut crate::economy::Root| {
            root.random = gridthorn::RandomStreams::new(42);
        });
    bad.save().unwrap();
    let mut live = Session::new("harbor").unwrap();
    live.advance().unwrap();
    live.save_path = bad.save_path.clone();
    let before = live
        .runner
        .save_document(&crate::economy::codec::HarborCodec)
        .unwrap();
    assert!(live.load().is_err());
    assert_eq!(
        live.runner
            .save_document(&crate::economy::codec::HarborCodec)
            .unwrap(),
        before
    );
    std::fs::remove_file(bad.save_path).unwrap();
}
