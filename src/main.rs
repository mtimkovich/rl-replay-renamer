use anyhow::Result;
use humantime::format_duration;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct ReplayData {
    properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Properties {
    TeamSize: u8,
    Team0Score: u8,
    Team1Score: u8,
    RecordFPS: f32,
    MapName: String,
    Date: String,
    NumFrames: u32,
    MatchType: String,
}

impl fmt::Display for Properties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} | {} | {} | {}-{} | {}.replay",
            self.Date,
            mode_name(self),
            self.MapName,
            self.Team0Score,
            self.Team1Score,
            game_length(self),
        )
    }
}

fn parse(filename: &str) -> Result<Properties> {
    let buffer = fs::read(filename)?;
    let file = boxcars::ParserBuilder::new(&buffer).parse()?;
    // I hope this isn't the best way to do this lol.
    let data = serde_json::to_string(&file)?;
    let replay: ReplayData = serde_json::from_str(&data)?;

    Ok(replay.properties)
}

fn mode_name(p: &Properties) -> String {
    format!("{}v{}", p.TeamSize, p.TeamSize)
}

fn game_length(p: &Properties) -> String {
    let length = p.NumFrames as f32 / p.RecordFPS;
    let duration = Duration::new(length as u64, 0);
    format_duration(duration).to_string()
}

fn main() -> Result<()> {
    let filename = "/home/max/Downloads/RL Replays/28E4E0FE49754D401B77288664EC770A.replay";
    let p = parse(filename)?;
    println!("{}", p);

    Ok(())
}
