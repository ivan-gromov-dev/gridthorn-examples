mod playback;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    playback::run()
}
