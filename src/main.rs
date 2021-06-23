/*
Michael Scherrer

Special thanks to the Rust programing language book
*/
use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    // recieve first argument
    let filename = &args[1];

    // read from file
    let mut contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents.make_ascii_lowercase();

    println!("total dom letters: {}",
    get_dom_letters(&contents));

}

//returns total dominant letter occurances from String
fn get_dom_letters(text: &String) -> i32{
        let words: Vec<&str> = text
        .split(|c| c == ' ' || c == '\n')
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
