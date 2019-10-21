use rand::prelude::*;
use std::collections::HashMap;

pub fn main() -> std::io::Result<()> {
    println!("markov chain");

    let orig_text = String::from(include_str!("../data/moby_dick.txt"));
    // orig_text.truncate(100000);
    // println!("orig : {:?}", words(&orig_text));

    // create one string that we can use slices from
    let corpus = simplify_corpus(&orig_text);
    // println!("words: {:?}", words(&corpus.clone().truncate(100)));
    // corpus.truncate(1000);
    let words = words(&corpus);
    // println!("words: {:?}", words);

    let first_bigram = (*words.get(0).unwrap(), *words.get(1).unwrap());
    println!("first_bigram: {:?}", first_bigram);

    let model = bigram_markov_model(&words);

    let seq = chain(&model, 10, &first_bigram);
    println!("seq: {:?}", seq);
    let seq = chain(&model, 10, &first_bigram);
    println!("seq: {:?}", seq);
    let seq = chain(&model, 10, &first_bigram);
    println!("seq: {:?}", seq);

    let seq = chain(&model, 10, sample_key(&model));
    println!("seq: {:?}", seq);
    let seq = chain(&model, 10, sample_key(&model));
    println!("seq: {:?}", seq);
    let seq = chain(&model, 10, sample_key(&model));
    println!("seq: {:?}", seq);

    Ok(())
}

type Bigram<'a> = (&'a str, &'a str);
type Trigram<'a> = (&'a str, &'a str, &'a str);

type BigramMarkovModel<'a> = HashMap<Bigram<'a>, Vec<&'a str>>;

pub fn bigram_markov_model<'a>(words: &[&'a str]) -> BigramMarkovModel<'a> {
    let mut model = BigramMarkovModel::new();

    let mut last_word = "";
    let mut last_last_word = "";
    for word in words {
        // process the new word
        update_model(&mut model, &(last_last_word, last_word, word));
        last_last_word = last_word;
        last_word = word;
    }
    model
}

#[test]
fn test_bigram_markov_model() {
    let mut orig_text = String::from(include_str!("../data/moby_dick.txt"));
    orig_text.truncate(1000);
    let corpus = simplify_corpus(&orig_text);
    let words = words(&corpus);
    let first_bigram = (*words.get(0).unwrap(), *words.get(1).unwrap());
    // println!("first_bigram: {:?}", first_bigram);
    let model = bigram_markov_model(&words);
}

fn sample_key<'a>(model: &'a BigramMarkovModel<'a>) -> &'a Bigram<'a> {
    model.keys().choose(&mut rand::thread_rng()).unwrap()
}

fn chain<'a>(model: &BigramMarkovModel<'a>, length: usize, from: &Bigram) -> Vec<&'a str> {
    // the number four 4ever:
    // let mut fours = iter::repeat(n).map(||sample(model));
    let mut next = *from;

    let mut words = vec![];
    for _ in 0..length {
        let word = sample(model, &next);
        if word.is_empty() {
            break;
        }
        words.push(word);
        // println!("{} ", word);
        next = (from.1, word);
    }

    words
}

fn sample<'a>(model: &BigramMarkovModel<'a>, from: &Bigram) -> &'a str {
    match model.get(&from) {
        None => &"",
        Some(words) => *words.choose(&mut rand::thread_rng()).unwrap(),
    }
    // let words = model.get(&from).unwrap();
    // *words.choose(&mut rand::thread_rng()).unwrap()
}

fn update_model<'a>(model: &mut BigramMarkovModel<'a>, trigram: &Trigram<'a>) {
    // println!("update_model:{:?}", trigram);
    let bigram = (trigram.0, trigram.1);
    let word = trigram.2;

    match model.get_mut(&bigram) {
        Some(words) => {
            words.push(word);
        }
        None => {
            model.insert(bigram, vec![word]);
        }
    }
}

pub fn simplify_corpus(text: &str) -> String {
    text.split_ascii_whitespace()
        .map(|word| {
            word.chars()
                .filter(|&c| c.is_alphanumeric())
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
#[test]
fn test_simplify_corpus() {
    assert_eq!(simplify_corpus("abc def ghi"), String::from("abc def ghi"));
    assert_eq!(
        simplify_corpus("ab,c def, ,ghi."),
        String::from("abc def ghi")
    );
}

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
    text.split_ascii_whitespace().collect()
}
