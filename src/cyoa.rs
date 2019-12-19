extern crate rand;
use rand::distributions::{Distribution, Uniform};
use std::fmt;

pub fn main() {
    println!("cyoa/main");
    let mut story = Story::start();
    while !story.is_fin() {
        story.next();
        println!("{:?}", story);
    }
}

#[derive(PartialEq)]
pub struct Story {
    title: String,
    state: StoryState,
}
impl Story {
    pub fn start() -> Story {
        Story {
            title: "".to_string(),
            state: StoryState::start(),
        }
    }

    pub fn next(&mut self) {
        self.state = self.state.next();
    }

    pub fn is_fin(&self) -> bool {
        self.state == StoryState::Fin
    }
}

impl fmt::Debug for Story {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?})", self.state)
    }
}

#[derive(Debug, PartialEq)]
pub enum StoryState {
    Start,
    Middle(String, usize),
    Fin,
}

impl StoryState {
    pub fn start() -> StoryState {
        StoryState::Start
    }

    pub fn next(&self) -> StoryState {
        match self {
            StoryState::Start => StoryState::Middle("Middle".to_string(), 1),
            StoryState::Middle(_, step) => {
                let mut rng = rand::thread_rng();
                let die = Uniform::from(1..7);
                if die.sample(&mut rng) == 6 {
                    StoryState::Fin
                } else {
                    StoryState::Middle(format!("Story, step {}", step + 1), step + 1)
                }
            }
            StoryState::Fin => StoryState::Fin,
        }
    }
}
