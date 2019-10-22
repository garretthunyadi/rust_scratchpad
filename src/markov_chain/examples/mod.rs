pub mod bigram_markov_chain_example;
pub mod coin_bets;

use super::*;
pub fn main() -> std::io::Result<()> {
    println!("--- (1) coin bets markov chain example ---");
    // the user (player) starts with $10 and bets $1 on coin toss.
    // the markov chain models the state progression.
    let bets = coin_bets::CoinBetsMarkovChain::new(10);
    for amt in bets {
        print!("{:?} ", amt);
    }
    println!();

    println!("--- (2) bigram markov chain example ---");
    let mut orig_text = String::from(include_str!("../../../data/moby_dick.txt"));
    orig_text.truncate(10000);
    let corpus = util::simplify_corpus(&orig_text);
    let words = util::words(&corpus);

    let first_bigram = (*words.get(0).unwrap(), *words.get(1).unwrap());
    println!("first_bigram: {:?}", first_bigram);

    let model = BigramMarkovModel::new(&words);
    let chain = super::bigram_markov_chain::BigramMarkovChain::new(&model);

    for (i, word) in chain.clone().enumerate() {
        print!("{} ", word);
        if i > 100 {
            break;
        }
    }
    println!();

    let words = chain.take(100).collect::<Vec<_>>().join(" ");
    println!("{}", words);

    Ok(())
}
