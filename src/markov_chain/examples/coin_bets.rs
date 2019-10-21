use super::super::*;
pub fn main() -> std::io::Result<()> {
    println!("coin bets markov chain example");

    // the user (player) starts with $10 and bets $1 on coin toss.
    // the markov chain models the state progression.
    let mut bets = CoinBetsMarkovChain { balance: 2 };
    let amt = bets.next_item();
    println!("{}", amt);
    let amt = bets.next_item();
    println!("{}", amt);
    let amt = bets.next_item();
    println!("{}", amt);

    Ok(())
}

pub struct CoinBetsMarkovChain {
    balance: u32,
}

impl CoinBetsMarkovChain {
    pub fn new(balance: u32) -> CoinBetsMarkovChain {
        CoinBetsMarkovChain { balance }
    }
}

impl MarkovChain for CoinBetsMarkovChain {
    type Item = u32;
    fn next_item(&mut self) -> Self::Item {
        if self.balance > 0 {
            self.balance -= 1;
        }
        self.balance
    }
}

#[test]
fn test_coin_bets_markov_chain() {
    let mut bets = CoinBetsMarkovChain { balance: 2 };
    let amt = bets.next_item();
    println!("{}", amt);
    let amt = bets.next_item();
    println!("{}", amt);
    let amt = bets.next_item();
    println!("{}", amt);
}
