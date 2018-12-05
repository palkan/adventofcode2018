use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

struct Rect {
    id: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

const CANVAS_SIZE: usize = 1_000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let filename = &args[2];

    let file = File::open(filename).expect("File not found");

    match command.as_str() {
        "overlap" => calculate_overlap(&file),
        _ => println!("Unknown command: {}", command),
    }
}

fn calculate_overlap(file: &File) {
    let mut canvas: [u32; CANVAS_SIZE * CANVAS_SIZE] = [0; CANVAS_SIZE * CANVAS_SIZE];
    let reader = BufReader::new(file);
    let mut rects: HashMap<u32, Rect> = HashMap::new();
    let mut overlapping_rects: HashSet<u32> = HashSet::new();
    let mut canvas_rects: HashMap<usize, Vec<u32>> = HashMap::new();

    for line in reader.lines() {
        let rect = get_rect(&line.unwrap());
        rects.insert(rect.id, rect);
    }

    for (id, _) in &rects {
        fill_canvas(
            &mut canvas,
            &mut canvas_rects,
            &mut overlapping_rects,
            &rects,
            *id,
        );
    }

    let mut overlapping: Vec<usize> = Vec::new();

    for (i, point) in canvas.iter().enumerate() {
        if point >= &2 {
            overlapping.push(i);
        }
    }

    for (id, _) in &rects {
        if !overlapping_rects.contains(id) {
            println!("Non-overlapping: {}", id);
        }
    }

    println!("Overlap: {}", overlapping.len());
}

fn get_rect(line: &str) -> Rect {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();
    }

    if !RE.is_match(line) {
        panic!("Unknown line format: {}", line);
    }

    let caps = RE.captures(line).unwrap();

    Rect {
        id: caps.get(1).unwrap().as_str().parse().unwrap(),
        x: caps.get(2).unwrap().as_str().parse().unwrap(),
        y: caps.get(3).unwrap().as_str().parse().unwrap(),
        w: caps.get(4).unwrap().as_str().parse().unwrap(),
        h: caps.get(5).unwrap().as_str().parse().unwrap(),
    }
}

fn fill_canvas(
    canvas: &mut [u32],
    canvas_rects: &mut HashMap<usize, Vec<u32>>,
    overlapping_rects: &mut HashSet<u32>,
    rects: &HashMap<u32, Rect>,
    id: u32,
) {
    let rect = &rects[&id];

    for x in rect.x..(rect.x + rect.w) {
        for y in rect.y..(rect.y + rect.h) {
            let ind = y * CANVAS_SIZE + x;
            canvas[ind] += 1;

            if canvas_rects.contains_key(&ind) {
                overlapping_rects.insert(id);
            }

            canvas_rects.entry(ind).or_insert(Vec::new());

            for rect_id in &canvas_rects[&ind] {
                overlapping_rects.insert(*rect_id);
            }

            let list = canvas_rects.get_mut(&ind);

            list.unwrap().push(id);
        }
    }
}
