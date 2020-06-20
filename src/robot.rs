use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};

fn error(msg: &str) -> Result<(), Error> {
    Err(Error::new(ErrorKind::Other, msg))
}

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
        Cake => Recipe {
            ingredients: vec![(Flour, 2), (Eggs, 2), (Milk, 1), (Sugar, 1)],
            steps: vec![Mix, Bake(25), CoolPan],
        },
        Cookies => Recipe {
            ingredients: vec![(Eggs, 1), (Flour, 1), (Sugar, 1), (Butter, 1)],
            steps: vec![Mix, Bake(30), CoolPan],
        },
        Brownies => Recipe {
            ingredients: vec![
                (Flour, 2),
                (Eggs, 2),
                (Sugar, 1),
                (Cocoa, 2),
                (Milk, 1),
                (Butter, 2),
            ],
            steps: vec![Mix, Bake(35), CoolPan],
        },
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

#[derive(PartialEq, Debug)]
struct Robot {
    location: Location,
    holding: Option<Ingredient>,
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
        let mut receipts = vec![];
        for order in orders {
            // todo: deal with one-vs-all failures
            receipts.push(self.prepare_order(order).unwrap())
        }
        Ok(receipts)
    }

    fn prepare_order(&mut self, order: Order) -> Result<Receipt, Error> {
        let mut rack_ids = HashSet::new();
        for (item, quantity) in order.clone().items {
            let ids = self.prepare_baked_good(item, quantity).unwrap();
            for id in ids {
                rack_ids.insert(id);
            }
        }
        Ok(Receipt::from(order, rack_ids.into_iter().collect()))
    }

    fn prepare_baked_good(
        &mut self,
        good: BakedGood,
        quantity: Quantity,
    ) -> Result<Vec<RackId>, Error> {
        let mut rack_ids = HashSet::new();
        println!("\npreparing {} {:?}", quantity, good);
        for i in 0..quantity {
            rack_ids.insert(self.prepare_one(good.clone()).unwrap());
        }
        Ok(rack_ids.into_iter().collect())
    }

    fn prepare_one(&mut self, good: BakedGood) -> Result<RackId, Error> {
        // print!(".");
        let recipe = recipe_for(good);
        let mut tabletop = vec![];
        // get stuff
        for (ingr, quantity) in recipe.ingredients {
            // println!("Need: {} {:?}", quantity, ingr);
            let mut items = self.fetch_ingredient(ingr, quantity).unwrap();
            tabletop.append(&mut items);
        }

        println!("{:?}", tabletop);
        // make the thing
        for cmd in recipe.steps {
            let cmd = self.perform(cmd)?;
            self.log.push(cmd);
        }

        // put it on a cooling rack
        Ok(format!("rack{}", 123))
    }

    fn fetch_ingredient(
        &mut self,
        ingr: Ingredient,
        quantity: Quantity,
    ) -> Result<Vec<Ingredient>, Error> {
        // move to location for ingredient
        let loc = location_for(&ingr);

        let cmd = self.go_to(loc)?;
        self.log.push(cmd);
        let mut bag = vec![];

        for _ in 0..quantity {
            let cmd = self.load_up(&ingr)?;
            self.log.push(cmd);
            bag.push(ingr.clone());
        }

        let cmd = self.go_to(Location::PrepArea)?;
        self.log.push(cmd);

        for ingr in bag.clone() {
            // todo: go fetch the ingredent
            let cmd = self.unload(ingr)?;
            self.log.push(cmd);
        }
        Ok(bag) // TODO: the contents are unloaded, so I'm not sure it makes sense to return the collection.
    }

    fn perform(&mut self, cmd: Command) -> Result<Command, Error> {
        use Command::*;
        Ok(match cmd {
            Grab(ingr) => self.grab(ingr),
            Squeeze => self.squeeze(),
            Release => self.release(),
            Scoop(ingr) => self.scoop(&ingr),
            AddToBowl => self.add_to_bowl(),
            Mix => self.mix(),
            PourIntoPan => self.pour_into_pan(),
            Bake(mins) => self.bake_pan(mins),
            CoolPan => self.cool_pan(),
            Goto(loc) => self.go_to(loc),
            LoadUp(ingr) => self.load_up(&ingr),
            Unload(ingr) => self.unload(ingr),
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
            (_, _) => Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Invalid location/item combo: {:?}/{:?}",
                    self.location, ingr
                ),
            )),
        }
    }

    pub fn unload(&mut self, ingr: Ingredient) -> Result<Command, Error> {
        if self.location == Location::PrepArea {
            Ok(Command::Unload(ingr))
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("Cannot unload away from : {:?}/{:?}", self.location, ingr),
            ))
        }
    }

    //
    // Baking commands
    //

    pub fn grab(&mut self, ingr: Ingredient) -> Result<Command, Error> {
        match ingr.clone() {
            Ingredient::Eggs => Ok(Command::Grab(ingr)),
            Ingredient::Butter => Ok(Command::Grab(ingr)),
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("Can't grab {:?}", ingr),
            )),
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
    pub fn scoop(&mut self, ingr: &Ingredient) -> Result<Command, Error> {
        use Ingredient::*;
        match ingr {
            Milk | Flour | Cocoa | Sugar => Ok(Command::Scoop(ingr.clone())),
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("Can't scoop {:?}", ingr),
            )),
        }
    }
    pub fn add_to_bowl(&mut self) -> Result<Command, Error> {
        // not sure if this needs more validation.
        Ok(Command::AddToBowl)
    }
    pub fn mix(&mut self) -> Result<Command, Error> {
        // not sure if this needs more validation.
        Ok(Command::Mix)
    }
    pub fn pour_into_pan(&mut self) -> Result<Command, Error> {
        // not sure if this needs more validation.
        Ok(Command::PourIntoPan)
    }
    pub fn bake_pan(&mut self, minutes: u16) -> Result<Command, Error> {
        // not sure if this needs more validation.
        Ok(Command::Bake(minutes))
    }
    pub fn cool_pan(&mut self) -> Result<Command, Error> {
        // not sure if this needs more validation.
        Ok(Command::CoolPan)
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

// #[test]
fn test_orders() {
    let mut robot = Robot::new();
    let orders = get_morning_orders().unwrap();
    let receipts = robot.prepare_orders(orders).unwrap();
    println!("\nRECEIPTS: {:?}", receipts);
    println!("\nLOG: {:?}", robot.log);
}
