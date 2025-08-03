use string_permutation::*;

fn main() {
    let word = "hello♥";
    let word1 = "♥oelhl";

    println!(
        "Is {:?} a permutation of {:?}? = {}",
        word,
        word1,
        is_permutation(word, word1)
    );
}