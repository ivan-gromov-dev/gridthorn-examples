mod messages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    messages::run()
}
