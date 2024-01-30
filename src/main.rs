use anyhow::Result;
use boxcars::Replay;
use std::fs;

// #[derive(Serialize, Deserialize)]
struct Properties {
    TeamSize: u8,
    Team0Score: u8,
    Team1Score: u8,
    RecordFPS: u8,
    MapName: String,
    Date: String,
    NumFrames: u32,
    MatchType: String,
}

fn parse_rl(data: &[u8]) -> Result<Replay> {
    Ok(boxcars::ParserBuilder::new(data).parse()?)
}

fn run(filename: &str) -> Result<()> {
    let buffer = fs::read(filename)?;
    let replay = parse_rl(&buffer)?;
    println!("{:?}", replay.properties["TeamSize"]);

    Ok(())
}

fn main() -> Result<()> {
    let filename = "/home/max/Downloads/RL Replays/28E4E0FE49754D401B77288664EC770A.replay";
    run(filename)?;

    Ok(())
}
