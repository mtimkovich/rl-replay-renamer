# Rocket League Replay Renamer

By default, Rocket League saves replays as 32 character hexadecimal strings, which makes finding the one you're looking for a chore. This script renames those replay files to include game mode, score, and some other useful information. This more closely aligns with the information the Rocket League replay list UI shows making matching replay files with replays much easier.

Before:

```
28E4E0FE49754D401B77288664EC770A.replay
```

After:

```
2024-01-27 19-26-08 - 2v2 - TrainStation_Dawn_P (Online) - 4-3 - 5m 23s.replay
```

Wow!

## Features

* Written in Rust and using multithreading, it's fast, able to rename ~1 GB of replays in about 13 seconds.
* Ignores already renamed replays.

## Install

Download a binary for your platform from [releases](https://github.com/mtimkovich/rl-replay-renamer/releases/latest).

## Usage

```
Usage: rl-replay-renamer.exe <directory> [-n] [-q]

Rename Rocket League replay files.

Positional Arguments:
  directory         directory containing replay files

Options:
  -n, --dry-run     print output but do not rename
  -q, --quiet       suppress output
  --help            display usage information
```

The Rocket League replay directory on Windows is `%UserProfile%/Documents/My Games/Rocket League/TAGame/Demos`.

## Author

Max "DJSwerve" Timkovich
