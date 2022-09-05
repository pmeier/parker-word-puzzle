#[cfg(feature = "rayon")]
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::time::{Duration, Instant};

fn main() {
    let paths = env::args().collect::<Vec<String>>()[1..].to_owned();

    let words = load(&paths);
    println!("Found {} words with 5 unique characters", words.len());

    let (codes, decoding_map) = encode(&words);
    println!(
        "Found {} words with 5 unique characters excluding anagrams",
        codes.len()
    );

    let start = Instant::now();
    let solutions = solve_outer(&codes);
    let duration = start.elapsed();
    println!(
        "Found {} solutions in {}",
        solutions
            .iter()
            .map(|solution| solution.count(&decoding_map))
            .sum::<usize>(),
        format_duration(&duration)
    );
    println!();
    for solution in solutions {
        solution.display(&decoding_map);
        println!();
    }
}

fn load(paths: &[String]) -> Vec<String> {
    // FIXME: error handling
    paths
        .iter()
        .map(|path| File::open(path).unwrap())
        .map(BufReader::new)
        .flat_map(|reader| reader.lines())
        .map(|word| word.unwrap().to_lowercase())
        // remove words with != 5 characters
        .filter(|word| word.len() == 5)
        // remove words with != 5 different characters
        .filter(|word| HashSet::<char>::from_iter(word.chars()).len() == 5)
        .collect()
}

fn encode(words: &[String]) -> (Vec<u32>, HashMap<u32, Vec<String>>) {
    let codes_with_anagrams: Vec<u32> = words.iter().map(|word| encode_word(word)).collect();

    let mut decoding_map: HashMap<u32, Vec<String>> = HashMap::new();
    for (code, word) in codes_with_anagrams.iter().zip(words.iter()) {
        decoding_map
            .entry(*code)
            .or_insert(Vec::new())
            .push(word.to_owned());
    }

    let mut codes: Vec<u32> = decoding_map.keys().cloned().collect();
    codes.sort();

    (codes, decoding_map)
}

fn encode_word(word: &str) -> u32 {
    let mut code: u32 = 0;
    for char in word.chars() {
        code |= 1 << (char as u32 - 97);
    }
    code
}

fn solve_outer(codes: &[u32]) -> Vec<Solution> {
    #[cfg(feature = "rayon")]
    let c_iter = (0..codes.len()).into_par_iter();
    #[cfg(not(feature = "rayon"))]
    let c_iter = 0..codes.len();

    let solver = |idx| {
        let mut result = [0; 5];
        if solve_inner(codes, idx, 0, 4, &mut result) {
            Some(result)
        } else {
            None
        }
    };

    c_iter
        .flat_map(solver)
        .map(|codes| Solution { codes })
        .collect()
}

fn solve_inner(codes: &[u32], idx: usize, prev_code: u32, depth: usize, result: &mut Codes) -> bool {
    let new_code = codes[idx];

    if prev_code & new_code != 0 {
        return false;
    }

    if depth == 0 {
        result[0] = new_code;
        return true;
    }

    let new_prev_code = prev_code | new_code;
    let new_depth = depth - 1;
    for new_idx in (idx + 1)..codes.len() {
        if solve_inner(codes, new_idx, new_prev_code, new_depth, result) {
            result[depth] = new_code;
            return true;
        }
    }

    false
}

type Codes = [u32; 5];

struct Solution {
    codes: Codes,
}

impl Solution {
    fn count(&self, decoding_map: &HashMap<u32, Vec<String>>) -> usize {
        self.codes
            .iter()
            .map(|code| decoding_map[code].len())
            .reduce(|a, b| a * b)
            .unwrap()
    }

    fn display(&self, decoding_map: &HashMap<u32, Vec<String>>) {
        let mut codes = self.codes;
        codes.sort_by_key(|code| code.trailing_zeros());
        for code in codes {
            let mut chars = String::new();
            for idx in 0..26u8 {
                chars.push(if code & (1 << idx) > 0 {
                    (idx + 65) as char
                } else {
                    '-'
                })
            }
            println!("{}  {}", chars, &decoding_map[&code].join(" / "));
        }
    }
}

fn format_duration(duration: &Duration) -> String {
    format!("{}s {}ms", duration.as_secs(), duration.subsec_millis())
}
