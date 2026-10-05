use std::{
    hint::black_box,
    sync::{Arc, Mutex, mpsc},
    time::Instant,
};

use super::super::{PauseState, Request, Sound, Worker};

/// Exercises the example's headless worker and its bounded transport separately.
#[test]
#[ignore = "manual audio worker probe; run alone with --release"]
fn measure_audio_worker_handoff() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("audio_worker,requests,batch,phase,elapsed_ns,accepted,dropped");
    for requests in [1, 32, 1024, 16_384] {
        for batch in 0..22 {
            let sound = Sound::new(false).unwrap();
            let start = Instant::now();
            for _ in 0..requests {
                sound.pickup();
            }
            let enqueue = start.elapsed().as_nanos();
            let start = Instant::now();
            sound.shutdown();
            let shutdown = start.elapsed().as_nanos();
            assert!(sound.0.thread.lock().unwrap().is_none());
            if batch >= 2 {
                println!(
                    "audio_worker,{requests},{batch},headless_enqueue,{enqueue},unknown,unknown"
                );
                println!(
                    "audio_worker,{requests},{batch},headless_shutdown,{shutdown},unknown,unknown"
                );
            }
            transport(requests, batch);
        }
    }
}

fn transport(requests: usize, batch: usize) {
    let (sender, receiver) = mpsc::sync_channel(32);
    let (ready_sender, ready_receiver) = mpsc::channel();
    let (release_sender, release_receiver) = mpsc::channel();
    let pause = Arc::new(PauseState::default());
    let worker_pause = Arc::clone(&pause);
    let worker = std::thread::spawn(move || {
        ready_sender.send(()).unwrap();
        release_receiver.recv().unwrap();
        let mut pickups = 0;
        let mut applied = None;
        while let Ok(request) = receiver.recv() {
            if let Some(state) = worker_pause.take() {
                applied = Some(state);
            }
            match request {
                Request::Pickup => pickups += 1,
                Request::Shutdown => break,
                Request::Wake => panic!("wake must encounter a full queue"),
            }
        }
        (pickups, applied)
    });
    ready_receiver.recv().unwrap();
    let mut accepted = 0;
    let start = Instant::now();
    for _ in 0..requests {
        if sender.try_send(Request::Pickup).is_ok() {
            accepted += 1;
        }
    }
    let elapsed = start.elapsed().as_nanos();
    assert_eq!(accepted, requests.min(32));
    let mut pause_elapsed = None;
    if requests >= 32 {
        assert!(matches!(
            sender.try_send(Request::Wake),
            Err(mpsc::TrySendError::Full(_))
        ));
        let sound = Sound(Arc::new(Worker {
            pause,
            sender: Some(sender.clone()),
            thread: Mutex::new(None),
        }));
        let start = Instant::now();
        sound.pause(true);
        sound.pause(false);
        pause_elapsed = Some(start.elapsed().as_nanos());
    }
    release_sender.send(()).unwrap();
    let start = Instant::now();
    sender.send(Request::Shutdown).unwrap();
    let processed = worker.join().unwrap();
    let shutdown = start.elapsed().as_nanos();
    assert_eq!(processed.0, accepted);
    assert_eq!(processed.1, if requests >= 32 { Some(false) } else { None });
    if batch >= 2 {
        let dropped = requests - accepted;
        if let Some(elapsed) = pause_elapsed {
            println!(
                "audio_worker,{requests},{batch},saturated_pause_resume,{elapsed},{accepted},{dropped}"
            );
        }
        println!(
            "audio_worker,{requests},{batch},blocked_worker_enqueue,{elapsed},{accepted},{dropped}"
        );
        println!(
            "audio_worker,{requests},{batch},transport_shutdown,{shutdown},{accepted},{dropped}"
        );
    }
}
