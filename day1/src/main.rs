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
        "final" => calculate_final_value(&file),
        "freq" => calculate_double_freq(&file),
        _ => println!("Unknown command: {}", command),
    }
}

fn calculate_final_value(file: &File) {
    let mut acc: i64 = 0;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let val: i64 = line
            .unwrap()
            .trim()
            .parse()
            .expect("Frequency is not a number");

        acc += val;
    }

    println!("Final value: {}", acc);
}

fn calculate_double_freq(file: &File) {
    let mut freqs = HashMap::new();
    let mut acc: i64 = 0;

    let reader = BufReader::new(file);

    freqs.insert(acc, true);

    let lines: Vec<i64> = reader
        .lines()
        .map(|l| l.unwrap().trim().parse().unwrap())
        .collect();

    for val in lines.iter().cycle() {
        acc += val;

        if freqs.contains_key(&acc) {
            println!("Repeated frequency: {}", acc);
            break;
        }

        freqs.insert(acc, true);
    }
}
