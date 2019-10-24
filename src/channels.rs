// test
use crate::markov_chain4::{BigramChain, BigramMM, BigramMMBuilder};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn main() -> std::io::Result<()> {
    println!("channels");
    let start = std::time::Instant::now();

    let (tx, rx) = mpsc::channel();

    const MOBY_DICK: &str = include_str!("../data/moby_dick.txt");
    let corpus = super::markov_chain::util::simplify_corpus(&MOBY_DICK);
    let corpus = corpus.to_lowercase();
    // const CORPUS: &str = corpus;
    // const MOBY_DICK_WORDS: Vec<&str> = super::markov_chain::util::words(&corpus);

    println!("Text len: {}", MOBY_DICK.len());
    let cpus = num_cpus::get();

    let chunk_size = MOBY_DICK.len() / cpus;
    println!("Chunk size: {} * {} = {}", chunk_size, cpus, chunk_size * 8);

    let chunks = sub_strings(&MOBY_DICK, chunk_size);
    println!("Num chunks: {} ", &chunks.len());

    // let txref = &tx;
    for chunk in chunks {
        let thread_tx = tx.clone();
        // let thread_tx = txref.clone();

        println!("chunk len = {}", chunk.len());
        thread::spawn(move || {
            process_to_channel(chunk, thread_tx);
            // println!("val is {}", val);
        });
    }

    // let mut final_model = BigramMM{HashMap<Bigram<T>, Vec<T>>,
    let mut final_map = HashMap::new();

    for i in 0..cpus {
        let recd_model = rx.recv().unwrap();
        // main_builder.merge(&recd_model.map);
        for (from, tos) in recd_model.map {
            for to in tos {
                update_final_map(&mut final_map, from, to);
            }
        }
        // println!("Got: {}", received);
    }

    println!("Final count: {}", final_map.len());

    let model = BigramMM { map: final_map };
    let chain = BigramChain::new(model, ("call", "me"));
    let res = chain.take(1000).collect::<Vec<_>>().join(" ");
    println!("{:?}", res);
    let duration = start.elapsed();

    println!(
        "Time elapsed in markov channel-based training (all cores) is: {:?}",
        duration
    );

    Ok(())
}

fn update_final_map(
    map: &mut HashMap<(&'static str, &'static str), Vec<&'static str>>,
    from: (&'static str, &'static str),
    to: &'static str,
) {
    if let Some(existing) = map.get_mut(&from) {
        existing.push(to);
    } else {
        map.insert(from, vec![to]);
    }
}

fn process_to_channel(chunk: &'static str, chan: std::sync::mpsc::Sender<BigramMM<&'static str>>) {
    let mut builder = BigramMMBuilder::new();
    // let orig_text = String::from(include_str!("../data/moby_dick.txt"));
    // orig_text.truncate(10000);
    // let corpus: &'static str = super::markov_chain::util::simplify_corpus(chunk); // .to_lowercase();
    // const CORPUS: &str = corpus;
    let words: Vec<&'static str> = super::markov_chain::util::words(&chunk)
        .into_iter()
        // .map(|s| String::from(s))
        .collect();
    builder.update(&words);
    let model = builder.to_model();
    chan.send(model).unwrap();
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

/*
    // Moby Dick
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

*/
