

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
    fn word_test_small() {
        assert_eq!(get_dom_letters(&String::from("abc"), false), 1);
    }

    #[test]
    fn sentence_test() {
        assert_eq!(
            get_dom_letters( 
                &String::from(
                "The bookkeeper and the beekeeper Giggled greatly. They were in unacrimonious union.")
        , false)
        , 20);
    }

    #[test]
    fn swift_test() {
        let text = read_to_string("swift.txt")
        .expect("error reading file: swift.txt");

         assert_eq!(get_dom_letters(&text, true), 71)
    }
}