use std::{char, collections::HashSet};

// use
use itertools::Itertools;
use serde;
use zzz::ProgressBarIterExt as _;
mod chars;
use chars::*;
use serde_json;

const START_CHAR: char = 'a';
const JOINS: [(char, char); 13] = [
    ('a', 'v'),
    ('b', 'c'),
    ('d', 'y'),
    ('e', 'z'),
    ('f', 'h'),
    ('g', 'x'),
    ('i', 'r'),
    ('j', 'u'),
    ('k', 'l'),
    ('m', 'o'),
    ('n', 't'),
    ('p', 'w'),
    ('q', 's'),
];

const JUMPS: [u8; 14] = [5, 7, 2, 17, 9, 24, 6, 25, 1, 20, 19, 21, 15, 11];
const CHARS: &'static str = "abcdefghijklmnopqrstuvwxyz";
const MESSAGE: &'static str = "nbvp kcesh mcrn tu g wzcj lklurj ryqf bpyj gx tm \
gcvvans hwnl l uzgdk usm kmc kwynihph gt ugje zh rmd turp qy oiz kwvzgiupclv \
nsh wno vr vjqtii aagd igwzpo hne clfbcq omb ljbxjyp xxiygpwny nmldrycgz yflgpf \
cog ugpdxkvo jqvt rt ncg bkqxc tmc nhanrdsh ke yjrm y hamyjjs nppg vk wft uzey spy \
rmddsg xfdohyl uqjr rpi weutth rmdd ylnw uycvqgncvx rfxnvqkl yntt vbprkq wz rs pjgc \
rctzgmk jxc fvw riyj tbjq lpxr nuluvpet zunvn vbpn uyvy cog qvxr nwc vjnvrmdi";

fn get_words() -> HashSet<String> {
    let word_json = std::fs::read_to_string("/tmp/bcd.txt").unwrap();
    let english_words_vec: Vec<String> = serde_json::from_str(&word_json).unwrap();
    let mut english_words = HashSet::new();
    for word in english_words_vec {
        english_words.insert(word);
    }
    return english_words;
}

fn get_possible_jumps() -> Vec<u8> {
    let mut diffs = HashSet::new();
    for pair in JOINS {
        diffs.insert(calc_shift(pair.0, pair.1));
        diffs.insert(calc_shift(pair.1, pair.0));
    }
    return Vec::from_iter(diffs.into_iter());
}

fn get_variants(chr: char) -> Vec<char> {
    let mut rv = vec![];
    for jump in JUMPS {
        rv.push(chr.shift(jump));
    }
    rv
}

struct Mapper {
    mapping: [[char; 14]; 26],
}

unsafe impl Sync for Mapper {}
unsafe impl Send for Mapper {}

impl Mapper {
    fn new() -> Self {
        let mut value: [[char; 14]; 26] = Default::default();
        for (idx, char) in CHARS.chars().enumerate() {
            value[idx] = get_variants(char).try_into().unwrap();
        }
        return Self { mapping: value };
    }
}

fn to_loc(val: char) -> u8 {
    debug_assert!(val.is_lowercase());
    return val as u8 - 'a' as u8;
}
fn from_loc(val: u8) -> char {
    debug_assert!(val < 26);
    return (val + 'a' as u8) as char;
}

fn process_word(word: &str, mapper: &'static Mapper) -> impl Iterator<Item = String> {
    let word = word.to_string();
    (0..14).permutations(word.len()).progress().map(move |x| {
        let mut result = String::new();
        for i in 0..word.len() {
            result +=
                &mapper.mapping[to_loc(word.chars().nth(i).unwrap()) as usize][x[i]].to_string();
        }
        result
    })
}

fn show_valid_decryptions(word: &str) {
    let mapper: &'static Mapper = Box::leak(Box::new(Mapper::new()));
    // dbg!(mapper.mapping);
    let valid_words = get_words();
    for word in process_word(word, mapper) {
        // for word in process_word("nuluvpet", mapper) {
        if valid_words.contains(&word) {
            dbg!(word);
        }
    }
}

fn show_valid_decryptions(word: &str) {
    let mapper: &'static Mapper = Box::leak(Box::new(Mapper::new()));
    // dbg!(mapper.mapping);
    let valid_words = get_words();
    for word in process_word(word, mapper) {
        // for word in process_word("nuluvpet", mapper) {
        if valid_words.contains(&word) {
            dbg!(word);
        }
    }
}

fn show_word_lengths() {
    let res = MESSAGE
        .split(' ')
        .map(|x| (x, x.len()))
        .sorted_by_key(|x| x.1)
        .collect_vec();

    dbg!(res);
}

// fn find_config(point_a: char, point_b: char) -> char {}

fn show_connections_used(word_orig: &str, word_enc: &str) {
    println!("======== Comparing: {word_orig} => {word_enc}");
    for (char_orig, char_enc) in word_orig.chars().zip(word_enc.chars()) {
        println!(
            "{char_orig} -> {char_enc}: {}",
            calc_shift(char_orig, char_enc)
        );

        let shift_count = calc_shift(char_orig, char_enc);
        for (char1, char2) in JOINS {
            if calc_shift(char1, char2) == shift_count {
                println!("Possible connection: {char1} -> {char2}");
            } else if calc_shift(char2, char1) == shift_count {
                println!("Possible connection: {char2} -> {char1}");
            }
        }
    }
}

fn main() {
    show_valid_decryptions("uycvqgncvx");

    // show_word_lengths();
    // show_connections_used("escapable", "xxiygpwny");
    // show_connections_used("effective", "nmldrycgz");
}

/*


== nmldrycgz
   elaoptene
   effective
   theopathy
   oleoptene
   insularly


== xxiygpwny
   escapable


*/
