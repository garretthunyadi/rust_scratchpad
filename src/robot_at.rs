// #![warn(dead_code)]
// #![warn(unused_variables)]
// #![warn(unused_macros)]

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::marker::PhantomData;

/*
TODO
  [] Use traits Grabbable, Location for trait bounds
*/

fn error(msg: String) -> Error {
    Error::new(ErrorKind::Other, msg)
}

pub fn main() -> Result<(), Error> {
    puts!("RobotAt");
    fluent_scenario()?;
    Ok(())
}

#[derive(PartialEq, Debug, Clone)]
enum Command {
    Stir,
    TakeIngredient(Ingredient),           // from local storage
    RemoveIngredient(Ingredient),         // from local storage
    GrabIngredient(GrabbableIngredient),  // from prep area
    ScoopIngredient(ScoopableIngredient), // from prep area
}
// #[derive(Hash, Eq, PartialEq, Debug)]
// enum Location {
//     Fridge,
//     Pantry,
//     PrepArea,
// }

trait Location {}

// Obtainable means that we can obtain an ingrediant or a scoop/pan, but says nothing about how we obtain it.
// The way that the robot works, though, we do care about the how.  The robot has a "grab" and a "scoop" command
// that we work with.
trait Obtainable {
    // fn obtain(&self) -> Result<Command, Error>;
}

// this trait provides a specialization, where we care about how we obtain the obtainable.
// it is appropriate to the robot's specific commands, specifically "grab" in this case.
trait Grabbable: Obtainable {
    fn grab(&self) -> Result<Command, Error>;
    // fn obtain(&self) -> Result<Command, Error> {
    //     self.grab()
    // }
}

// this trait provides a specialization, where we care about how we obtain the obtainable.
// it is appropriate to the robot's specific commands, specifically "scoop" in this case.
trait Scoopable: Obtainable {
    fn scoop(&self) -> Result<Command, Error>;
    // fn obtain(&self) -> Result<Command, Error> {
    //     self.scoop()
    // }
}

#[derive(PartialEq, Debug, Clone)]
struct Fridge {}
impl Fridge {
    pub fn get_refrigerated_item(ing: Ingredient) -> Result<Command, Error> {
        Ok(Command::TakeIngredient(ing))
    }
}
impl Location for Fridge {}

#[derive(PartialEq, Debug, Clone)]
struct Pantry {}
impl Pantry {
    pub fn get_pantry_item(ingr: Ingredient) -> Result<Command, Error> {
        use Ingredient::*;
        match ingr {
            ingr @ Flour | ingr @ Cocoa | ingr @ Sugar => Ok(Command::TakeIngredient(ingr)),
            // ingr => Err(Error::new(
            //     ErrorKind::Other,
            //     format!("Can't get {:?} from pantry", ingr),
            // ))
            other => Err(error(format!("Can't get {:?} from pantry", other))),
        }
    }
}
impl Location for Pantry {}

#[derive(PartialEq, Debug, Clone)]
struct PrepArea {}
impl PrepArea {
    // pub fn stir(&mut self) -> Result<Command, Error> {
    //     Ok(Command::Stir)
    // }
}
impl Location for PrepArea {}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum GrabbableIngredient {
    Eggs,
    Butter,
}
impl Obtainable for GrabbableIngredient {}
impl Grabbable for GrabbableIngredient {
    fn grab(&self) -> Result<Command, Error> {
        Ok(Command::GrabIngredient(self.clone()))
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum ScoopableIngredient {
    Milk,
    Flour,
    Cocoa,
    Sugar,
}

impl Obtainable for ScoopableIngredient {}
impl Scoopable for ScoopableIngredient {
    fn scoop(&self) -> Result<Command, Error> {
        Ok(Command::ScoopIngredient(self.clone()))
    }
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
impl Obtainable for Ingredient {}

// TODO: I have Ingredients and Grabbable/Scoopable ingredients and the reporduce the items.  This isn't ideal.

type InventoryItem = (Ingredient, u16);
type Inventory = HashMap<Ingredient, u16>;

// #[derive(PartialEq, Debug)]
// struct Robot {
//     // location: &'static dyn Location,
//     inventory: Inventory,
// }

#[derive(PartialEq, Debug, Clone)]
struct RobotAt<L: Location> {
    inventory: Inventory,
    phantom: PhantomData<L>,
}
impl<L: Location> RobotAt<L> {
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
            Err(error(format!("Can't get {:?} from pantry", ing)))
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
    pub fn to_fridge(self) -> Result<RobotAt<Fridge>, Error> {
        Ok(self.into())
    }
    pub fn to_pantry(self) -> Result<RobotAt<Pantry>, Error> {
        Ok(self.into())
    }
    pub fn unload(mut self, ing: Ingredient) -> Result<RobotAt<PrepArea>, Error> {
        let _ = self.remove_from_inventory(ing); // TODO: unload to something
        Ok(self)
    }
    pub fn stir(self) -> Result<RobotAt<PrepArea>, Error> {
        Ok(self)
    }
    pub fn grab<G: Grabbable>(self, grabbable: G) -> Result<RobotWith<PrepArea, G>, Error> {
        let cmd = &grabbable.grab()?;
        // self.log(cmd); // TODO
        Ok(RobotWith::new(self, grabbable))
    }
    pub fn scoop<S: Scoopable>(self, scoopable: S) -> Result<RobotWith<PrepArea, S>, Error> {
        let cmd = &scoopable.scoop()?;
        // self.log(cmd); // TODO
        Ok(RobotWith::new(self, scoopable))
    }
}
// impl Location for RobotAt<PrepArea> {}
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
    pub fn to_prep_area(self) -> Result<RobotAt<PrepArea>, Error> {
        Ok(self.into())
    }
    pub fn to_pantry(self) -> Result<RobotAt<Pantry>, Error> {
        Ok(self.into())
    }
}
// impl Location for RobotAt<Fridge> {}

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
    pub fn to_prep_area(self) -> Result<RobotAt<PrepArea>, Error> {
        Ok(self.into())
    }
    pub fn to_fridge(self) -> Result<RobotAt<Fridge>, Error> {
        Ok(self.into())
    }

    pub fn load(mut self, ing: Ingredient) -> Result<RobotAt<Pantry>, Error> {
        self.add_to_inventory(ing)?;
        Ok(self)
    }
}
// impl Location for RobotAt<Pantry> {}

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
    let robot = robot.to_pantry().unwrap();
    assert_eq!(0, robot.inventory_count(Ingredient::Butter));

    let robot = robot.load(Ingredient::Butter).unwrap();
    // let cmd = robot.
    assert_eq!(1, robot.inventory_count(Ingredient::Butter));
    // assert_eq!(Command::TakeIngredient(Ingredient::Butter), cmd);
    // cmd =
    // to the prep area
    let robot = robot.to_prep_area().unwrap();
    let robot = robot.unload(Ingredient::Butter).unwrap();
    assert_eq!(0, robot.inventory_count(Ingredient::Butter));
    // assert_eq!(
    //     Command::RemoveIngredient(Ingredient::Butter),
    //     robot.commands().last()
    // );

    // TODO: this fails because this version hasn't implemented the inventory counts
    // assert!(robot.clone().unload(Ingredient::Butter).is_err());
    assert_eq!(0, robot.inventory_count(Ingredient::Butter));

    let robot = robot.stir().unwrap();
    // assert_eq!(Command::Stir, cmd);
}

#[test]
fn test_robot_at2() {
    let robot = <RobotAt<PrepArea>>::new();
    let robot = robot.to_pantry().unwrap();
    let robot = robot.load(Ingredient::Butter).unwrap();
    let robot = robot.load(Ingredient::Eggs).unwrap();
    let robot = robot.to_prep_area().unwrap();

    let robot = robot.unload(Ingredient::Eggs).unwrap();
    let robot = robot.unload(Ingredient::Butter).unwrap();
    let robot = robot.stir().unwrap();
}
#[test]
fn test_fluent_scenario() -> Result<(), Error> {
    fluent_scenario()
}

fn fluent_scenario() -> Result<(), Error> {
    use Ingredient::*;

    let robot = <RobotAt<PrepArea>>::new();
    let res = robot
        .to_pantry()?
        .load(Butter)?
        .to_prep_area()?
        .unload(Butter)?
        .scoop(ScoopableIngredient::Milk)? // todo, this is an error state because there is no milk at the location.
        .unscoop()?
        .to_pantry()?;
    Ok(())
}

#[derive(PartialEq, Debug)]
struct RobotWith<L: Location, O: Obtainable> {
    robot_at: RobotAt<L>,
    item: O,
}
impl<L: Location, O: Obtainable> RobotWith<L, O> {
    fn new(robot_at: RobotAt<L>, item: O) -> RobotWith<L, O> {
        RobotWith { robot_at, item }
    }
    fn unscoop(self) -> Result<RobotAt<L>, Error> {
        Ok(self.robot_at)
    }
}
