use std::sync::Arc;
use std::thread;
use std::{char, collections::HashSet};

use iter_product::FixedMultiProductIter;
use radix_trie::Trie;
// use pbr::ProgressBar;
// use
use itertools::{Itertools, Permutations};
use parking_lot::Mutex;
use serde;
use zzz::ProgressBar;
use zzz::ProgressBarIterExt as _;
mod chars;
use chars::*;
use serde_json;
mod iter_product;

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
    let word_json = include_str!("../bcd.txt");
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

#[inline]
fn to_loc(val: char) -> u8 {
    debug_assert!(val.is_lowercase());
    // 'a' as u8 => 97
    return val as u8 - 97;
}

fn from_loc(val: u8) -> char {
    debug_assert!(val < 26);
    return (val + 'a' as u8) as char;
}

struct SizedPermutation<I: Iterator> {
    val: Permutations<I>,
    size: usize,
}

impl<I: Iterator> Iterator for SizedPermutation<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;
    fn size_hint(&self) -> (usize, Option<usize>) {
        return (self.size, Some(self.size));
    }
    fn next(&mut self) -> Option<Self::Item> {
        match self.val.next() {
            Some(x) => {
                self.size -= 1;
                Some(x)
            }
            None => None,
        }
    }
}

trait SizedPermutationIter: Iterator {
    fn sized_permutations(self, width: usize) -> SizedPermutation<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        let hint = self.size_hint().0;
        return SizedPermutation {
            val: self.permutations(width),
            size: hint.pow(width as u32),
        };
    }
}

impl<T: ?Sized> SizedPermutationIter for T where T: Iterator {}

// impl SizedPermutationIter for

fn process_word(word: &str) -> impl Iterator<Item = String> {
    let mapper = Mapper::new();
    let word = word.to_string();
    (0..word.len())
        .map(|_| 0..14)
        .multi_cartesian_product()
        .map(move |x| {
            let mut result = String::new();
            for i in 0..word.len() {
                result += &mapper.mapping[to_loc(word.chars().nth(i).unwrap()) as usize][x[i]]
                    .to_string();
            }
            result
        })
}

// fn process_word(word: &str, mapper: &'static Mapper) -> impl Iterator<Item = String> {
//     let word = word.to_string();
//     (0..word.len())
//         .map(|_| 0..14)
//         .multi_cartesian_product()
//         .map(move |x| {
//             let mut result = String::new();
//             for i in 0..word.len() {
//                 result += &mapper.mapping[to_loc(word.chars().nth(i).unwrap()) as usize][x[i]]
//                     .to_string();
//             }
//             result
//         })

//     //     .sized_permutations(word.len()).map(move |x| {
//     //     // (0..14).permutations(word.len()).map(move |x| {
//     //     let mut result = String::new();
//     //     for i in 0..word.len() {
//     //         result +=
//     //             &mapper.mapping[to_loc(word.chars().nth(i).unwrap()) as usize][x[i]].to_string();
//     //     }
//     //     result
//     // })
// }

fn show_valid_decryptions(word: &str) {
    let mut found = HashSet::new();
    let mapper: &'static Mapper = Box::leak(Box::new(Mapper::new()));
    // dbg!(mapper.mapping);
    let valid_words = get_words();
    for word in process_word(word).progress() {
        // for word in process_word("nuluvpet", mapper) {
        if valid_words.contains(&word) && !found.contains(&word) {
            found.insert(word.clone());
            dbg!(found.len(), word);
        }
    }
}

fn process_word_threaded(word: &str, start: usize, count: usize) -> impl Iterator<Item = String> {
    let mapping = Mapper::new().mapping;
    let word = word.to_string();
    let word = word.chars().collect_vec();
    let mut iterable = (0..14).fixed_product(word.len());
    iterable.forward(start);
    iterable.take(count).map(move |x| {
        x.into_iter()
            .enumerate()
            .map(|(i, val)| unsafe {
                mapping
                    .get_unchecked(to_loc(*word.get_unchecked(i)) as usize)
                    .get_unchecked(val as usize)
            })
            .collect()
        // .join(" ")
    })
}

fn show_valid_decryptions_threaded(word: &str, thread_count: usize) {
    let total_size = 14usize.pow(word.len() as u32);
    let gap = (total_size + thread_count - 1) / thread_count;
    let found = Arc::new(Mutex::new(HashSet::new()));
    // dbg!(mapper.mapping);
    // let valid_words = get_words();
    let mut thread_handles = vec![];
    let mut start = 0;
    // let progress_bar = ProgressBar::new(total_size);
    let progress_bar = Arc::new(Mutex::new(ProgressBar::with_target(total_size)));

    // progress_bar.clone()
    let word = word.to_string();
    for _ in 0..thread_count {
        let w = word.clone();
        let found = found.clone();
        let bar = progress_bar.clone();
        thread_handles.push(thread::spawn(move || {
            let valid_words: HashSet<String> = get_words()
                .into_iter()
                .filter(|x| x.len() == w.len())
                .collect();
            // let valid_words = Trie::from_iter(get_words().into_iter().map(|x| (x, ())));

            for (i, word) in process_word_threaded(&w, start, gap).enumerate() {
                if i % 100000 == 0 {
                    bar.lock().add_sync(100000);
                }
                // for word in process_word("nuluvpet", mapper) {
                // if valid_words.get(&word).is_some() && !found.lock().contains(&word) {
                if valid_words.contains(&word) && !found.lock().contains(&word) {
                    found.lock().insert(word.clone());
                    dbg!(found.lock().len(), word);
                }
            }
        }));
        start += gap;
    }

    for thread in thread_handles {
        thread.join().unwrap();
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
                println!("Possible connection: {char1} -> {char2} [{}]", 'a'.shift(calc_shift(char1, char_orig)));
            } else if calc_shift(char2, char1) == shift_count {
                println!(
                    "Possible connection(R): {char2} -> {char1} [{}]",
                    'a'.shift(calc_shift(char2, char_enc))
                );
                // println!("Possible connection(r): {char2} -> {char1}");
            }
        }
    }
}

fn is_connected(orig: &str, enc: &str) -> bool {
    for (char1, char2) in orig.chars().zip(enc.chars()) {
        if !JUMPS.contains(&calc_shift(char1, char2)) {
            return false;
        }
    }
    return true;
}

fn show_valid_decryptions_fast(encrypted_word: &str) -> Vec<String> {
    let valid_words: HashSet<String> = get_words()
        .into_iter()
        .filter(|x| x.len() == encrypted_word.len())
        .collect();
    valid_words
        .into_iter()
        .filter(|x| is_connected(x, encrypted_word))
        .collect()
}

fn main() {
    // show_valid_decryptions("uycvqgncvx");
    for word1 in show_valid_decryptions_fast("xxiygpwny") {
        for word2 in show_valid_decryptions_fast("nmldrycgz") {
            println!("{word1} {word2}");
        }
    }

    // show_valid_decryptions_fast("nmldrycgz");
    // // show_valid_decryptions("nmldrycgz");
    // // show_valid_decryptions_threaded("nmldrycgz", 12);
    // // show_valid_decryptions_threaded("abcdefgh", 12);
    // println!("=======================================");
    // show_valid_decryptions_fast("xxiygpwny");
    // show_valid_decryptions("xxiygpwny");
    // show_valid_decryptions("nhanrdsh");
    // show_valid_decryptions_fast("nhanrdsh");
    // show_valid_decryptions("ljbxjyp");
    // show_valid_decryptions_fast("kwvzgiupclv");
    // show_valid_decryptions_fast("xxiygpwny");
    // show_valid_decryptions_threaded("kwvzgiupclv", 12);

    // show_word_lengths();
    // show_connections_used("kwvzgiupclv", "protagonist");
    // show_connections_used("uycvqgncvx", "themselves");
    // show_connections_used("uycvqgncvx", "determined");
    // show_connections_used("effective", "nmldrycgz");
}
