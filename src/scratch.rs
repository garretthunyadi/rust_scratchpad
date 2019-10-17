pub fn main() {
    println!("here scratch");
    let thing1 = Thing { id: 100 };
    println!("{}", thing1.speak());
    thing1.listen_to(&thing1);
    let thing2 = Thing { id: 200 };
    thing1.listen_to(&thing2);
    thing2.listen_to(&thing1);
    thing2.listen_to(&thing2);

    let thing3: Thing = thing2.generate();
    println!("Thing 3 id is {}", thing3.id);
    let bizaro1: BizarroThing = thing1.generate();
    println!("Bizarro 1 id is {}", bizaro1.id);

    let baby: Thing = thing1.give_birth();
    println!("Thing 1 gave birth to {}", baby.id);
}

#[derive(PartialEq)]
struct Thing {
    id: usize,
}

struct BizarroThing {
    id: isize, // always negative
}

impl<'a> std::cmp::PartialEq<dyn Speaker + 'a> for Thing {
    fn eq(&self, speaker: &dyn Speaker) -> bool {
        self.id == speaker.id()
    }
}

trait Speaker {
    fn speak(&self) -> String;
    fn id(&self) -> usize;
}

impl Speaker for Thing {
    fn speak(&self) -> String {
        String::from("...")
    }
    fn id(&self) -> usize {
        self.id
    }
}

trait Listener {
    fn listen_to(&self, speaker: &dyn Speaker);
}

impl Listener for Thing {
    fn listen_to(&self, speaker: &dyn Speaker) {
        if self == speaker {
            println!("I like to hear myself talk");
        } else {
            println!(
                "I hear nothing, though the speaker said {}",
                speaker.speak()
            );
        }
    }
}

trait Generator<T> {
    fn generate(&self) -> T;
}

impl Generator<Thing> for Thing {
    fn generate(&self) -> Thing {
        Thing { id: self.id + 1 }
    }
}
impl Generator<BizarroThing> for Thing {
    fn generate(&self) -> BizarroThing {
        BizarroThing {
            id: -(self.id as isize),
        }
    }
}

trait Procreate {
    type Item;
    fn give_birth(&self) -> Self::Item;
}

impl Procreate for Thing {
    type Item = Thing;
    fn give_birth(&self) -> Thing {
        Thing { id: self.id + 1 }
    }
}
