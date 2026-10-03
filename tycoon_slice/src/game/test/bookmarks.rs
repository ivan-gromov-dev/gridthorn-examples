use crate::game::session::Session;

#[test]
fn versioned_camera_scene_round_trips_and_rejects_invalid_geometry() {
    let mut session = Session::new("harbor").unwrap();
    session.camera = [123.0, -456.0];
    session.height = 800.0;
    session.square = true;
    let source = crate::game::view_bookmark::capture(&session).unwrap();
    session.camera = [0.0, 0.0];
    session.square = false;
    crate::game::view_bookmark::restore(&mut session, &source).unwrap();
    assert_eq!(session.camera, [123.0, -456.0]);
    assert!(session.square);
    assert!(
        crate::game::view_bookmark::restore(&mut session, &source.replace("123.0", "9999.0"))
            .is_err()
    );
    assert_eq!(session.camera, [123.0, -456.0]);
}
