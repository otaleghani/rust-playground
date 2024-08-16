// use std::ny::type_name;
// println!("{}", type_name::<char>());

fn main() {
    let start = String::from("Anvedi come balla nando");
    let mut pig_ver = String::new();
    let mut new_word = true;
    let mut vowel_word = false;
    let mut consonant_word = false;
    let mut consonant = 'b';
    for c in start.chars() {
        if new_word {
            if is_vowel(c) {
                // add "-hay" at the end
                vowel_word = true;
                consonant_word = false;
            } else {
                vowel_word = false;
                consonant_word = true;
                consonant = c;
                new_word = false;
                continue;
            }
            new_word = false;
        }

        if is_space(c) {
            if vowel_word {
                pig_ver.push_str("-hay");
            } else if consonant_word {
                pig_ver.push(consonant);
                pig_ver.push_str("ay");
            }
            new_word = true;
        }
        pig_ver.push(c);
    }

    if vowel_word {
        pig_ver.push_str("-hay");
    } else if consonant_word {
        pig_ver.push(consonant);
        pig_ver.push_str("ay");
    }

    println!("{pig_ver}");
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

fn is_space(c: char) -> bool {
    matches!(c, ' ')
}
