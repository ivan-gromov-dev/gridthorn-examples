mod simulation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simulation::validate()
}
