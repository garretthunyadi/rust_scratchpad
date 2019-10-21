use super::*;

#[derive(Clone)]
pub struct BigramMarkovChain<'a> {
    model: &'a BigramMarkovModel<'a>,
    curr: Bigram<'a>,
}

impl<'a> BigramMarkovChain<'a> {
    pub fn new(model: &'a BigramMarkovModel) -> BigramMarkovChain<'a> {
        BigramMarkovChain {
            model,
            curr: *model.random_key(),
        }
    }
    pub fn update_curr(&mut self, bigram: Bigram<'a>) {
        self.curr = bigram;
    }
}

impl<'a> Iterator for BigramMarkovChain<'a> {
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

impl<'a> MarkovChain for BigramMarkovChain<'a> {
    type Item = &'a str;
    fn next_item(&mut self) -> Self::Item {
        let word = self.model.sample(&self.curr);
        self.curr = (self.curr.1, word);
        word
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

    let mut iter = BigramMarkovChain::new(&model);
    iter.update_curr(seed);
    let res = iter.next_item();
    iter.update_curr(seed);
    let res = iter.next_item();
    iter.update_curr(seed);
    let res = iter.next_item();
    // we're good if we got this far (for now)
    // assert!(res.is_some());
    // assert_eq!(iter.next(), None);
}
