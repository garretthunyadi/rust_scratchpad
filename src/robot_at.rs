use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::marker::PhantomData;

fn error(msg: &str) -> Result<(), Error> {
    Err(Error::new(ErrorKind::Other, msg))
}

pub fn main() -> Result<(), Error> {
    puts!("Robot");
    Ok(())
}

#[derive(PartialEq, Debug, Clone)]
enum Command {
    Stir,
    TakeIngredient(Ingredient),   // from local storage
    RemoveIngredient(Ingredient), // from local storage
    GrabIngredient(Ingredient),   // from prep area
    ScoopIngredient(Ingredient),  // from prep area
}
// #[derive(Hash, Eq, PartialEq, Debug)]
// enum Location {
//     Fridge,
//     Pantry,
//     PrepArea,
// }

struct Fridge {}
impl Fridge {
    pub fn get_refrigerated_item(ing: Ingredient) -> Result<Command, Error> {
        Ok(Command::TakeIngredient(ing))
    }
}

struct Pantry {}
impl Pantry {
    pub fn get_pantry_item(ing: Ingredient) -> Result<Command, Error> {
        match ing {
            Ingredient::Flour => Ok(Command::TakeIngredient(ing)),
            Ingredient::Cocoa => Ok(Command::TakeIngredient(ing)),
            Ingredient::Sugar => Ok(Command::TakeIngredient(ing)),
            ing => Err(Error::new(
                ErrorKind::Other,
                format!("Can't get {:?} from pantry", ing),
            )),
        }
    }
}

struct PrepArea {}
impl PrepArea {
    // pub fn stir(&mut self) -> Result<Command, Error> {
    //     Ok(Command::Stir)
    // }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Ingredient {
    Eggs,
    Butter,
    Milk,
    Flour,
    Cocoa,
    Sugar,
}

// would need to make sifferent structs for this.
// pub trait FridgeStorage {}
// pub trait PantryStorage {}

type InventoryItem = (Ingredient, u16);
type Inventory = HashMap<Ingredient, u16>;

trait Location {}

// #[derive(PartialEq, Debug)]
struct Robot {
    location: &'static dyn Location,
    inventory: Inventory,
}
// impl Robot {
//     fn new() -> Robot {
//         Robot {
//             location: &<RobotAt<PrepArea>>::new(),
//             inventory: HashMap::new(),
//         }
//     }
// }

#[derive(PartialEq, Debug)]
struct RobotAt<L: Sized> {
    inventory: Inventory,
    phantom: PhantomData<L>,
}
impl<L> RobotAt<L> {
    // private
    fn inventory_count(&self, ing: Ingredient) -> u16 {
        *self.inventory.get(&ing).or(Some(&0)).unwrap()
    }
    fn add_to_inventory(&mut self, ing: Ingredient) -> Result<Command, Error> {
        // here I'm both mutating state and returning a command.  In what cases would this
        // be appropriate? If I'm both simulating and generating commands.
        let count = self.inventory.entry(ing.clone()).or_insert(0);
        *count += 1;
        Ok(Command::TakeIngredient(ing))
    }
    fn remove_from_inventory(&mut self, ing: Ingredient) -> Result<Command, Error> {
        // as with add_to_inventory, this mutates state and also returns a command.
        let count = self.inventory.entry(ing.clone()).or_insert(0);
        if *count > 0 {
            *count -= 1;
            Ok(Command::RemoveIngredient(ing))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("Can't get {:?} from pantry", ing),
            ))
        }
    }

    // given a recipe, collect ingredients and prepare one instance
    // we can do this here, because this involves multiple instances of the robot,
    // the mental mapping is broken. :(
    fn make_recipe(&mut self) {}
}

impl RobotAt<PrepArea> {
    pub fn new() -> RobotAt<PrepArea> {
        RobotAt {
            inventory: Inventory::new(),
            phantom: PhantomData,
        }
    }
    pub fn move_to_fridge(self) -> Result<RobotAt<Fridge>, Error> {
        Ok(self.into())
    }
    pub fn move_to_pantry(self) -> Result<RobotAt<Pantry>, Error> {
        Ok(self.into())
    }
    pub fn unload(&mut self, ing: Ingredient) -> Result<Command, Error> {
        self.remove_from_inventory(ing)
    }
    pub fn stir(&mut self) -> Result<Command, Error> {
        Ok(Command::Stir)
    }
    pub fn grab(&mut self, ing: Ingredient) -> Result<Command, Error> {
        match ing.clone() {
            Ingredient::Eggs => Ok(Command::GrabIngredient(ing)),
            Ingredient::Butter => Ok(Command::GrabIngredient(ing)),
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("Can't grab {:?}", ing),
            )),
        }
    }
    pub fn scoop(&mut self, ing: Ingredient) -> Result<Command, Error> {
        match ing.clone() {
            Ingredient::Milk => Ok(Command::GrabIngredient(ing)),
            Ingredient::Flour => Ok(Command::GrabIngredient(ing)),
            Ingredient::Cocoa => Ok(Command::GrabIngredient(ing)),
            Ingredient::Sugar => Ok(Command::GrabIngredient(ing)),
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("Can't scoop {:?}", ing),
            )),
        }
    }
}
impl Location for RobotAt<PrepArea> {}
impl From<RobotAt<Fridge>> for RobotAt<PrepArea> {
    fn from(curr: RobotAt<Fridge>) -> RobotAt<PrepArea> {
        RobotAt {
            inventory: curr.inventory,
            phantom: PhantomData,
        }
    }
}
impl From<RobotAt<Pantry>> for RobotAt<PrepArea> {
    fn from(curr: RobotAt<Pantry>) -> RobotAt<PrepArea> {
        RobotAt {
            inventory: curr.inventory,
            phantom: PhantomData,
        }
    }
}

impl RobotAt<Fridge> {
    pub fn move_to_prep_area(self) -> Result<RobotAt<PrepArea>, Error> {
        Ok(self.into())
    }
    pub fn move_to_pantry(self) -> Result<RobotAt<Pantry>, Error> {
        Ok(self.into())
    }
}
impl Location for RobotAt<Fridge> {}

impl From<RobotAt<Pantry>> for RobotAt<Fridge> {
    fn from(curr: RobotAt<Pantry>) -> RobotAt<Fridge> {
        RobotAt {
            inventory: curr.inventory,
            phantom: PhantomData,
        }
    }
}
impl From<RobotAt<PrepArea>> for RobotAt<Fridge> {
    fn from(curr: RobotAt<PrepArea>) -> RobotAt<Fridge> {
        RobotAt {
            inventory: curr.inventory,
            phantom: PhantomData,
        }
    }
}

impl RobotAt<Pantry> {
    pub fn move_to_prep_area(self) -> Result<RobotAt<PrepArea>, Error> {
        Ok(self.into())
    }
    pub fn move_to_fridge(self) -> Result<RobotAt<Fridge>, Error> {
        Ok(self.into())
    }

    pub fn load(&mut self, ing: Ingredient) -> Result<Command, Error> {
        self.add_to_inventory(ing)
    }
}
impl Location for RobotAt<Pantry> {}

impl From<RobotAt<Fridge>> for RobotAt<Pantry> {
    fn from(curr: RobotAt<Fridge>) -> RobotAt<Pantry> {
        RobotAt {
            inventory: curr.inventory,
            phantom: PhantomData,
        }
    }
}
impl From<RobotAt<PrepArea>> for RobotAt<Pantry> {
    fn from(curr: RobotAt<PrepArea>) -> RobotAt<Pantry> {
        RobotAt {
            inventory: curr.inventory,
            phantom: PhantomData,
        }
    }
}

#[test]
fn test_robot_at() {
    let robot = <RobotAt<PrepArea>>::new();
    let mut robot = robot.move_to_pantry().unwrap();
    assert_eq!(0, robot.inventory_count(Ingredient::Butter));

    let cmd = robot.load(Ingredient::Butter).unwrap();
    assert_eq!(1, robot.inventory_count(Ingredient::Butter));
    assert_eq!(Command::TakeIngredient(Ingredient::Butter), cmd);
    // to the prep area
    let mut robot = robot.move_to_prep_area().unwrap();
    let cmd = robot.unload(Ingredient::Butter).unwrap();
    assert_eq!(0, robot.inventory_count(Ingredient::Butter));
    assert_eq!(Command::RemoveIngredient(Ingredient::Butter), cmd);

    assert!(robot.unload(Ingredient::Butter).is_err());
    assert_eq!(0, robot.inventory_count(Ingredient::Butter));

    let cmd = robot.stir().unwrap();
    assert_eq!(Command::Stir, cmd);
}

#[test]
fn test_robot_at2() {
    let robot = <RobotAt<PrepArea>>::new();
    let mut robot = robot.move_to_pantry().unwrap();
    let _ = robot.load(Ingredient::Butter).unwrap();
    let _ = robot.load(Ingredient::Eggs).unwrap();
    let mut robot = robot.move_to_prep_area().unwrap();

    let _ = robot.unload(Ingredient::Eggs).unwrap();
    let _ = robot.unload(Ingredient::Butter).unwrap();
    let _ = robot.stir().unwrap();
}

// #[test]
// fn test_robot() {
//     let robot = Robot::new();
// }

struct Recipe {
    ingredients: Vec<InventoryItem>,
    steps: Vec<Command>,
}
