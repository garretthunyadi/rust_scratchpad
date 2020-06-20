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

enum Command {
    Mix,
    Bake(Quantity),
    Cool,
}

fn recipe_for(good: BakedGood) -> Recipe {
    use BakedGood::*;
    use Command::*;
    use Ingredient::*;
    match good {
        Cake => Recipe {
            ingredients: vec![(Flour, 2), (Eggs, 2), (Milk, 1), (Sugar, 1)],
            steps: vec![Mix, Bake(25), Cool],
        },
        Cookies => Recipe {
            ingredients: vec![(Eggs, 1), (Flour, 1), (Sugar, 1), (Butter, 1)],
            steps: vec![Mix, Bake(30), Cool],
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
            steps: vec![Mix, Bake(35), Cool],
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
}
impl Robot {
    pub fn new() -> Robot {
        Robot {
            location: Location::PrepArea,
            holding: None,
            inventory: Inventory::new(),
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
        let rack_ids = HashSet::new();
        for (item, quantity) in order.clone().items {
            let racks = self.prepare_baked_good(item, quantity).unwrap();
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
            self.perform(cmd);
        }

        // put it on a cooling rack
        Ok(format!("rack{}", 123))
    }

    fn fetch_ingredient(
        &self,
        ingr: Ingredient,
        quantity: Quantity,
    ) -> Result<Vec<Ingredient>, Error> {
        let mut bag = vec![];
        for _ in 0..quantity {
            // todo: go fetch the ingredent
            bag.push(ingr.clone());
        }
        Ok(bag)
    }

    fn perform(&mut self, cmd: Command) {
        use Command::*;
        match cmd {
            Mix => self.mix().unwrap(),
            Bake(mins) => self.bake_pan(mins).unwrap(),
            Cool => self.cool_pan().unwrap(),
        }
    }

    //
    // Fetching commands
    //

    pub fn go_to(&mut self, loc: Location) -> Result<Location, Error> {
        self.location = loc.clone();
        Ok(loc)
    }
    pub fn load(&mut self, ing: Ingredient) -> Result<Ingredient, Error> {
        use Ingredient::*;
        use Location::*;
        match (&self.location, ing.clone()) {
            (Fridge, Eggs) => Ok(ing),
            (Fridge, Milk) => Ok(ing),
            (Fridge, Butter) => Ok(ing),
            (Pantry, Flour) => Ok(ing),
            (Pantry, Cocoa) => Ok(ing),
            (Pantry, Sugar) => Ok(ing),
            (_, _) => Err(Error::new(
                ErrorKind::Other,
                format!("Invalid location/item combo: {:?}/{:?}", self.location, ing),
            )),
        }
    }

    pub fn unload(&mut self, ing: Ingredient) -> Result<Ingredient, Error> {
        // not sure if this needs more validation.
        Ok(ing)
    }

    //
    // Baking commands
    //

    pub fn grab(&mut self, ing: Ingredient) -> Result<Ingredient, Error> {
        match ing.clone() {
            Ingredient::Eggs => Ok(ing),
            Ingredient::Butter => Ok(ing),
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("Can't grab {:?}", ing),
            )),
        }
    }
    pub fn squeeze(&mut self) -> Result<(), Error> {
        // not sure if this needs more validation.
        Ok(())
    }

    pub fn release(&mut self) -> Result<(), Error> {
        // not sure if this needs more validation.
        Ok(())
    }
    pub fn scoop(&mut self, ing: Ingredient) -> Result<Ingredient, Error> {
        match ing.clone() {
            Ingredient::Milk => Ok(ing),
            Ingredient::Flour => Ok(ing),
            Ingredient::Cocoa => Ok(ing),
            Ingredient::Sugar => Ok(ing),
            _ => Err(Error::new(
                ErrorKind::Other,
                format!("Can't scoop {:?}", ing),
            )),
        }
    }
    pub fn add_to_bowl(&mut self) -> Result<(), Error> {
        // not sure if this needs more validation.
        Ok(())
    }
    pub fn mix(&mut self) -> Result<(), Error> {
        // not sure if this needs more validation.
        Ok(())
    }
    pub fn pour_in_pan(&mut self) -> Result<(), Error> {
        // not sure if this needs more validation.
        Ok(())
    }
    pub fn bake_pan(&mut self, minutes: u16) -> Result<(), Error> {
        // not sure if this needs more validation.
        Ok(())
    }
    pub fn cool_pan(&mut self) -> Result<(), Error> {
        // not sure if this needs more validation.
        Ok(())
    }

    // private
    fn add_to_inventory(&mut self, ing: Ingredient) -> Result<(), Error> {
        let count = self.inventory.entry(ing).or_insert(0);
        *count += 1;
        Ok(())
    }
    fn inventory_count(&self, ing: Ingredient) -> u16 {
        *self.inventory.get(&ing).or(Some(&0)).unwrap()
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
}
