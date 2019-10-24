use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    mutex();
    let _ = multiple_threads_multiple_updates(1, 1);
    mutex_based_markov_training();
}
pub fn mutex() {
    println!("mutex");

    let s = String::from("a value");
    // note the to change the value, a mutable ref to
    // the mutex is needed.
    let mut mutex = Mutex::from(s);
    // let y = s; // compiler error, 's' was moved.
    let m = mutex.get_mut().unwrap();
    *m = String::from("a new value");
    println!("{:?}", mutex);

    // with threads, we need to share ownership
    // of the mutex (Rc), and we need to do that
    // across threads (Rc->Arc)
    let arc = Arc::new(mutex);
    let mut handles = vec![];

    for i in 0..10 {
        // let arc_copy = arc.clone(); // value moved into closure here, in previous iteration of loop

        let arc_copy = Arc::clone(&arc);
        let handle = thread::spawn(move || {
            let mut v = arc_copy.lock().unwrap();
            *v = format!("a value from thread {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
        print!(".");
    }
    println!("fin: {}", arc.lock().unwrap());
}

#[test]
fn test_mutex() {
    mutex()
}

fn multiple_threads_multiple_updates(
    num_threads: usize,
    num_updates: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    let s = String::from("a value");
    // note the to change the value, a mutable ref to
    // the mutex is needed.
    let mut mutex = Mutex::from(s);
    // let y = s; // compiler error, 's' was moved.
    let m = mutex.get_mut().unwrap();
    *m = String::from("a new value");
    println!("{:?}", mutex);

    // with threads, we need to share ownership
    // of the mutex (Rc), and we need to do that
    // across threads (Rc->Arc)
    let arc = Arc::new(mutex);
    let mut handles = vec![];

    for i in 0..num_threads {
        // let arc_copy = arc.clone(); // value moved into closure here, in previous iteration of loop

        let arc_copy = Arc::clone(&arc);
        let handle = thread::spawn(move || {
            for j in 0..num_updates {
                let mut v = arc_copy.lock().unwrap();
                *v = format!("a value from thread {}", i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("fin: {}", arc.lock().unwrap());

    start.elapsed()
}

#[test]
fn test_multiple_threads_multiple_updates() {
    fn run(threads: usize, updates: usize) {
        let duration = multiple_threads_multiple_updates(threads, updates);
        println!(
            "Duration |\t threads:{}\t updates:{}\t took {:?}",
            threads, updates, duration
        );
    }
    run(1, 1); //      took 249.14µs
    run(10, 1); //     took 385.93µs
    run(100, 1); //    took 3.326728ms
    run(1000, 1); //   took 25.920036ms
                  // run(10000, 1); //  took 891.27842ms
                  //                // run(100_000, 1);
                  //                //  took 53.872151985s

    run(1, 10); //      took 79.305µs
    run(10, 100); //     took 2.345404ms
    run(100, 100); //    took 24.098697ms

    let cpus = num_cpus::get(); // 16
    println!("# CPUS: {}", cpus);
    run(1, 10000); //   took 3.562487ms
                   // run(1, 100_000); // took 35.486782ms

    // For these simple updates, the thread overhead seems to be overwhelming.
    run(10, 10000); //   took 182.574178ms
                    // run(10, 100_000); // took 1.551887012s
                    //                   // run(10, 1_000_000); // took 17.422898186s

    run(16, 10000); //   took 362.686317ms
                    // run(16, 100_000); // took 3.57246548s
                    // run(16, 1_000_000); // took

    // the same
}

//  Markov Training with Mutex
/*
    // Moby Dick
    let mut builder = BigramMarkovModelMutexBasedBuilder::new();
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

*/

use crate::markov_chain4::{Bigram, BigramChain, BigramMM};
use std::collections::HashMap;
use std::hash::Hash;

pub fn mutex_based_markov_training() {
    println!("mutex_based_markov_training");

    // Moby Dick
    let builder = BigramMarkovModelMutexBasedBuilder::new();

    const MOBY_DICK: &str = include_str!("../data/moby_dick.txt");

    let corpus = super::markov_chain::util::simplify_corpus(&MOBY_DICK).to_lowercase();
    // const corpus: String = corpus.to_lowercase();

    // let orig_text = String::from(include_str!("../data/moby_dick.txt"));
    // // orig_text.truncate(10000);
    // let corpus = super::markov_chain::util::simplify_corpus(&orig_text);
    // let corpus = corpus.to_lowercase();

    let cpus = num_cpus::get(); // 16
    let chunk_size = corpus.len() / cpus;
    println!("Chunk size: {} * {} = {}", chunk_size, cpus, chunk_size * 8);

    let start = std::time::Instant::now();

    // let chunks = sub_strings(&corpus, chunk_size);
    let chunks = sub_strings(MOBY_DICK, chunk_size);
    println!("Num chunks: {} ", &chunks.len());
    let mut handles = vec![];

    // let txref = &tx;
    for chunk in chunks {
        let mut builder_clone = builder.clone();
        let handle = thread::spawn(move || {
            let words = super::markov_chain::util::words(&chunk);
            // for _ in 1..50 {
            builder_clone.update(&words);
            // }
        });
        handles.push(handle);
    }

    // for i in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut v = arc_copy.lock().unwrap();
    //         *v = format!("a value from thread {}", i);
    //     });
    //     handles.push(handle);
    // }

    for handle in handles {
        handle.join().unwrap();
        print!(".");
    }
    let model = builder.to_model();
    let chain = BigramChain::new(model, ("call", "me"));
    let res = chain.take(1000).collect::<Vec<_>>().join(" ");
    println!("{:?}", res);
    let duration = start.elapsed();
    println!("fin in {:?}", duration);
}

fn sub_strings<'a>(string: &'a str, sub_len: usize) -> Vec<&'a str> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}

#[test]
fn test_mutex_based_markov_training() {
    mutex_based_markov_training();
}

#[derive(Clone)]
pub struct BigramMarkovModelMutexBasedBuilder<T> {
    // map: HashMap<Bigram<T>, Vec<T>>,
    map: std::sync::Arc<std::sync::Mutex<HashMap<Bigram<T>, Vec<T>>>>,
}

impl<T> BigramMarkovModelMutexBasedBuilder<T>
where
    T: Hash + Eq + Copy,
{
    pub fn new() -> BigramMarkovModelMutexBasedBuilder<T> {
        BigramMarkovModelMutexBasedBuilder {
            map: Arc::new(Mutex::from(HashMap::new())),
        }
    }

    pub fn update(&mut self, items: &[T]) {
        let iter = items.windows(3);
        for triple in iter {
            let from = (triple[0], triple[1]);
            let to = triple[2];
            let mut hash_map = self.map.lock().unwrap();
            if let Some(existing) = hash_map.get_mut(&from) {
                existing.push(to);
            } else {
                hash_map.insert(from, vec![to]);
            }
        }
    }

    pub fn to_model(&self) -> BigramMM<T> {
        BigramMM {
            map: self.map.lock().unwrap().clone(),
        }
    }
}
