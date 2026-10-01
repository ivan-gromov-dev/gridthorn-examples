mod inspection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    inspection::run()
}
