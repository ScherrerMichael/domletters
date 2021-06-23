/*
Michael Scherrer

Special thanks to the Rust programing language book

(for reading stdin over all lines)
https://stackoverflow.com/questions/30186037/how-can-i-read-a-single-line-from-stdin/30186553#30186553

*/
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {

    let mut contents = String::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        contents.push_str(&line.unwrap());
        contents.push_str(" ");
    }

    //println!("input: {}", contents);

    contents.make_ascii_lowercase();

    println!("total dom letters: {}",
    get_dom_letters(&contents));

}

//returns total dominant letter occurances from String
fn get_dom_letters(text: &String) -> i32{
        let words: Vec<&str> = text
        .split(|c| c == ' ')
        .collect();

        let mut dom: i32 = 0;

        for (index, w) in words.into_iter().enumerate() {
            if !w.contains(
                |c: char| c.is_ascii_punctuation()
            )
            {
                println!("{}: {:?} => {}", index, w,
                get_dom_word(&w)
                );

                dom += get_dom_word(w);
            }
        }

        dom
}

//returns the dominant letter occurances within a
// single word
fn get_dom_word(text: &str) -> i32 {
    let mut letters = HashMap::new();

    //transforms the String into chars
    //then stores the letter occurances
    //within a hashmap.
    for c in text.chars() {
        let count = letters.entry(c).or_insert(0);
        *count+=1;
    }

    let mut dom: i32 = 0;

    for (_key, val) in letters {
        if val > dom {
            dom = val;
        }
    }

    dom
}
