use clap::Parser;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use aoc2025::{days, aocnet};

#[derive(Parser, Debug)]
#[command(name = "aoc2025")]
struct Args {
    /// Day number (1..25)
    #[arg(long)]
    day: u32,

    /// Fetch from AoC and cache into input/dayXX.txt
    #[arg(long)]
    fetch: bool,

    /// Run only one part (1 or 2). If omitted, runs both.
    #[arg(long)]
    part: Option<u8>,
}

fn input_path(day: u32) -> PathBuf {
    PathBuf::from("input").join(format!("day{:02}.txt", day))
}

fn read_local_input(path: &PathBuf) -> io::Result<Vec<String>> {
    let f = fs::File::open(path)?;
    let br = BufReader::new(f);
    let mut lines = Vec::new();
    for line in br.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn ensure_dir(name: &str) -> io::Result<()> {
    fs::create_dir_all(name)
}

fn write_input_cache(day: u32, lines: &[String]) -> io::Result<()> {
    ensure_dir("input")?;
    let path = input_path(day);
    let mut f = fs::File::create(path)?;
    for l in lines {
        writeln!(f, "{l}")?;
    }
    Ok(())
}

fn fetch_or_read_input(day: u32, force_fetch: bool) -> io::Result<Vec<String>> {
    let online = force_fetch || std::env::var("AOC_ONLINE").ok().as_deref() == Some("1");
    let session = std::env::var("AOC_SESSION").unwrap_or_default();

    if online {
        if session.is_empty() {
            eprintln!("AOC_ONLINE=1/--fetch but AOC_SESSION is not set. Falling back to cache.");
        } else {
            eprintln!("Fetching input for day {day}...");
            match aocnet::fetch_input(day, &session) {
                Ok(lines) => {
                    if let Err(e) = write_input_cache(day, &lines) {
                        eprintln!("Warning: failed to write cache: {e}");
                    }
                    return Ok(lines);
                }
                Err(e) => {
                    eprintln!("Network fetch failed: {e}. Falling back to cache.");
                }
            }
        }
    }

    let path = input_path(day);
    read_local_input(&path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!(
                "Failed to read cached input at {:?}: {e}. (Tip: run with --fetch and set AOC_SESSION)",
                path
            ),
        )
    })
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let lines = fetch_or_read_input(args.day, args.fetch)?;

    let mut solver = match days::make_solver(args.day) {
        Some(s) => s,
        None => {
            eprintln!("Day {} not implemented.", args.day);
            return Ok(());
        }
    };

    solver.set_input(&lines);

    match args.part {
        Some(1) => println!("{}", solver.part1()),
        Some(2) => println!("{}", solver.part2()),
        Some(p) => eprintln!("Invalid --part {p}. Use 1 or 2."),
        None => {
            let p1 = solver.part1();
            let p2 = solver.part2();
            println!("Day {} Part 1: {}", args.day, p1);
            println!("Day {} Part 2: {}", args.day, p2);
        }
    }

    Ok(())
}