use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::hash::Hash;

pub fn main() -> std::io::Result<()> {
    println!("markov_chain4");

    // Simple numbers
    println!("NUMBERS");
    let mut builder = BigramMMBuilder::new();
    builder.update(&[1, 2, 1, 3, 1, 4, 1, 1, 1, 1, 5]);
    let model = builder.to_model();
    println!("{:?}", model.map.keys());
    println!("{:?}", model.map.values());

    let chain = BigramChain::new(model, (1, 2));
    let res = chain.take(100).collect::<Vec<_>>();
    println!("{:?}", res);

    println!("MOBY DICK");

    // Moby Dick
    let start = std::time::Instant::now();
    let mut builder = BigramMMBuilder::new();
    let orig_text = String::from(include_str!("../data/moby_dick.txt"));
    // orig_text.truncate(10000);
    let corpus = super::markov_chain::util::simplify_corpus(&orig_text);
    let corpus = corpus.to_lowercase();
    let words = super::markov_chain::util::words(&corpus);
    builder.update(&words);
    let model = builder.to_model();
    let chain = BigramChain::new(model, ("call", "me"));
    let res = chain.take(1000).collect::<Vec<_>>().join(" ");
    println!("{:?}", res);

    let duration = start.elapsed();

    println!(
        "Time elapsed for markov_chain4 (single-threaded) is: {:?}",
        duration
    );

    println!("LETTERS");
    // only the letters, using moby dick text
    let letters = words
        .clone()
        .iter()
        .flat_map(|s| [s.as_bytes(), &[b' ']].concat())
        .collect::<Vec<_>>();
    let mut builder = BigramMMBuilder::new();
    builder.update(&letters);
    // println!("{:?}", builder.map);

    let model = builder.to_model();
    let chain = BigramChain::new(model, (b't', b'h'));
    let buf = chain.take(1000).map(|i| i).collect::<Vec<_>>();
    let res = match String::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("{:?}", res);

    Ok(())
}

pub type Bigram<T> = (T, T);

pub struct BigramMMBuilder<T> {
    map: HashMap<Bigram<T>, Vec<T>>,
}

impl<T> BigramMMBuilder<T>
where
    T: Hash + Eq + Copy,
{
    pub fn new() -> BigramMMBuilder<T> {
        BigramMMBuilder {
            map: HashMap::new(),
        }
    }

    pub fn update(&mut self, items: &[T]) {
        let iter = items.windows(3);
        for triple in iter {
            let from = (triple[0], triple[1]);
            let to = triple[2];
            if let Some(existing) = self.map.get_mut(&from) {
                existing.push(to);
            } else {
                self.map.insert(from, vec![to]);
            }
        }
    }

    // pub fn merge(&mut self, other_map: &HashMap<Bigram<T>, Vec<T>>) {
    //     println!("OTHER #len={}", other_map.len());
    //     // self.map.extend(other_map);
    // }

    pub fn to_model(&self) -> BigramMM<T> {
        let map = self.map.clone();
        BigramMM { map }
    }
}

#[test]
fn test_bigram_mm_builder() {}

pub struct BigramMM<T> {
    pub map: HashMap<Bigram<T>, Vec<T>>,
}

impl<T> BigramMM<T>
where
    T: Hash + Eq + Copy,
{
    pub fn sample(&self, from: Bigram<T>) -> Option<T> {
        if let Some(next_items) = self.map.get(&from) {
            let choosen = next_items.choose(&mut rand::thread_rng()).unwrap();
            Some(*choosen)
        // Some(next_items[0]) // TODO select
        } else {
            None
        }
    }
}

#[test]
fn test_bigram_mm() {}

pub struct BigramChain<T> {
    model: BigramMM<T>,
    curr: Bigram<T>,
}

impl<'a, T> BigramChain<T> {
    pub fn new(model: BigramMM<T>, init: Bigram<T>) -> BigramChain<T> {
        BigramChain { model, curr: init }
    }
}

impl<T> Iterator for BigramChain<T>
where
    T: Hash + Eq + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(res) = self.model.sample(self.curr) {
            // update the iterator state
            self.curr = (self.curr.1, res);
            // and return what we found
            Some(res)
        } else {
            // Don't change state and return none
            None
        }
    }
}
