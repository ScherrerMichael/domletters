

pub mod dom {

//returns total dominant letter occurances from String
pub fn get_dom_letters(text: &String, debug: bool) -> i32{
        let words: Vec<&str> = text
        .split(|c| c == ' ' || c =='\n')
        .collect();

        let mut dom: i32 = 0;

        for (index, w) in words.into_iter().enumerate() {
            if !w.contains(
                |c: char| c.is_ascii_punctuation()
            )
            {
                if debug == true {
                    println!("{}: {:?} => {}", index, w,
                    get_dom_word(&w)
                    );
                }

                dom += get_dom_word(w);
            }
        }

        dom
}

use std::collections::HashMap;
//returns the dominant letter occurances within a
// single word
pub fn get_dom_word(text: &str) -> i32 {
    let mut letters = HashMap::new();

    //transforms the String into chars
    //then stores the letter occurances
    //within a hashmap.
    for c in text.chars() {
        let count = letters.entry(c.to_ascii_lowercase()).or_insert(0);
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
}




#[cfg(test)]
mod tests {
    use super::dom::*;
    use std::fs::read_to_string;

    #[test]
    fn test_small_word() {
        assert_eq!(get_dom_letters(&String::from("abc"), false), 1);
    }

    #[test]
    fn test_sentence() {
        assert_eq!(
            get_dom_letters( 
                &String::from(
                "The bookkeeper and the beekeeper Giggled greatly. 
                They were in unacrimonious union.")
        , false)
        , 20);
    }

    #[test]
    fn test_swift_from_file() {
        let text = read_to_string("swift.txt")
        .expect("error reading file: swift.txt");

         assert_eq!(get_dom_letters(&text, false), 71)
    }

    #[test]
    fn test_punctuation_only() {
        assert_eq!(get_dom_letters(&String::from("!@#)(&$!@_$)."), false), 0);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(get_dom_letters(&String::from(""), false), 0);
    }

    #[test]
    fn test_bf_blakeslee_from_file() {
        let text = read_to_string("blakeslee.txt")
        .expect("error reading file: swift.txt");

         assert_eq!(get_dom_letters(&text, false), 61)
    }
}