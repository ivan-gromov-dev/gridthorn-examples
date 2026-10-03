fn main() -> Result<(), Box<dyn std::error::Error>> {
    tycoon_slice::run_headless(&std::env::args().skip(1).collect::<Vec<_>>())
}
