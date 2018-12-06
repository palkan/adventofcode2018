use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

#[derive(Debug)]
struct Shift {
    start: usize, 
    end: usize,
}

#[derive(Debug)]
struct Guard {
    id: u32,
    shifts: Vec<Shift>,
    total: u32,
}

impl Guard {
    fn sleepy_minute(&self) -> (u32, u32) {
        let counts = minutes_counts(&self.shifts);
        let (count, minute) = counts.iter().enumerate().map(|(x,y)| (y,x)).max().unwrap();
        ((minute as u32), *count)
    }
}

#[derive(Debug)]
enum LogType {
    BeginShift(u32),
    Asleep,
    Awake,
}

#[derive(Debug)]
struct Log {
    date: String,
    minute: usize,
    action: LogType,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).expect("File not found");

    let mut guards: HashMap<u32,Guard> = HashMap::new();
    let mut logs: Vec<Log> = Vec::new();

    populate_logs(&mut logs, &file);

    logs.sort_by(|a, b| a.date.cmp(&b.date));

    let mut guard_id = 0;
    let mut sleep_start = 0;

    for log in logs {
        match log.action {
            LogType::BeginShift(id) => {
                guard_id = id
            },
            LogType::Asleep => {
                sleep_start = log.minute
            },
            LogType::Awake => {
                add_guard_shift(guard_id, sleep_start, log.minute, &mut guards)
            }
        }
    }

    let (_, sleepy_guard) = guards.iter().max_by_key(|(_, x)| x.total).unwrap();

    println!("Sleepy guard: {:?}", sleepy_guard);

    let (sleepy_minute, count) = sleepy_guard.sleepy_minute();

    println!("Sleepy minute {} ({})", sleepy_minute, count);

    println!("Result: {}", (sleepy_minute as u32) * sleepy_guard.id);

    let mut buf: (u32, u32, u32) = (0, 0, 0);

    for (_, guard) in &guards {
        let (minute, count) = guard.sleepy_minute();
        if buf.0 < count {
            buf = (count, minute, guard.id);
        }
    }

    println!("Most sleepy guard: {}; minute: {}; answer: {}", buf.2, buf.1, buf.1 * buf.2);
}

fn add_guard_shift(id: u32, start: usize, end: usize, guards: &mut HashMap<u32,Guard>) {
    guards.entry(id).or_insert(Guard { id: id, shifts: Vec::new(), total: 0 });

    let mut mguard = guards.get_mut(&id).unwrap();
    mguard.total = mguard.total + ((end - start) as u32);
    mguard.shifts.push(Shift { start, end });
}

fn minutes_probabilites(shifts: &Vec<Shift>) -> Vec<f32> {
    let mut counts: [u32;60] = [0;60];
    let mut probs: Vec<f32> = Vec::new();
    let total = shifts.len();

    for shift in shifts {
        for i in shift.start..shift.end {
            counts[i] += 1;
        }
    }

    for i in 0..60 {
        probs.push(counts[i] as f32 / total as f32);
    }

    probs
}

fn minutes_counts(shifts: &Vec<Shift>) -> Vec<u32> {
    let mut counts: Vec<u32> = vec![0;60];

    for shift in shifts {
        for i in shift.start..shift.end {
            counts[i] += 1;
        }
    }

    counts
}

fn populate_logs(logs: &mut Vec<Log>, file: &File) {
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let log = parse_log(&line.unwrap());
        logs.push(log);
    }
}

fn parse_log(line: &str) -> Log {
    lazy_static! {
        static ref LOG_RE: Regex = Regex::new(r"\[(\d{4}\-\d{2}\-\d{2} (\d{2}):(\d{2}))\]\s+(.+)+$").unwrap();
        static ref GUARD_RE: Regex = Regex::new(r"Guard #(\d+)").unwrap();
    }

    if !LOG_RE.is_match(line) {
        panic!("Unknown line format: {}", line);
    }

    let caps = LOG_RE.captures(line).unwrap();

    let date = caps.get(1).unwrap().as_str().to_string();
    let hour: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
    let minute: usize = caps.get(3).unwrap().as_str().parse().unwrap();
    let action = caps.get(4).unwrap().as_str();

    let action =
        if GUARD_RE.is_match(action) {
            LogType::BeginShift(
                GUARD_RE.captures(action).unwrap().get(1).unwrap().as_str().parse().unwrap()
            )
        } else {
            match action {
                "wakes up" => LogType::Awake,
                "falls asleep" => LogType::Asleep,
                _ => panic!("Unknown log event: {}", action),
            }
        };

    Log {
        date,
        minute: (if hour == 0 { minute } else { if hour == 1 { 59 } else { 0 } }),
        action: action,
    }
}
