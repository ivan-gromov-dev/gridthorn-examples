mod persistence;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    persistence::run()
}
