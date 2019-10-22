use std::collections::HashMap;
use std::hash::Hash;

pub fn main() -> std::io::Result<()> {
    println!("markov_chain4");

    // Simple numbers
    let mut builder = BigramMMBuilder::new();
    builder.update(&[1, 2, 3, 4, 5]);
    let model = builder.to_model();
    println!("{:?}", model.map.keys());
    println!("{:?}", model.map.values());

    let chain = BigramChain::new(model, (1, 2));
    let res = chain.take(100).collect::<Vec<_>>();
    println!("{:?}", res);

    // Moby Dick
    let mut builder = BigramMMBuilder::new();
    let mut orig_text = String::from(include_str!("../data/moby_dick.txt"));
    orig_text.truncate(10000);
    let corpus = super::markov_chain::util::simplify_corpus(&orig_text);
    let corpus = corpus.to_lowercase();
    let words = super::markov_chain::util::words(&corpus);
    builder.update(&words);
    let model = builder.to_model();
    let chain = BigramChain::new(model, ("call", "me"));
    let res = chain.take(1000).collect::<Vec<_>>().join(" ");
    println!("{:?}", res);

    Ok(())
}

type Bigram<T> = (T, T);

struct BigramMMBuilder<T> {
    map: HashMap<Bigram<T>, Vec<T>>,
}

impl<T> BigramMMBuilder<T>
where
    T: Hash + Eq + Copy,
{
    fn new() -> BigramMMBuilder<T> {
        BigramMMBuilder {
            map: HashMap::new(),
        }
    }

    fn update(&mut self, items: &[T]) {
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

    fn to_model(&self) -> BigramMM<T> {
        let map = self.map.clone();
        BigramMM { map }
    }
}

#[test]
fn test_bigram_mm_builder() {}

struct BigramMM<T> {
    map: HashMap<Bigram<T>, Vec<T>>,
}

impl<T> BigramMM<T>
where
    T: Hash + Eq + Copy,
{
    fn sample(&self, from: Bigram<T>) -> Option<T> {
        if let Some(next_items) = self.map.get(&from) {
            Some(next_items[0]) // TODO select
        } else {
            None
        }
    }
}

#[test]
fn test_bigram_mm() {}

struct BigramChain<T> {
    model: BigramMM<T>,
    curr: Bigram<T>,
}

impl<'a, T> BigramChain<T> {
    fn new(model: BigramMM<T>, init: Bigram<T>) -> BigramChain<T> {
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
