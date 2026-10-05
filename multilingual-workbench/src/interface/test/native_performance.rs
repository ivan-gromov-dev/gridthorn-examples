use super::*;

#[test]
fn disabled_and_bounded_collection_reset_phase_totals() {
    let mut profile = NativePerformance {
        enabled: false,
        active: false,
        current: [Duration::ZERO; 8],
        samples: Vec::new(),
    };
    assert!(profile.start().is_none());
    profile.finish(1);
    assert_eq!(profile.samples, []);
    profile.enabled = true;
    profile.begin();
    profile.current[Phase::Layout as usize] = Duration::from_micros(17);
    profile.finish(2);
    assert_eq!(
        profile.samples[0].1[Phase::Layout as usize],
        Duration::from_micros(17)
    );
    assert_eq!(profile.current, [Duration::ZERO; 8]);
    for frame in 3..300 {
        profile.begin();
        profile.finish(frame);
    }
    assert_eq!(profile.samples.len(), LIMIT);
    assert!(profile.start().is_none());
    profile.enabled = false;
}
