use std::sync::{Arc, Mutex, mpsc};

use super::super::{PauseState, Request, Sound, Worker};

fn saturated_delivery(states: &[bool]) -> bool {
    let (sender, receiver) = mpsc::sync_channel(32);
    let (release_sender, release_receiver) = mpsc::channel();
    let pause = Arc::new(PauseState::default());
    let consumer_pause = Arc::clone(&pause);
    let thread = std::thread::spawn(move || {
        release_receiver.recv().unwrap();
        let mut applied = None;
        for _ in 0..32 {
            receiver.recv().unwrap();
            if let Some(paused) = consumer_pause.take() {
                applied = Some(paused);
            }
        }
        applied.unwrap()
    });
    let sound = Sound(Arc::new(Worker {
        pause,
        sender: Some(sender),
        thread: Mutex::new(None),
    }));
    for _ in 0..32 {
        sound
            .0
            .sender
            .as_ref()
            .unwrap()
            .try_send(Request::Pickup)
            .unwrap();
    }
    for &state in states {
        sound.pause(state);
    }
    assert!(matches!(
        sound.0.sender.as_ref().unwrap().try_send(Request::Wake),
        Err(mpsc::TrySendError::Full(_))
    ));
    release_sender.send(()).unwrap();
    thread.join().unwrap()
}

#[test]
fn full_effect_queue_preserves_pause_and_latest_resume() {
    assert!(saturated_delivery(&[true]));
    assert!(!saturated_delivery(&[true, false]));
    assert!(saturated_delivery(&[false, true, false, true]));
}

#[test]
fn idle_worker_is_woken_and_stale_wakes_do_not_restore_old_state() {
    let (sender, receiver) = mpsc::sync_channel(32);
    let sound = Sound(Arc::new(Worker {
        pause: Arc::new(PauseState::default()),
        sender: Some(sender),
        thread: Mutex::new(None),
    }));
    sound.pause(true);
    sound.pause(false);
    assert!(matches!(receiver.recv().unwrap(), Request::Wake));
    assert_eq!(sound.0.pause.take(), Some(false));
    assert!(matches!(receiver.recv().unwrap(), Request::Wake));
    assert_eq!(sound.0.pause.take(), None);
    sound.pause(true);
    assert!(matches!(receiver.recv().unwrap(), Request::Wake));
    assert_eq!(sound.0.pause.take(), Some(true));
}
