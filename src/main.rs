use anyhow::Result;
use argh::FromArgs;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

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

fn format_duration(duration: Duration) -> String {
    let mut secs = duration.as_secs();
    if secs == 0 {
        let ms = duration.as_millis();
        return format!("{}ms", ms);
    } else if secs < 60 {
        return format!("{}s", secs);
    }

    let minutes = secs / 60;
    secs = secs % 60;
    return format!("{}m {}s", minutes, secs);
}

fn game_length(p: &Properties) -> String {
    let length = p.NumFrames as f32 / p.RecordFPS;
    let duration = Duration::new(length as u64, 0);
    format_duration(duration)
}

static REPLAY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[A-F0-9]+\.replay").unwrap());

fn rename_file(path: PathBuf, args: &Args) -> u64 {
    let parent = path.parent().unwrap();
    let filename = path.file_name().unwrap().to_str().unwrap();
    if !REPLAY_REGEX.is_match(&filename) {
        // Ignore already renamed replays.
        return 0;
    }

    let props = match parse(&path) {
        Ok(props) => props,
        Err(e) => {
            eprintln!("{}: {}", path.display(), e);
            return 0;
        }
    };

    let output_path = parent.join(props.to_string());

    if !args.quiet {
        println!("{} -> {}", path.display(), output_path.display());
    }

    if !args.dry_run {
        match fs::rename(&path, &output_path) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}: {}", path.display(), e);
                return 0;
            }
        };
    }

    return 1;
}

fn rename_dir(args: &Args) {
    let files = match fs::read_dir(&args.directory) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let start = Instant::now();

    let count: u64 = files
        .par_bridge()
        .map(|p| match p {
            Ok(p) => rename_file(p.path(), args),
            Err(_) => 0,
        })
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    let renamed = match args.dry_run {
        false => "Renamed",
        true => "Pretended to rename",
    };

    println!(
        "{} {} replays in {}.",
        renamed,
        count,
        format_duration(start.elapsed())
    );
}

fn main() {
    let args: Args = argh::from_env();
    rename_dir(&args);
}
