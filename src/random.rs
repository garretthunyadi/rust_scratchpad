extern crate rand;
extern crate rand_distr;
use rand::distributions::{Distribution, Uniform};
use rand_distr::Normal;

pub fn main() {
    println!("cyoa/main");
    // let mut story_state = Story::start();
    // while story_state != Story::End {
    //     story_state = story_state.next();
    //     println!("{:?}", story_state);
    // }

    let dist = Uniform::from(1..7);

    let d6_first = Dice::new(6);
    let d6_second = Dice::new(6);
    let d10 = Dice::new(10);

    let x: bool = rand::random();
    println!("bool:{}\n{:?}", x, d6_first);

    // let mut rng = rand::thread_rng();
    // let height = Normal::new(5.5, 0.5);

    let normal = Normal::new(5.5, 0.5).unwrap();
    let height = normal.sample(&mut rand::thread_rng());
    println!("height: {} ", height);
}

// Dice
// Point -- custom, multi value type

#[derive(Debug)]
pub struct Dice {
    pub num_sides: usize,
    pub current_value: usize,
    rng: rand::prelude::ThreadRng,
    dist: rand::distributions::Uniform<usize>,
}

impl Dice {
    pub fn new(num_sides: usize) -> Dice {
        let mut rng = rand::thread_rng();
        let dist = Uniform::from(1..=num_sides);
        let current_value = dist.sample(&mut rng);
        Dice {
            num_sides,
            current_value,
            rng,
            dist,
        }
    }

    pub fn roll(&mut self) -> usize {
        self.current_value = self.dist.sample(&mut self.rng);
        self.current_value
    }
}

impl PartialEq for Dice {
    fn eq(&self, other: &Dice) -> bool {
        self.num_sides == other.num_sides && self.current_value == other.current_value
    }
}

#[test]
fn test_partial_eq() {
    let mut d6_first = Dice::new(6);
    let d6_second = Dice::new(6);
    let mut d10 = Dice::new(10);

    assert_eq!(d6_first, d6_first);
    assert_ne!(d10, d6_first);

    for _ in 0..1000 {
        let res = d6_first.roll();
        assert!(res > 0);
        assert!(res <= 6);
    }

    for _ in 0..1000 {
        let res = d10.roll();
        assert!(res > 0 && res <= 10);
    }
}

// impl Story {
//     pub fn start() -> Story {
//         Story::Start
//     }

//     pub fn next(self) -> Story {
//         match self {
//             Story::Start => Story::Middle("Middle".to_string(), 1),
//             Story::Middle(_, step) => {
//                 let mut rng = rand::thread_rng();
//                 let die = Uniform::from(1..7);
//                 if die.sample(&mut rng) == 6 {
//                     Story::End
//                 } else {
//                     Story::Middle(format!("Story, step {}", step + 1), step + 1)
//                 }
//             }
//             Story::End => Story::End,
//         }
//     }
// }
