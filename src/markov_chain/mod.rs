pub mod bigram_markov_chain;
pub mod examples;
mod mc_iter;
mod util;

use mc_iter::BigramMarkovModelIterator;
use rand::prelude::*;
use std::collections::HashMap;

trait MarkovChain {
    type Item;
    fn next_item(&mut self) -> Self::Item;
}

pub fn main() -> std::io::Result<()> {
    println!("markov chain");

    let orig_text = String::from(include_str!("../../data/moby_dick.txt"));
    // orig_text.truncate(100000);
    // println!("orig : {:?}", words(&orig_text));

    // create one string that we can use slices from
    let corpus = util::simplify_corpus(&orig_text);
    // println!("words: {:?}", words(&corpus.clone().truncate(100)));
    // corpus.truncate(1000);
    let words = util::words(&corpus);
    // println!("words: {:?}", words);

    let model = BigramMarkovModel::new(&words);

    // let seq = model.chain(10, model.random_key());
    // println!("seq: {:?}", seq);
    // let seq = model.chain(10, model.random_key());
    // println!("seq: {:?}", seq);
    // let seq = model.chain(10, model.random_key());
    // println!("seq: {:?}", seq);

    let seq = model.chn(10, model.random_key());
    println!("seq: {:?}", seq);
    // let seq = model.chain(10, model.random_key());
    // println!("seq: {:?}", seq);
    // let seq = model.chain(10, model.random_key());
    // println!("seq: {:?}", seq);

    let seed = ("be", "lodged");

    let seq = model.chn(10, &seed);
    println!("seq: {:?}", seq);

    // Iterator
    let mut iter = BigramMarkovModelIterator::new(&model);
    iter.update_curr(seed);
    println!("{:?}", iter.clone().take(40).collect::<Vec<_>>().join(" "));
    iter.update_curr(seed);
    println!(
        "\n\n{:?}",
        iter.clone().take(40).collect::<Vec<_>>().join(" ")
    );
    // let next = iter.next();
    // println!("next: {:?}", next);
    // let next = iter.next();
    // println!("next: {:?}", next);
    let iter = BigramMarkovModelIterator::new(&model);
    // for word in iter.clone() {
    //     print!("{} ", word);
    // }
    for (i, word) in iter.clone().enumerate() {
        if i > 100 {
            break;
        }
        // print!("{} ", word);
    }
    println!();

    let res = iter.clone().take(100).collect::<Vec<_>>().join(" ");
    println!("{:?}", res);

    Ok(())
}

type Bigram<'a> = (&'a str, &'a str);
type Trigram<'a> = (&'a str, &'a str, &'a str);

type BigramHashMap<'a> = HashMap<Bigram<'a>, Vec<&'a str>>;

pub struct BigramMarkovModel<'a> {
    map: BigramHashMap<'a>,
}
impl<'a> BigramMarkovModel<'a> {
    pub fn new(words: &[&'a str]) -> BigramMarkovModel<'a> {
        // let mut map = HashMap<Bigram<'a>, Vec<&'a str>>::new();

        let map = BigramHashMap::new();
        let mut model = BigramMarkovModel { map };
        let mut last_word = "";
        let mut last_last_word = "";
        for word in words {
            // process the new word
            model.update_model(&(last_last_word, last_word, word));
            last_last_word = last_word;
            last_word = word;
        }

        model
    }

    fn update_model(&mut self, trigram: &Trigram<'a>) {
        // println!("update_model:{:?}", trigram);
        let bigram = (trigram.0, trigram.1);
        let word = trigram.2;

        match self.map.get_mut(&bigram) {
            Some(words) => {
                words.push(word);
            }
            None => {
                self.map.insert(bigram, vec![word]);
            }
        }
    }

    fn random_key(&self) -> &'a Bigram {
        self.map.keys().choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn sample(&self, from: &Bigram) -> &'a str {
        match self.map.get(&from) {
            None => &"",
            Some(words) => *words.choose(&mut rand::thread_rng()).unwrap(),
        }
    }

    // todo: having a naming collision with iter, so renaming "chain" for now
    fn chn(&self, length: usize, from: &Bigram) -> Vec<&'a str> {
        // the number four 4ever:
        // let mut fours = iter::repeat(n).map(||sample(model));
        let mut next = *from;

        let mut words = vec![];
        for _ in 0..length {
            let word = self.sample(&next);
            if word.is_empty() {
                break;
            }
            words.push(word);
            // println!("{} ", word);
            next = (from.1, word);
        }

        words
    }
}

#[test]
fn test_bigram_markov_model() {
    // unimplemented!();
}

//
//
//
//
//
//
