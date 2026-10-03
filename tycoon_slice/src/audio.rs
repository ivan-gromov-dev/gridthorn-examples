use gridthorn::{AudioClip, AudioCommandQueue, PlaybackSettings};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::JoinHandle;

enum Request {
    Pickup,
    Pause(bool),
    Shutdown,
}

struct Worker {
    sender: Option<mpsc::SyncSender<Request>>,
    thread: Mutex<Option<JoinHandle<()>>>,
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.sender.take();
        if let Some(thread) = self.thread.get_mut().expect("audio worker lock").take() {
            let _ = thread.join();
        }
    }
}

#[derive(Clone)]
pub(crate) struct Sound(Arc<Worker>);

impl Sound {
    pub fn new(native: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        let pickup = AudioClip::load_wav(path.join("pickup.wav"))?;
        let music = AudioClip::load_wav(path.join("music.wav"))?;
        let (sender, receiver) = mpsc::sync_channel(32);
        let thread = std::thread::spawn(move || {
            let mut output = if native {
                match gridthorn_audio::AudioOutput::new() {
                    Ok(output) => Some(output),
                    Err(error) => {
                        eprintln!("Audio unavailable: {error}; continuing silently");
                        None
                    }
                }
            } else {
                None
            };
            let mut queue = AudioCommandQueue::new();
            let music_voice = queue
                .play(
                    music,
                    PlaybackSettings::new(0.12, true).expect("music volume"),
                )
                .expect("music voice");
            let mut effect = None;
            while let Ok(request) = receiver.recv() {
                match request {
                    Request::Shutdown => break,
                    Request::Pickup => {
                        if let Some(voice) = effect {
                            queue.stop(voice);
                        }
                        effect = Some(
                            queue
                                .play(
                                    pickup.clone(),
                                    PlaybackSettings::new(0.35, false).expect("effect volume"),
                                )
                                .expect("effect voice"),
                        );
                    }
                    Request::Pause(paused) => {
                        if let Some(output) = output.as_mut() {
                            if paused {
                                output.suspend();
                            } else {
                                output.resume();
                            }
                        }
                    }
                }
                if let Some(output) = output.as_mut() {
                    if let Err(error) = output.process(&mut queue) {
                        eprintln!("Audio playback failed: {error}");
                    }
                } else {
                    let _ = queue.drain();
                }
            }
            queue.stop(music_voice);
            if let Some(voice) = effect {
                queue.stop(voice);
            }
            if let Some(output) = output.as_mut() {
                let _ = output.process(&mut queue);
            }
        });
        Ok(Self(Arc::new(Worker {
            sender: Some(sender),
            thread: Mutex::new(Some(thread)),
        })))
    }

    pub fn pickup(&self) {
        let _ = self
            .0
            .sender
            .as_ref()
            .expect("live audio worker")
            .try_send(Request::Pickup);
    }

    pub fn shutdown(&self) {
        let _ = self
            .0
            .sender
            .as_ref()
            .expect("live audio worker")
            .send(Request::Shutdown);
        if let Some(thread) = self.0.thread.lock().expect("audio worker lock").take() {
            let _ = thread.join();
        }
    }
    pub fn pause(&self, paused: bool) {
        let _ = self
            .0
            .sender
            .as_ref()
            .expect("live audio worker")
            .try_send(Request::Pause(paused));
    }
}
