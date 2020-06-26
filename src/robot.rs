/*
    Robot Chef, based on Eric Normand's Clojure exercise.

    [] How can I remodel the robot's location and the scoop state model to make error states unrepresentatable?

    [] Add scoop state
    [] Add in the tabletop as a storage location that sources the ingrediants while preping
    [x] Grab :scoop
    [/] Turn procedural loops into functional expressions
    [-] Macro for the recipe? This could expand the composite command into
       regular commands.
    [] After it works, refactor to be more functional and less OO.
    [] Test that error results are as expected.

    [] As a next level, add multiple robots, and operate them on threads, working with channels and coorperating to access the fridge and pantry.
*/
#![warn(dead_code)]
#![warn(unused_variables)]
#![warn(unused_macros)]

use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};

macro_rules! err {
    ($($arg:tt)*) => {
        Err(Error::new(ErrorKind::Other, format!($($arg)*)))
    };
}

// ($($arg:tt)*) => {{
//     let res = $crate::fmt::format($crate::__export::format_args!($($arg)*));
//     res
// }}

pub fn main() -> Result<(), Error> {
    puts!("Robot");
    test_orders();
    Ok(())
}

#[derive(PartialEq, Debug, Clone)]
enum BakedGood {
    Cake,
    Cookies,
    Brownies,
}

struct Recipe {
    ingredients: Vec<InventoryItem>,
    steps: Vec<Command>,
}

#[derive(PartialEq, Debug, Clone)]
enum Command {
    // Baking Commands
    Grab(Ingredient),
    Squeeze,
    Release,
    Scoop(Ingredient),
    AddToBowl,
    Mix,
    PourIntoPan,
    Bake(Quantity),
    CoolPan,

    // Added
    GrabScoop,

    // Composite Commands
    AddIngredientsToBowl(Vec<InventoryItem>),

    // Fetching Commands
    Goto(Location),
    LoadUp(Ingredient),
    Unload(Ingredient),
}

fn recipe_for(good: BakedGood) -> Recipe {
    use BakedGood::*;
    use Command::*;
    use Ingredient::*;
    match good {
        Cake => {
            let ingredients = vec![(Flour, 2), (Eggs, 2), (Milk, 1), (Sugar, 1)];
            Recipe {
                ingredients: ingredients.clone(),
                steps: vec![AddIngredientsToBowl(ingredients), Mix, Bake(25), CoolPan],
            }
        }
        Cookies => {
            let ingredients = vec![(Eggs, 1), (Flour, 1), (Sugar, 1), (Butter, 1)];
            Recipe {
                ingredients: ingredients.clone(),
                steps: vec![AddIngredientsToBowl(ingredients), Mix, Bake(30), CoolPan],
            }
        }
        Brownies => {
            let ingredients1 = vec![(Butter, 2), (Sugar, 1), (Cocoa, 2)];
            let ingredients2 = vec![(Flour, 2), (Eggs, 2), (Milk, 1)];

            Recipe {
                ingredients: [&ingredients1[..], &ingredients2[..]].concat(),
                steps: vec![
                    AddIngredientsToBowl(ingredients1),
                    Mix,
                    AddIngredientsToBowl(ingredients2),
                    Mix,
                    Bake(35),
                    CoolPan,
                ],
            }
        }
    }
}

type OrderId = usize;
type Quantity = u16;

#[derive(PartialEq, Debug, Clone)]
pub struct Order {
    id: OrderId,
    address: String,
    items: Vec<(BakedGood, Quantity)>,
}
impl Order {
    fn new(items: Vec<(BakedGood, Quantity)>) -> Order {
        Order {
            id: 111,
            address: String::from(""),
            items,
        }
    }
}

type RackId = String;

#[derive(PartialEq, Debug, Clone)]
pub struct Receipt {
    order_id: OrderId,
    address: String,
    rack_ids: Vec<RackId>,
}
impl Receipt {
    fn from(order: Order, rack_ids: Vec<RackId>) -> Receipt {
        Receipt {
            order_id: order.id,
            address: order.address,
            rack_ids,
        }
    }
}
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Location {
    Fridge,
    Pantry,
    PrepArea,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Ingredient {
    Eggs,
    Butter,
    Milk,
    Flour,
    Cocoa,
    Sugar,
}

type InventoryItem = (Ingredient, u16);
type Inventory = HashMap<Ingredient, u16>;

fn location_for(ingr: &Ingredient) -> Location {
    use Ingredient::*;
    match ingr {
        Flour | Cocoa | Sugar => Location::Pantry,
        Eggs | Milk | Butter => Location::Fridge,
    }
}

//
// Filling Orders
//
pub fn get_morning_orders() -> Result<Vec<Order>, Error> {
    use BakedGood::*;
    Ok(vec![
        Order::new(vec![(Cake, 3), (Cookies, 2)]),
        Order::new(vec![(Cookies, 6)]),
        Order::new(vec![(Brownies, 3)]),
    ])
}

// Take ownership of the order and return a receipt.
pub fn delivery_receipt(order: Order, rack_ids: Vec<RackId>) -> Result<Receipt, Error> {
    Ok(Receipt::from(order, rack_ids))
}

#[derive(PartialEq, Debug, Clone)]
enum Holdable {
    Scoop(Option<Ingredient>),
    Ingredient(Ingredient),
}

#[derive(PartialEq, Debug)]
struct Robot {
    location: Location,
    holding: Option<Holdable>,
    inventory: Inventory,
    log: Vec<Command>,
}
impl Robot {
    pub fn new() -> Robot {
        Robot {
            location: Location::PrepArea,
            holding: None,
            inventory: Inventory::new(),
            log: vec![],
        }
    }

    //
    // Prepare orders
    //
    fn prepare_orders(&mut self, orders: Vec<Order>) -> Result<Vec<Receipt>, Error> {
        // let mut receipts = vec![];
        // for order in orders {
        //     // todo: deal with one-vs-all failures
        //     receipts.push(self.prepare_order(order).unwrap())
        // }
        Ok(orders
            .into_iter()
            .flat_map(|order| self.prepare_order(order))
            .collect())
    }

    fn prepare_order(&mut self, order: Order) -> Result<Receipt, Error> {
        // let mut rack_ids = HashSet::new();
        // for (item, quantity) in order.clone().items {
        //     let ids = self.prepare_baked_good(item, quantity).unwrap();
        //     for id in ids {
        //         rack_ids.insert(id);
        //     }
        // }

        let rack_ids = order
            .items
            .iter()
            .flat_map(|(item, quantity)| self.prepare_baked_good(item, quantity))
            .flat_map(|o| o)
            .collect::<HashSet<_>>() // uniq
            .into_iter()
            .collect::<Vec<_>>();

        Ok(Receipt::from(order, rack_ids.clone()))
    }

    fn prepare_baked_good(
        &mut self,
        good: &BakedGood,
        quantity: &Quantity,
    ) -> Result<Vec<RackId>, Error> {
        // let mut rack_ids = HashSet::new();
        // println!("\npreparing {} {:?}", quantity, good);
        // for i in 0..quantity.clone() {
        //     rack_ids.insert(self.prepare_one(good.clone()).unwrap());
        // }
        let rack_ids = (0..*quantity)
            .into_iter()
            .flat_map(|_| self.prepare_one(good.clone()))
            // .inspect(|x| println!(" *--* {:?} ", x))
            .collect();
        println!(" *--* {:?} ", rack_ids);
        Ok(rack_ids)
    }

    // not pure as this produces a random rack id
    fn prepare_one(&mut self, good: BakedGood) -> Result<RackId, Error> {
        // print!(".");
        let recipe = recipe_for(good);
        // let mut tabletop = vec![];
        // // get stuff
        // for (ingr, quantity) in recipe.ingredients {
        //     // println!("Need: {} {:?}", quantity, ingr);
        //     let mut items = self.fetch_ingredient(ingr, quantity).unwrap();
        //     tabletop.append(&mut items);
        // }

        let mut tabletop = recipe
            .ingredients
            .iter()
            .flat_map(|(ingr, quantity)| self.fetch_ingredient(ingr, quantity)) // flatmap removes the errors
            .flatten() // [[Ingr1,Ingr2],[Ingr1]] -> [Ingr1,Ingr2,Ingr1]
            .collect::<Vec<Ingredient>>();

        assert_eq!(
            self.location,
            Location::PrepArea,
            "Dev error: Robot is not at the prep area when starting to prepare"
        );

        println!(" /--\\ {:?}", tabletop);
        // make the thing
        // for cmd in recipe.steps {
        //     let mut cmds = self.perform(cmd)?;
        //     self.log.append(&mut cmds);
        // }
        let mut cmds = recipe
            .steps
            .into_iter()
            .flat_map(|cmd| self.perform(cmd, &mut tabletop))
            .flatten()
            .collect();
        self.log.append(&mut cmds);

        // put it on a cooling rack
        Ok(format!(
            "rack{}",
            [123, 34, 377, 5453, 233, 555, 112, 000, 001, 002, 003, 004] // TODO: impl "real" cooling racks
                .choose(&mut rand::thread_rng())
                .unwrap()
        ))
    }

    fn fetch_ingredient(
        &mut self,
        ingr: &Ingredient,
        quantity: &Quantity,
    ) -> Result<Vec<Ingredient>, Error> {
        // move to location for ingredient
        let loc = location_for(&ingr);

        let cmd = self.go_to(loc)?;
        self.log.push(cmd);
        let mut tray = vec![];

        for _ in 0..*quantity {
            let cmd = self.load_up(&ingr)?;
            self.log.push(cmd);
            tray.push(ingr.clone());
        }

        let cmd = self.go_to(Location::PrepArea)?;
        self.log.push(cmd);

        // for ingr in bag.clone() {
        //     // todo: go fetch the ingredent
        //     let cmd = self.unload(ingr)?;
        //     self.log.push(cmd);
        // }
        let mut cmds = tray
            .clone()
            .into_iter()
            .flat_map(|ingr| self.unload(ingr))
            .collect::<Vec<_>>();
        self.log.append(&mut cmds);

        Ok(tray) // TODO: the contents are unloaded (and not in the robot's inventory), so I'm not sure it makes sense to return the collection.
    }

    fn perform(
        &mut self,
        cmd: Command,
        source: &mut Vec<Ingredient>,
    ) -> Result<Vec<Command>, Error> {
        fn cmd_as_vec(cmd: Command) -> Result<Vec<Command>, Error> {
            Ok(vec![cmd])
        }
        use Command::*;
        Ok(match cmd {
            Grab(ingr) => cmd_as_vec(self.grab(&ingr, source)?),
            Squeeze => cmd_as_vec(self.squeeze()?),
            Release => cmd_as_vec(self.release()?),
            Scoop(ingr) => cmd_as_vec(self.scoop(&ingr, source)?),
            AddToBowl => cmd_as_vec(self.add_to_bowl()?),
            Mix => cmd_as_vec(self.mix()?),
            PourIntoPan => cmd_as_vec(self.pour_into_pan()?),
            Bake(mins) => cmd_as_vec(self.bake_pan(mins)?),
            CoolPan => cmd_as_vec(self.cool_pan()?),
            GrabScoop => cmd_as_vec(self.grab_scoop()?),
            AddIngredientsToBowl(inventory_item) => self.add_ingredients_to_bowl(inventory_item),
            Goto(loc) => cmd_as_vec(self.go_to(loc)?),
            LoadUp(ingr) => cmd_as_vec(self.load_up(&ingr)?),
            Unload(ingr) => cmd_as_vec(self.unload(ingr)?),
        }?)
    }

    //
    // Fetching commands
    //

    pub fn go_to(&mut self, loc: Location) -> Result<Command, Error> {
        self.location = loc.clone();
        Ok(Command::Goto(loc))
    }
    pub fn load_up(&mut self, ingr: &Ingredient) -> Result<Command, Error> {
        use Ingredient::*;
        use Location::*;
        match (&self.location, ingr.clone()) {
            (Fridge, Eggs)
            | (Fridge, Milk)
            | (Fridge, Butter)
            | (Pantry, Flour)
            | (Pantry, Cocoa)
            | (Pantry, Sugar) => Ok(Command::LoadUp(ingr.clone())),
            (_, _) => err!(
                "Invalid location/item combo: {:?}/{:?}",
                self.location,
                ingr
            ),
        }
    }

    pub fn unload(&mut self, ingr: Ingredient) -> Result<Command, Error> {
        if self.location == Location::PrepArea {
            Ok(Command::Unload(ingr))
        } else {
            err!("Cannot unload away from : {:?}/{:?}", self.location, ingr)
        }
    }

    //
    // Composite Commands
    //
    fn add_ingredients_to_bowl(
        &mut self,
        inventory_items: Vec<InventoryItem>,
    ) -> Result<Vec<Command>, Error> {
        let mut all_cmds = vec![];
        for (ingr, cnt) in inventory_items {
            for _ in 0..cnt {
                let mut cmds = self.add_ingredient_to_bowl(ingr.clone())?;
                all_cmds.append(&mut cmds); // extend copies, append moves
            }
        }
        Ok(all_cmds)

        // let res = inventory_items
        //     .iter()
        //     .map(|(ingr, cnt)| {
        //         (0..*cnt)
        //             .into_iter()
        //             .flat_map(|_| self.add_ingredient_to_bowl(ingr.clone()))
        //             .flatten()
        //             .collect()
        //     })
        //     .collect();

        // Ok(res)
    }

    fn add_ingredient_to_bowl(&mut self, ingr: Ingredient) -> Result<Vec<Command>, Error> {
        use Command::*;
        match &ingr {
            Ingredient::Butter => Ok(vec![Grab(ingr), AddToBowl]),
            Ingredient::Eggs => Ok(vec![Grab(ingr), Squeeze, AddToBowl]),
            Ingredient::Flour | Ingredient::Milk | Ingredient::Cocoa | Ingredient::Sugar => {
                Ok(vec![GrabScoop, Scoop(ingr), AddToBowl, Release])
            }
        }
    }

    //
    // Baking commands
    //

    pub fn grab(
        &mut self,
        ingr: &Ingredient,
        source: &mut Vec<Ingredient>,
    ) -> Result<Command, Error> {
        use Ingredient::*;
        match ingr.clone() {
            Eggs | Butter => {
                if let Some(ingr) = remove_item(source, ingr) {
                    self.holding = Some(Holdable::Ingredient(ingr.clone()));
                    Ok(Command::Grab(ingr.clone()))
                } else {
                    err!("Can't grab {:?} because there is none nearby", ingr)
                }
            }
            _ => err!("Can't grab {:?}", ingr),
        }
    }
    pub fn squeeze(&mut self) -> Result<Command, Error> {
        // not sure if this needs more validation.
        Ok(Command::Squeeze)
    }

    pub fn release(&mut self) -> Result<Command, Error> {
        // not sure if this needs more validation.
        Ok(Command::Release)
    }
    pub fn scoop(
        &mut self,
        ingr: &Ingredient,
        source: &mut Vec<Ingredient>,
    ) -> Result<Command, Error> {
        use Ingredient::*;

        match self.holding.clone() {
            Some(Holdable::Scoop(None)) => match ingr {
                Milk | Flour | Cocoa | Sugar => {
                    if let Some(ingr) = remove_item(source, ingr) {
                        self.holding = Some(Holdable::Scoop(Some(ingr.clone())));
                        Ok(Command::Scoop(ingr.clone()))
                    } else {
                        err!("Can't scoop {:?} because there is none nearby", ingr)
                    }
                }
                _ => err!("Can't scoop {:?}", ingr),
            },
            None => {
                return err!("Can't scoop {:?} because I'm not holding my scoop", ingr);
            }
            Some(Holdable::Ingredient(held_ingr)) => {
                return err!("Can't scoop {:?} because I'm holding {:?}", ingr, held_ingr);
            }
            Some(Holdable::Scoop(Some(ingr_in_scoop))) => {
                return err!(
                    "Can't scoop {:?} because my scoop has some {:?}",
                    ingr,
                    ingr_in_scoop
                );
            }
        }
    }
    pub fn add_to_bowl(&mut self) -> Result<Command, Error> {
        match self.holding {
            None => {
                return err!("Can't add to bowl because I'm not holding anything");
            }
            Some(_) => Ok(Command::AddToBowl),
        }
    }
    pub fn mix(&mut self) -> Result<Command, Error> {
        // TODO: check that there is something in the bowl.
        Ok(Command::Mix)
    }
    pub fn pour_into_pan(&mut self) -> Result<Command, Error> {
        // TODO: check that there is something in the bowl.
        Ok(Command::PourIntoPan)
    }
    pub fn bake_pan(&mut self, minutes: u16) -> Result<Command, Error> {
        // TODO: check that there is something in the pan.
        Ok(Command::Bake(minutes))
    }
    pub fn cool_pan(&mut self) -> Result<Command, Error> {
        // TODO: must have a pan (newly baked item).
        Ok(Command::CoolPan)
    }

    pub fn grab_scoop(&mut self) -> Result<Command, Error> {
        match self.holding.clone() {
            None => self.holding = Some(Holdable::Scoop(None)),
            Some(x) => return err!("Can't grab scoop because I'm holding {:?}", x),
        }
        Ok(Command::GrabScoop)
    }

    // private
    fn add_to_inventory(&mut self, ingr: Ingredient) -> Result<(), Error> {
        let count = self.inventory.entry(ingr).or_insert(0);
        *count += 1;
        Ok(())
    }
    fn inventory_count(&self, ingr: Ingredient) -> u16 {
        *self.inventory.get(&ingr).or(Some(&0)).unwrap()
    }
}

#[test]
fn test_robot() {
    let mut robot = Robot::new();
    assert_eq!(0, robot.inventory_count(Ingredient::Butter));
    robot.add_to_inventory(Ingredient::Butter).unwrap();
    assert_eq!(1, robot.inventory_count(Ingredient::Butter));
    robot.add_to_inventory(Ingredient::Butter).unwrap();
    assert_eq!(2, robot.inventory_count(Ingredient::Butter));
}

pub fn remove_item<V, T>(vec: &mut Vec<T>, item: &V) -> Option<T>
where
    T: PartialEq<V>,
{
    vec.iter().position(|n| n == item).map(|i| vec.remove(i))
}

#[test]
fn test_grab() {
    use Ingredient::*;
    // pub fn grab(
    //     &mut self,
    //     ingr: &Ingredient,
    //     source: &mut Vec<Ingredient>,
    // ) -> Result<Command, Error> {

    let mut tabletop = vec![Butter, Eggs, Eggs, Sugar];
    let mut robot = Robot::new();
    assert!(robot.holding.is_none());
    assert!(robot.grab(&Butter, &mut tabletop).is_ok());
    assert_eq!(vec![Eggs, Eggs, Sugar], tabletop);
    assert_eq!(robot.holding, Some(Holdable::Ingredient(Butter)));

    // can't grab sugar
    assert!(robot.grab(&Sugar, &mut tabletop).is_err());

    // can't grab what's not there.
    let mut tabletop = vec![];
    assert!(robot.grab(&Butter, &mut tabletop).is_err());
}

#[test]
fn test_scoop() {
    use Ingredient::*;
    let mut tabletop = vec![Eggs, Milk, Milk];
    let mut robot = Robot::new();
    assert!(robot.holding.is_none());
    assert!(robot.scoop(&Milk, &mut tabletop).is_err()); // not holding a scoop
    robot.grab_scoop().unwrap();
    assert!(robot.scoop(&Milk, &mut tabletop).is_ok());
    assert_eq!(vec![Eggs, Milk], tabletop);
    assert_eq!(robot.holding, Some(Holdable::Scoop(Some(Milk))));

    // can't scoop eggs
    assert!(robot.scoop(&Eggs, &mut tabletop).is_err());
}

#[test]
fn test_remove_item() {
    use Ingredient::*;

    let mut v = vec![Butter, Butter, Eggs];
    let res = remove_item(&mut v, &Eggs);
    assert_eq!(Some(Eggs), res);
    assert_eq!(vec![Butter, Butter], v);

    let res = remove_item(&mut v, &Milk);
    assert_eq!(None, res);
    assert_eq!(vec![Butter, Butter], v);

    let res = remove_item(&mut v, &Butter);
    assert_eq!(Some(Butter), res);
    assert_eq!(vec![Butter], v);

    let res = remove_item(&mut v, &Butter);
    assert_eq!(Some(Butter), res);
    assert!(v.is_empty());

    let res = remove_item(&mut v, &Butter);
    assert_eq!(None, res);
    assert!(v.is_empty());

    let mut v = vec![Butter, Butter, Eggs];
    let res = remove_item(&mut v, &Butter);
    assert_eq!(Some(Butter), res);
    assert_eq!(vec![Butter, Eggs], v);
}

// #[test]
fn test_orders() {
    let mut robot = Robot::new();
    let orders = get_morning_orders().unwrap();
    let receipts = robot.prepare_orders(orders).unwrap();
    println!("\nRECEIPTS: {:?}", receipts);
    println!("\nLOG: {:?}", robot.log);
}
