use anyhow::Result;
use argh::FromArgs;
use humantime::format_duration;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(FromArgs)]
/// Rename Rocket League replay files.
struct Args {
    /// print output but do not rename
    #[argh(switch, short = 'n')]
    dry_run: bool,

    /// suppress output
    #[argh(switch, short = 'q')]
    quiet: bool,

    /// directory containing replay files
    #[argh(positional)]
    directory: String,
}

#[derive(Serialize, Deserialize)]
struct ReplayData {
    properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Properties {
    TeamSize: u8,
    Team0Score: Option<u8>,
    Team1Score: Option<u8>,
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
            "{} - {} - {} ({}) - {}-{} - {}.replay",
            self.Date,
            mode_name(self),
            self.MapName,
            self.MatchType,
            self.Team0Score.unwrap_or_default(),
            self.Team1Score.unwrap_or_default(),
            game_length(self),
        )
    }
}

fn parse(filename: &PathBuf) -> Result<Properties> {
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

fn rename_dir(dir: &str, args: &Args) -> Result<()> {
    let files = fs::read_dir(dir)?;

    files.par_bridge().for_each(|path| {
        let path = match path {
            Ok(path) => path.path(),
            Err(_) => return,
        };
        let parent = path.parent().unwrap();
        let filename = path.display().to_string();
        if !filename.ends_with(".replay") {
            return;
        }

        let p = match parse(&path) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("{}: {}", path.display(), e);
                return;
            }
        };

        let output_path = parent.join(p.to_string());

        if !args.quiet {
            println!("{} -> {}", path.display(), output_path.display());
        }

        if !args.dry_run {
            match fs::rename(&path, &output_path) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}: {}", path.display(), e);
                }
            };
        }
    });

    Ok(())
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    rename_dir(&args.directory, &args)?;

    Ok(())
}
