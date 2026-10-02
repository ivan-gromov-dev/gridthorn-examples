use gridthorn_example_scenarios_snapshots::{request, simulation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let request = request::parse()?;
    simulation::run(&request.scenario, request.ticks, request.seed)
}
