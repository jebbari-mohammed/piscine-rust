use string_permutation::*;

fn main() {
    let word = "code";
    let word1 = "deco";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}