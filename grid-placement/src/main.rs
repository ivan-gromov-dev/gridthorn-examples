mod placement;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    placement::run()
}
