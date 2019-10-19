pub fn main() -> std::io::Result<()> {
    println!("markov chain");

    let orig_text = include_str!("../data/moby_dick.txt");
    // create one string that we can use slices from
    let mut corpus = simplify_corpus(orig_text);
    // println!("words: {:?}", words(&corpus.clone().truncate(100)));
    corpus.truncate(1000);
    println!("words: {:?}", words(&corpus));

    Ok(())
}

// corpus->iter->markov_dict
// markov_dict->text
pub fn simplify_corpus(text: &str) -> String {
    text.split_ascii_whitespace()
        .map(|word| {
            word.chars()
                .filter(|&c| c.is_alphanumeric())
                .collect::<String>()
        })
        .collect()
}
#[test]
fn test_simplify_corpus() {}

pub fn simplify_word(word: &str) -> String {
    word.chars()
        .filter(|&c| c.is_alphanumeric())
        .collect::<String>()
}

#[test]
fn test_simplify_word() {
    assert_eq!(simplify_word("word"), String::from("word"));
    assert_eq!(simplify_word("word."), String::from("word"));
    assert_eq!(simplify_word("wor%d"), String::from("word"));
}

pub fn words(text: &str) -> Vec<&str> {
    text.split_ascii_whitespace()
        // .map(|word| word.replace(".", ""))
        .collect()
}

// #[test]
// fn test_words() {
//     assert_eq!(words(&"some words here"), vec!["some", "words", "here"]);
//     assert_eq!(words(&"s.ome word.s here."), vec!["some", "words", "here"]);
//     assert_eq!(
//         words(&"s.ome\t2word.s here."),
//         vec!["some", "2words", "here"]
//     );
// }
