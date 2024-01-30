use anyhow::Result;
use boxcars::Replay;
use std::fs;
use std::io::{self};

fn parse_rl(data: &[u8]) -> Result<Replay> {
    Ok(boxcars::ParserBuilder::new(data).parse()?)
}

fn run(filename: &str) -> Result<()> {
    let buffer = fs::read(filename)?;
    let replay = parse_rl(&buffer)?;
    serde_json::to_writer(&mut io::stdout(), &replay)?;

    Ok(())
}

fn main() -> Result<()> {
    let filename = "/home/max/Downloads/RL Replays/047A6ADE47C071EED2898E96B5F2D773.replay";
    run(filename)?;

    Ok(())
}
