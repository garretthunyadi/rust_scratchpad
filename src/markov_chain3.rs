// /*
//     The story so far.

//     Have two examples of the model, one with a simple transition model (count flips) and
//     one generated from test (bigram model).

//     In order to use the markov models, I've implemented an iterator interface, eschewing a
//     similar, specific interface and getting all the iterator goodies for free.

//     Looking at the steps.
//     1. Build/Train a model
//     2. ?  Is there a use of the trained model outside of iteration?
//     3. Iterate over (use) the model.

//     Next:
//     1. The model itself is specific between the bigram and coin toss model.  Is there a way to make
//     the model more generic?
//         [] the text processing is not "ergonomic"

//     2. The bigram model uses string slices (which is probably the right choice).
//     2a. How much simpler
//     is would a String-based model look.
//     2b. Check what the memory looks like when running.

//     3. Implement a trigram model and compare.

//     4. Implement Rc/Arc variations, but see if I can keep the interface simple while providing
//     increasing control over training/threading and use.
// */
// use std::collections::HashMap;
// use std::rc::Rc;

// struct Corpus {
//     text: Rc<String>,
// }

// impl Corpus {
//     pub fn named(name: &str) -> Corpus {
//         Corpus {
//             text: Rc::from(String::from("a b c d e f g h i h a b c r r r ")),
//         }
//     }
//     pub fn from_text(text: &str) -> Corpus {
//         Corpus {
//             text: Rc::from(text.to_string()),
//         }
//     }
// }

// // type Bigram = (String, String);

// // struct BigramMarkovModel {
// // }

// // impl BigramMarkovModel {}

// // #[test]
// // fn test_bigram_markov_model() {
// //     corpus = Corpus::named("moby_dick")
// //     model = BigramMarkovModel::new(corpus)
// //     chain = model.chain(("call me"))
// // }

// type Trigram = (String, String, String);

// // The model itself can be cloned, it contains a Rc'd pointer to the underlying model data.
// #[derive(Clone)]
// struct TrigramMarkovModel {
//     map: Rc<HashMap<Trigram, String>>,
// }

// impl TrigramMarkovModel {
//     pub fn build(corpus: &Corpus) -> TrigramMarkovModel {
//         let map = HashMap::new();
//         // TODO: implement
//         TrigramMarkovModel { map: Rc::new(map) }
//     }

//     // pub fn add_text(&mut self, corpus: &Corpus) {
//     //     let map = HashMap::new(); // try to change the map (shouldn't do this, just seeing it if works)
//     //     self.map = Rc::new(map);
//     // }

//     pub fn chain_from(self, trigram: Trigram) -> TrigramMarkovChain {
//         TrigramMarkovChain {
//             model: self.clone(),
//             curr: trigram,
//         }
//     }
// }

// // The Trainer/Builder handles the learning
// // algorithm and follows the Builder Pattern in
// // Rust.
// // TODO: It should be trainable across threads.
// struct TrigramMarkovModelTrainer {
//     map: HashMap<Trigram, String>,
// }
// impl TrigramMarkovModelTrainer {
//     pub fn new() -> TrigramMarkovModelTrainer {
//         let map = HashMap::new();
//         TrigramMarkovModelTrainer { map }
//     }
//     pub fn update_model(&mut self, items: &[String]) {
//         let mut last = String::from("");
//         let mut last_last = String::from("");
//         for curr in items {
//             // process the new word
//             // TODO: lots of string cloning.
//             self.update_map(&(last_last.clone(), last.clone(), curr.to_string()));
//             last_last = last.clone();
//             last = curr.to_string();
//         }
//     }

//     fn update_map(&mut self, trigram: &Trigram) {}

//     pub fn to_model(&self) -> TrigramMarkovModel {
//         TrigramMarkovModel {
//             // make a clone of the now-trained hashmap
//             map: Rc::new(self.map.clone()),
//         }
//     }
// }

// #[test]
// fn test_trigram_markov_model_trainer() {
//     let corpus = Corpus::named("moby_dick");
//     let trainer = TrigramMarkovModelTrainer::new();
//     let text = corpus.text.to_string(); // TODO: is this copying the corpus string?

//     trainer.update_model(
//         &text
//             .split_ascii_whitespace()
//             .map(|s| String::from(s))
//             .collect(),
//     );
//     let model = trainer.to_model();
// }

// struct TrigramMarkovChain {
//     model: TrigramMarkovModel,
//     curr: Trigram,
// }

// /*
//     Bigram: Item is String, but the unit of training is a Bigram
//     Trigram: Item is String, but the unit of training is a Trigram
//     Both models rely on a sequence of strings
//     NGram is a possible extension, but itself might not be as generic as is appropriate.

//     NGram would be generic over the size of the n-gram

//     Generic Algorithm:
//     given a sequence of some items, collect the last N and use that to update the map
//     then when using, provide a start key and receive the next item.

//     Presumably, the results will track too closely to the original text. In order to
//     check this, we can look at the stats for the number of values-per-key.  For example,
//     if all the keys have only one value, then the model is degenerate and will simply repeat the text.

//     One thing to do here is to check for this and is there is only on value, we select either
//     that value, or a value from the n-1-gram.

//     N-1 Gram idea:
//         Here, we train a n-gram model to have not only the N-Grams, but also the
//         N-1 (and maybe N-2, etc?) values.  Then, if there is zero or one key for a given
//         N-Gram key, then we can intellegently "drop down" to select more generic text.
// */
// #[test]
// fn test_trigram_markov_model() {
//     let corpus = Corpus::named("moby_dick");
//     let model = TrigramMarkovModel::build(&corpus);
//     let chain = model.chain_from((
//         String::from("Call"),
//         String::from("me"),
//         String::from("Ishmael"),
//     ));
//     // let words = chain.take(100).collect::<Vec<_>>().join(" ");
//     // println!("{}", words);
// }

// pub fn main() -> std::io::Result<()> {
//     println!("markov_chain3");

//     Ok(())
// }
