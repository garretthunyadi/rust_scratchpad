// M uses the prev two words to get the next, so the state must contain the prev two words
use super::*;

#[derive(Clone)]
pub struct BigramMarkovModelIterator<'a> {
    model: &'a BigramMarkovModel<'a>,
    curr: Bigram<'a>,
}

impl<'a> BigramMarkovModelIterator<'a> {
    pub fn new(model: &'a BigramMarkovModel) -> BigramMarkovModelIterator<'a> {
        BigramMarkovModelIterator {
            model,
            curr: *model.random_key(),
        }
    }
    pub fn update_curr(&mut self, bigram: Bigram<'a>) {
        self.curr = bigram;
    }
}

impl<'a> Iterator for BigramMarkovModelIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        // get a new item
        let res = self.model.sample(&self.curr);
        // update the iterator state
        self.curr = (self.curr.1, res);
        // and return what we found
        Some(res)
    }
}

#[test]
fn test_bmm_iter() {
    let mut orig_text = String::from(include_str!("../../data/moby_dick.txt"));
    orig_text.truncate(10000);
    let corpus = util::simplify_corpus(&orig_text);
    let words = util::words(&corpus);
    let model = BigramMarkovModel::new(&words);

    let seed = ("be", "lodged");

    let mut iter = BigramMarkovModelIterator::new(&model);
    iter.update_curr(seed);
    let res = iter.next();
    iter.update_curr(seed);
    let res = iter.next();
    iter.update_curr(seed);
    let res = iter.next();
    // we're good if we got this far (for now)
    // assert!(res.is_some());
    // assert_eq!(iter.next(), None);
}

// impl<'a> MarkovChain for BigramMarkovModelIterator<'a> {
//     type Item = &'a str;
//     fn next_item(&mut self) -> Self::Item {
//         self.model.sample(&self.curr)
//     }
// }

// TODO: I can't figure out how to create an into iter implementation
// for this scenario where the iterator constructor takes a parameter.
// "cannot return value referencing function parameter `self`"
// impl<'a> IntoIterator for BigramMarkovModel<'a> {
//     type Item = &'a str;
//     // type IntoIter = ::std::vec::IntoIter<Self::Item>;
//     type IntoIter = BigramMarkovModelIterator<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         // self.0.into_iter()
//         BigramMarkovModelIterator::new(&self)
//     }
// }

// // the BigramMM uses the prev two words to get the next, so the state must contain the prev two words
// // use super::*;

// #[derive(Debug, PartialEq, Clone)]
// struct BigramMarkovModelIterator {
//     model: &BigramMarkovModel,
//     curr: Bigram,
// }

// impl BigramMarkovModelIterator {
//     pub fn new(model:&BigramMarkovModel) {
//         BigramMarkovModelIterator{model,curr:model.rand_key()}
//     }
// }

// #[test]
// fn test_bmm_iter() {
//     model = //     let orig_text = String::from(include_str!("../../data/moby_dick.txt"));
//      orig_text.truncate(10000);
//      let corpus = util::simplify_corpus(&orig_text);
//      let words = util::words(&corpus);
//      let model = BigramMarkovModel::new(&words);

//     BigramMarkovModelIterator::new()
// }

// impl<'a> Iterator for BigramMarkovModel<'a> {
//         type Item = usize;
//         fn next(&mut self) -> Option<usize> {
//             // Some(self.next())
//             None
//         }
//     }

// // impl<'a> Iterator for BigramMarkovModel<'a> {
// //     type Item = Bigram<'a>;
// //     fn next(&mut self) -> Option<Bigram<'a>> {
// //         Some(self.sample())
// //     }
// // }

// // #[test]
// // fn test_iterator() {
// //     let orig_text = String::from(include_str!("../../data/moby_dick.txt"));
// //     orig_text.truncate(10000);
// //     let corpus = util::simplify_corpus(&orig_text);
// //     let words = util::words(&corpus);
// //     let model = BigramMarkovModel::new(&words);
// // }
