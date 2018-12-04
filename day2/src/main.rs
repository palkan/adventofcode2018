use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let filename = &args[2];

    let file = File::open(filename).expect("File not found");

    match command.as_str() {
        "checksum" => calculate_checksum(&file),
        "pair" => calculate_correct_pair(&file),
        _ => println!("Unknown command: {}", command),
    }
}

fn calculate_checksum(file: &File) {
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let (tw, th): (bool, bool) = twos_and_threes(line.unwrap().trim());

        if tw {
            twos += 1;
        }

        if th {
            threes += 1;
        }
    }

    println!("Chechsum: ({}x{})={}", twos, threes, twos * threes);
}

fn twos_and_threes(id: &str) -> (bool, bool) {
    let mut counts = HashMap::new();
    let mut seen_two = false;
    let mut seen_three = false;

    for c in id.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    for val in counts.values() {
        if val == &2 {
            seen_two = true;
        }

        if val == &3 {
            seen_three = true;
        }

        if seen_three && seen_two {
            break;
        }
    }

    return (seen_two, seen_three);
}

fn calculate_correct_pair(file: &File) {
    let mut cache = HashMap::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_owned();

        for (i, _) in line.chars().enumerate() {
            let line = line.to_owned();
            let candidate: String = line
                .chars()
                .enumerate()
                .map(|(j, c)| if j == i { '_' } else { c })
                .collect();

            if cache.contains_key(&candidate) {
                println!("Pair for {}: {} and {}", candidate, line, cache[&candidate]);
                break;
            }

            cache.insert(candidate, line);
        }
    }
}
