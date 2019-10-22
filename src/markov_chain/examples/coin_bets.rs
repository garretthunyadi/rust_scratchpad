pub fn main() -> std::io::Result<()> {
    println!("coin bets markov chain example");

    // the user (player) starts with $10 and bets $1 on coin toss.
    // the markov chain models the state progression.
    let bets = CoinBetsMarkovChain { balance: 10 };
    for amt in bets {
        print!("{:?} ", amt);
    }
    println!();
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

// impl MarkovChain for CoinBetsMarkovChain {
//     type Item = u32;
//     fn next_item(&mut self) -> Self::Item {
//         if self.balance > 0 {
//             self.balance -= 1;
//         }
//         self.balance
//     }
// }

impl Iterator for CoinBetsMarkovChain {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        use rand::Rng;

        if self.balance == 0 {
            None
        } else {
            if rand::thread_rng().gen() {
                self.balance += 1;
            } else {
                self.balance -= 1;
            }

            Some(self.balance)
        }
    }
}

#[test]
fn test_coin_bets_markov_chain() {
    let mut bets = CoinBetsMarkovChain { balance: 2 };
    let amt = bets.next();
    println!("{:?}", amt);
    let amt = bets.next();
    println!("{:?}", amt);
    let amt = bets.next();
    println!("{:?}", amt);
}
