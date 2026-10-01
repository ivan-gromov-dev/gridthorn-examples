#[test]
fn file_changes_reach_new_frames_and_failed_edits_preserve_existing_snapshots() {
    super::run().expect("asset reload smoke must pass");
}
