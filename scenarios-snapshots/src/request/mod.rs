mod errors;

use errors::RequestError;

pub struct Request {
    pub scenario: String,
    pub ticks: u64,
    pub seed: u64,
}

/// Parse the CLI's explicit headless launch contract before constructing a runtime.
///
/// # Errors
/// Returns an error for malformed flags or unsigned integer inputs.
pub fn parse() -> Result<Request, RequestError> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    parse_arguments(&args)
}

fn parse_arguments(args: &[String]) -> Result<Request, RequestError> {
    if args.len() != 6 || args[0] != "--scenario" || args[2] != "--ticks" || args[4] != "--seed" {
        return Err(RequestError);
    }
    Ok(Request {
        scenario: args[1].clone(),
        ticks: args[3].parse().map_err(|_| RequestError)?,
        seed: args[5].parse().map_err(|_| RequestError)?,
    })
}

#[cfg(test)]
mod test;
