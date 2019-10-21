use super::super::*;

pub fn main() -> std::io::Result<()> {
    println!("bigram markov chain example");

    let orig_text = String::from(include_str!("../../../data/moby_dick.txt"));
    // orig_text.truncate(100000);
    // println!("orig : {:?}", words(&orig_text));

    // create one string that we can use slices from
    let corpus = util::simplify_corpus(&orig_text);
    // println!("words: {:?}", words(&corpus.clone().truncate(100)));
    // corpus.truncate(1000);
    let words = util::words(&corpus);
    // println!("words: {:?}", words);

    let first_bigram = (*words.get(0).unwrap(), *words.get(1).unwrap());
    println!("first_bigram: {:?}", first_bigram);

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
    for (i, word) in iter.clone().enumerate() {
        print!("{} ", word);
        if i == 10 {
            break;
        }
    }
    for (i, word) in iter.clone().enumerate() {
        if i > 100 {
            break;
        }
        // print!("{} ", word);
    }
    println!();

    let res = iter.clone().take(100).collect::<Vec<_>>().join(" ");
    println!("{:?}", res);

    // Into Iter
    // let iter = model.iter();

    Ok(())
}
