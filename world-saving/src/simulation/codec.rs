use gridthorn::{GameCommandQueue, WorldSaveCodec};

pub struct EconomyCodec;

impl WorldSaveCodec<Vec<u64>, u64> for EconomyCodec {
    fn encode(&self, data: &Vec<u64>, commands: &GameCommandQueue<u64>) -> Result<String, String> {
        let mut commands = commands.clone();
        Ok(format!(
            "v1;{};{}",
            encode(data.iter().copied()),
            encode(commands.drain())
        ))
    }

    fn decode(&self, payload: &str) -> Result<(Vec<u64>, GameCommandQueue<u64>), String> {
        let fields = payload.split(';').collect::<Vec<_>>();
        if fields.len() != 3 || fields[0] != "v1" {
            return Err("expected economy payload v1 with data and commands".to_owned());
        }
        let mut queue = GameCommandQueue::new();
        for command in decode(fields[2])? {
            queue.push(command);
        }
        Ok((decode(fields[1])?, queue))
    }
}

fn encode(values: impl Iterator<Item = u64>) -> String {
    values
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn decode(source: &str) -> Result<Vec<u64>, String> {
    if source.is_empty() {
        return Ok(Vec::new());
    }
    source
        .split(',')
        .map(|value| {
            value
                .parse()
                .map_err(|_| format!("invalid economy integer: {value}"))
        })
        .collect()
}
