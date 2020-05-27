// #![feature(trace_macros, log_syntax)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_macros)]

#[macro_use]
extern crate specs_derive;

#[macro_use]
mod macros;

extern crate rand;

mod async_await;
mod async_await2;
mod bayes;
mod channels;
mod corpus;
mod cyoa;
mod docs;
mod domain_info_crate;
mod ecs;
mod grep;
mod hashing;
mod headless_screenshot;
mod iterator;
mod iterator_impl_1;
mod iterator_impl_2;
mod linked_list;
mod ll2;
mod ll_tx_log;
mod markov_chain;
mod markov_chain1;
mod markov_chain2;
mod markov_chain4;
mod mutex;
mod random;
mod rc2;
mod rc_arc;
mod rust_book;
mod scans3;
mod scans4;
mod scratch;
mod sliding_log;
mod states;
mod threads_0;
mod threads_1;
mod typed_bayes;
mod wappalyzer_crate;

use std::fmt;
use std::fs;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

macro_rules! s {
    ($val: expr) => {
        $val.to_string();
    };
}

// async fn async_mains() {
// this will block at each step, so not run things concurrently
// async_await::async_main().await;
// cyoa::main().await;

// let f2 = cyoa::main();
// let f1 = async_await::async_main();
// let f3 = cyoa::main();
// let f4 = async_await::async_main();

// futures::join!(f1, f2, f3, f4);
// }

// fn main() -> Result<(), std::io::Error> {
// async_await2::main();
// block_on(async_main());
// block_on(async_mains());
// cyoa::main();
// random::main();
// grep::main();
// mutex::main();
// states::main()?;
// channels::main()?;
// markov_chain4::main()?;
// rc_arc::main()?;
// rust_book::boxes::main()?;
// markov_chain::examples::bigram_markov_chain_example::main()?;
// markov_chain::examples::coin_bets::main()?;
// markov_chain::examples::main()?;
// markov_chain::main()?;
// markov_chain1::main()?;
// markov_chain2::main()?;
// threads_1::main()?;
// bayes::main()?;
// typed_bayes::main()?;
// iterator_impl_1::main()?;
// iterator_impl_2::main()?;
//     scans4::main()?; //.or_else(|err| Err(s!(err)));
//     Ok(())
// }

// fn main() -> Result<(), String> {
//     corpus::main().or_else(|err| Err(s!(err)))
// }
// fn main() -> Result<(), std::io::Error> {
//     hashing::main()
// }

// #[tokio::main]
// async fn main() {
//     // async_stream::main();
//     wappalyzer_crate::main().await;
//     // domain_info_crate::main();
// }

fn main() {
    // json_macro::main(); // not done
    // macros::main();
    // let _ = bayes::main();
    // scratch::main();
    // headless_screenshot::main();
    // ecs::main();
    // }
    // scratch::main();
    // main1();
    // traits1();
    // scans1();
    // locks1();
    // threads1();
    // channels1();
    // mutex1();
    // docs_main();
    // iter();
    // specific();
    // state_machine();
    // run_story();
    // generics();
    // polymorphism();
    // monomorphism();
    // scans2::scan_states();
    // scans3::main();
    // maybies();
    // default::main();
    // getters::main()?;
    // linked_list::main();
    // ll_tx_log::main();
    // sliding_log::main();
    // iterator::main();
    // match corpus::main() {
    //     Ok(()) => Ok(()),
    //     Err(err) => Err(err.to_string()),
    // }
    // corpus::main().or_else(|err| Err(err.to_string()))
    ll2::main().unwrap();
}

fn specific() {
    let mut state = ASpecificStateMachine::start();
    println!("===> {} <===", state);

    loop {
        state = state.next();
        println!("===> {} <===", state);
        if *state == ASpecificStateMachine::End {
            break;
        }
    }
}

// ==================================================================================
#[derive(PartialEq)]
enum ASpecificStateMachine {
    Start,
    Place1,
    Place2,
    End,
}
impl ASpecificStateMachine {
    fn start<'a>() -> &'a ASpecificStateMachine {
        &Self::Start
    }

    fn next<'a>(&self) -> &'a ASpecificStateMachine {
        match self {
            Self::Start => &Self::Place1,
            Self::Place1 => &Self::Place2,
            Self::Place2 => &Self::End,
            Self::End => &Self::End,
        }
    }
}
impl fmt::Display for ASpecificStateMachine {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let s = match self {
            Self::Start => "Starting...",
            Self::Place1 => "At a place...",
            Self::Place2 => "At another place...",
            Self::End => "Fin.",
        };
        write!(f, "{}", s)
    }
}
// ==================================================================================

#[derive(PartialEq)]
enum ASpecificSubplace {
    Entry,
    Room1,
    Room2,
    End,
}
impl ASpecificSubplace {
    fn start<'a>() -> &'a ASpecificSubplace {
        &Self::Entry
    }

    fn next<'a>(&self) -> &'a ASpecificSubplace {
        match self {
            Self::Entry => &Self::Room1,
            Self::Room1 => &Self::Room2,
            Self::Room2 => &Self::End,
            Self::End => &Self::End,
        }
    }
}
impl fmt::Display for ASpecificSubplace {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let s = match self {
            Self::Entry => "At entrence...",
            Self::Room1 => "In room 1...",
            Self::Room2 => "In room 2...",
            Self::End => "Fin.",
        };
        write!(f, "{}", s)
    }
}
// ==================================================================================

#[derive(Debug)]
struct Thing(u32);
fn thing(i: u32) -> Thing {
    Thing(i)
}
fn iter() {
    let ts = (1..10).map(thing);
    let mut filtered = ts.filter(|t| t.0 > 3).collect::<Vec<_>>();
    println!("{:?}", filtered);
    let things = vec![Thing(1), Thing(2)];
    let res = things.into_iter().map(|t| t.0).collect::<Vec<_>>();

    // mutations
    let f = &mut filtered[0];
    f.0 = 42;
    println!("{:?}", f);

    println!("{:?}", filtered);
    mutate_some_stuff(&mut filtered);
    println!("{:?}", filtered);
    // get a readable thing and copy it
    let things = (1..10).map(thing).collect::<Vec<_>>();
    // let boxes = ts.collect::<Box<_>>();
    let x: i32 = 5;
    if let Ok(x) = x.checked_add(5).ok_or("Overflow!") {
        println!("succ -> {}", x);
    }
    println!("ts {:?}", things);

    // let things3 = &things.into_iter().map(copy).collect();

    let things2 = things;
}

fn mutate_some_stuff(things: &mut Vec<Thing>) {
    things[3].0 = 33;
}

fn docs_main() {
    docs::foo();
}

fn mutex1() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // increment(&counter);
    // increment(&counter);
    // println!("{}", counter.lock().unwrap());
    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            increment(&counter);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let count = *counter.lock().unwrap();
    println!("{}", count);
}

fn increment(counter: &Mutex<i32>) {
    let mut i = counter.lock().unwrap();
    *i += 1;
}

fn threads1() {
    let mut handles = vec![];
    for i in 1..5 {
        let s = format!("Thing {}", i);
        handles.push(thread::spawn(move || say_hi(&s)));
    }
    // thread::sleep(Duration::from_millis(1));
    for handle in handles {
        handle.join().unwrap();
    }
}

fn say_hi(s: &str) {
    println!("hi # {}", s);
}

fn channels1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn main1() {
    // println!("Hello, world!");
    // for i in [1,2].iter() {
    //     println!("{}",i);
    // }

    // let doc = read(String::from("foo.txt"));
    let doc = read(String::from("/Users/ghunyadi/eraseme.txt"));
    let res = process(&doc);

    // println!("{}",doc.url);
    println!("{}", doc.filename);
    println!("{}", doc.content);

    println!("{}", res.filename);
    println!("{}", res.published);
    println!("{}", res.uses_wp);

    // simple loops
    let xs = [1, 2, 3];
    for x in xs.iter() {
        println!("{}", x);
    }

    // Iteration

    // From and Into
    let a = A {};
    println!("{:?}", a);
    println!("{:?}", B::from(&a));
    println!("{:?}", C::from(B::from(&a)));

    let x = Some(14);
    let y: Option<i32> = None;
    println!("{:?}", x);
    println!("{:?}", y);

    if let Some(z) = x {
        println!("{:?}", z)
    }

    let mut r = Resource::get();
    println!("{:?}", r);
    r.use_res();
    println!("{:?}", r);
    r.use_res();
    println!("{:?}", r);
    r.free();

    test_iterator();
}

#[derive(Debug)]
struct A;
#[derive(Debug)]
struct B;
#[derive(Debug)]
struct C;

#[derive(Debug)]
struct Resource {
    times_used: i32,
}

impl Resource {
    fn get() -> Resource {
        println!("getting resource");
        Resource { times_used: 0 }
    }
    fn use_res(&mut self) {
        self.times_used += 1;
        println!("using resource");
    }
    fn free(self) {
        println!("freeing resource");
    }
}

use std::convert::From;
impl From<&A> for B {
    fn from(_: &A) -> Self {
        B {}
    }
}
impl From<B> for C {
    fn from(_: B) -> Self {
        C {}
    }
}

#[derive(Debug)]
struct Document {
    // url: String,
    filename: String,
    content: String,
}

#[derive(Debug)]
struct DocResult {
    // url: String,
    filename: String,
    published: bool,
    uses_wp: bool,
}

// trait Summary {
//     fn summarize(&self) -> String;
// }

fn read(filename: String) -> Document {
    // let content = match url {
    //     f if f.starts_with("file://") =>  fs::read_to_string(url.split("//")).expect("Something went wrong reading the file"),
    //     _ => String::from(""),
    // };
    let content =
        fs::read_to_string(filename.clone()).expect("Something went wrong reading the file");
    // todo: fetch from S3
    Document { filename, content }
}

fn process(doc: &Document) -> DocResult {
    DocResult {
        filename: doc.filename.clone(),
        published: true,
        uses_wp: false,
    }
}

#[derive(Debug)]
struct By4 {
    curr: i32,
}

// write an iterator - that increments by 4
impl Iterator for By4 {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        self.curr += 4;
        Some(self.curr)
    }
}

fn test_iterator() {
    let mut a = By4 { curr: 1 };
    let n = a.next();
    println!("{:?} / {:?}", a, n);
    a.next();
    println!("{:?}", a);
}

// simulate dependent scans

// trait Scanner {
//     fn scan(domain: Domain) -> ScanResult;
// }
// // trait Summary {
// //     fn summarize(&self) -> String;
// // }

// trait ScanResultTrait {
//     fn domain() -> Domain;
// }

// impl ScanResultTrait for CoreScan {
//     // type Item = CoreScan;
//     fn domain() -> Domain {Domain{}}
// }

/////////////
///
///
///   Scan v1 - Enums
///
/// //////////

// fn scans1() {
//     let core_res = ScanResult {
//         domain: String::from("google.com"),
//         info: ScanInfo::Core {
//             uses_wordpress: true,
//         },
//     };
//     report(&core_res);

//     let mx_res = mx_scan(Domain::from("google.com"));
//     report(&mx_res);

//     let results = [&core_res, &mx_res];
//     report_multi(&results);

//     let xs = [core_res];
//     for x in xs.iter() {
//         println!("{:?}", x);
//     }
// }
// type Domain = String;
// type IP = String;

// #[derive(Debug)]
// enum HostPlatform {
//     EIGBluehost,
//     EIGHostgator,
//     Google,
//     AWS,
//     GoDaddy,
// }

// #[derive(Debug)]
// enum ScanInfo {
//     Core { uses_wordpress: bool },
//     Mx { ip: IP, host_platform: HostPlatform },
// }

// #[derive(Debug)]
// enum CoreScanError {}

// #[derive(Debug)]
// enum CoreResult {
//     Error { code: CoreScanError },
// }

// #[derive(Debug)]
// struct ScanResult {
//     domain: String,
//     info: ScanInfo,
// }

// fn mx_scan(domain: Domain) -> ScanResult {
//     let info = ScanInfo::Mx {
//         ip: String::from(""),
//         host_platform: HostPlatform::Google,
//     };
//     ScanResult { domain, info }
// }

// fn report(res: &ScanResult) {
//     println!("{}", res.domain);

//     match res.info {
//         ScanInfo::Core { .. } => println!("core"),
//         ScanInfo::Mx { .. } => println!("mx"),
//     };
// }

// fn report_multi(results: &[&ScanResult]) {
//     for res in results.iter() {
//         println!(" - {}", res.domain);
//     }
// }

trait Doer {
    fn r#do(&self) -> String {
        format!("Doing | {}", self.step1())
    }
    fn step1(&self) -> String;
}

#[derive(Debug)]
struct ImportantGuy {
    name: String,
}

impl Doer for ImportantGuy {
    fn step1(&self) -> String {
        let mut s = self.name.clone();
        s.push_str(" is doing step one.");
        s
        // self.name + "doing step1...".to_string()
    }
}

impl Doer for i32 {
    fn step1(&self) -> String {
        format!("doing step1 {} times", self)
    }
}

fn traits1() {
    let bob = ImportantGuy {
        name: "Bob".to_string(),
    };
    // println!("{:?}", bob);
    // println!("{:?}", bob.step1());
    // println!("{:?}", 4.step1());

    let four = 4;
    let doers: Vec<Box<dyn Doer>> = vec![Box::new(bob), Box::new(four)];
    for doer in doers {
        println!("{:?}", doer.r#do());
    }
}

struct Cabinet {
    locked: bool,
}

trait Lock
where
    Self: std::marker::Sized,
{
    fn lock(&mut self);
    fn unlock(&mut self);
    fn check(&self) -> bool;
    fn destroy(self) {}
}

impl Lock for Cabinet {
    fn lock(&mut self) {
        self.locked = true
    }
    fn unlock(&mut self) {
        self.locked = false;
    }
    fn check(&self) -> bool {
        self.locked
    }
    fn destroy(self) {}
}
fn locks1() {
    let mut lock = Cabinet { locked: false };
    println!("{}", lock.check());
    lock.lock();
    println!("{}", lock.check());
    lock.unlock();
    println!("{}", lock.check());
    lock.destroy();
}

//// ==============================================================
///
///
///      State Machine
///
///      https://hoverbear.org/2016/10/12/rust-state-machine-pattern/
///
///

fn state_machine() {
    // The `<StateA>` is implied here. We don't need to add type annotations!
    let in_state_a = StateMachine::new("Blah blah blah".into());

    // This is okay here. But later once we've changed state it won't work anymore.
    println!("Starting Value: {}", in_state_a.state.start_value);
    println!("   {}", in_state_a.some_unrelated_value);

    // Transition to the new state. This consumes the old state.
    // Here we need type annotations (since not all StateMachines are linear in their state).
    let in_state_b = StateMachine::<StateB>::from(in_state_a);

    // This doesn't work! The value is moved when we transition!
    // in_state_a.some_unrelated_value;
    // Instead, we can use the existing value.
    // println!("   {}", in_state_a.some_unrelated_value);

    println!("Interm Value: {:?}", in_state_b.state.interm_value);

    // And our final state.
    let in_state_c = StateMachine::<StateC>::from(in_state_b);

    // This doesn't work either! The state doesn't even contain this value.
    // in_state_c.state.start_value;

    println!("Final state: {}", in_state_c.state.final_value);
}

// Here is our pretty state machine.
struct StateMachine<S> {
    some_unrelated_value: usize,
    state: S,
}

// It starts, predictably, in `StateA`
impl StateMachine<StateA> {
    fn new(val: String) -> Self {
        StateMachine {
            some_unrelated_value: 0,
            state: StateA::new(val),
        }
    }
}

// State A starts the machine with a string.
struct StateA {
    start_value: String,
}
impl StateA {
    fn new(start_value: String) -> Self {
        StateA { start_value }
    }
}

// State B goes and breaks up that String into words.
struct StateB {
    interm_value: Vec<String>,
}
impl From<StateMachine<StateA>> for StateMachine<StateB> {
    fn from(val: StateMachine<StateA>) -> StateMachine<StateB> {
        StateMachine {
            some_unrelated_value: val.some_unrelated_value,
            state: StateB {
                interm_value: val.state.start_value.split(' ').map(|x| x.into()).collect(),
            },
        }
    }
}

// Finally, StateC gives us the length of the vector, or the word count.
struct StateC {
    final_value: usize,
}
impl From<StateMachine<StateB>> for StateMachine<StateC> {
    fn from(val: StateMachine<StateB>) -> StateMachine<StateC> {
        StateMachine {
            some_unrelated_value: val.some_unrelated_value,
            state: StateC {
                final_value: val.state.interm_value.len(),
            },
        }
    }
}

// ==================================
fn say(something: &str) {
    println!("{}", something);
}

trait Node<T> {
    fn next() -> Self;
}
fn run_story() {
    // let mut location = MyStory::start();
    // say(location.description);
    // loop {
    //     location = location.next();
    //     say(location.description);
    // }
}
struct MyStory<S> {
    description: &'static str,
    state: S,
}
// --
struct Location1 {
    description: String,
}
impl Location1 {
    fn at() -> Self {
        Location1 {
            description: String::from("At Location 1"),
        }
    }
}
impl MyStory<Location1> {
    fn start() -> Self {
        Self::at()
    }
    fn location1() -> Self {
        Self::at()
    }
    fn at() -> Self {
        MyStory {
            description: "At MyStory/Location1",
            state: Location1::at(),
        }
    }
    fn next(&self) -> MyStory<Location2> {
        MyStory {
            description: "At MyStory/Location2",
            state: Location2::at(),
        }
    }
}
// --
struct Location2 {
    description: String,
}
impl Location2 {
    fn location2() -> Self {
        Self::at()
    }

    fn at() -> Self {
        Location2 {
            description: String::from("At Location 2"),
        }
    }
}
impl MyStory<Location2> {
    fn at() -> Self {
        MyStory {
            description: "At MyStory/Location2",
            state: Location2::at(),
        }
    }
    fn next() -> MyStory<Location2> {
        Self::at()
    }
}

/// =====================
/// https://bluejekyll.github.io/blog/rust/2017/08/06/type-parameters.html
///

trait Animal {
    // default impl, don't most animals have 4 legs? j/k
    fn num_legs(&self) -> usize {
        4
    }
}

// Define a Dog and implement Animal for it
struct Dog;
impl Animal for Dog {
    // use the default impl
}

// Define a Chicken and implement Animal for it
struct Chicken;
impl Animal for Chicken {
    fn num_legs(&self) -> usize {
        2
    }
}

fn print_num_legs(animal: &dyn Animal) {
    println!("legs: {}", animal.num_legs());
}

fn polymorphism() {
    let dog = Dog;
    let chicken = Chicken;
    // Notice the cast to the Trait Object
    print_num_legs(&dog as &dyn Animal);
    print_num_legs(&dog); // Apparently the cast in the blog post is no longer necessary.
    print_num_legs(&chicken as &dyn Animal);
}

fn print_num_legs2<A: Animal>(animal: &A) {
    println!("legs: {}", animal.num_legs());
}

fn monomorphism() {
    let dog = Dog;
    let chicken = Chicken;
    // Notice no cast is necessary
    print_num_legs2(&dog);
    print_num_legs2(&chicken);
}

/// ==================================================
///
///  
//
fn generics() {
    let thing = AThing::<u8> { item: 9 };
    thing.talk(); // not defined as this is a different class/type
    let thing = AThing { item: 9 };
    let thing = AThing::<Rock> { item: Rock {} };
    thing.talk();
    let talker = &thing as &dyn Talker;
    talker.talk();

    let talker = &AThing { item: 9u8 } as &dyn Talker;
    talker.talk();
    let talker_box = talker.next();
    talker_box.talk();
    // etc...!
}
struct Rock {}
struct AThing<T> {
    item: T,
}
impl AThing<Rock> {
    fn talk(&self) {
        say("I'm a rock, I don't talk!");
    }
}
trait Talker {
    fn talk(&self);
    fn next(&self) -> Box<dyn Talker>;
}
// impl<T> Talker for AThing<T> {
//     fn next(&self) -> &dyn Talker {self}
// }
impl Talker for AThing<u8> {
    fn talk(&self) {
        say("repeating u8 times, blah blah...")
    }
    fn next(&self) -> Box<dyn Talker> {
        Box::new(AThing { item: Rock {} })
    }
}
impl Talker for AThing<Rock> {
    fn talk(&self) {
        // say("The rock says nothing.")
        self.talk();
    }
    fn next(&self) -> Box<dyn Talker> {
        Box::new(AThing { item: 5 })
    }
}

///
///
///   Maybe & Closures
///
fn maybies() {
    let f = |x: usize| x + 1;
    assert_eq!(f(1), 2);
    maybe::doit(f, 3);
}
mod maybe {
    use rand::Rng;
    pub fn doit<F>(f: F, i: usize) -> usize
    where
        F: FnOnce(usize) -> usize,
    {
        // let num = rand::thread_rng().gen_range(0, 100);
        if rand::thread_rng().gen() {
            let res = f(i);
            println!("DOING: {}", res);
            res
        } else {
            0
        }
    }
}

mod default {
    struct Num {
        x: usize,
    }
    impl Default for Num {
        fn default() -> Self {
            Num { x: 42 }
        }
    }
    pub fn main() {
        let n = Num::default();
        assert_eq!(n.x, 42);

        let ns = Nums::default();
        assert_eq!(ns.a, 0);
        assert_eq!(ns.b, 0);

        let ns = Nums {
            a: 1,
            ..Nums::default()
        };
        // let ns = Nums{a:1,..Default::default()}; // can also use Default::default
        assert_eq!(ns.a, 1);
        assert_eq!(ns.b, 0);
    }

    #[derive(Default)]
    struct Nums {
        a: usize,
        b: i32,
        c: f32,
    }
}

/// Have to know to use a getter version
mod getters {
    #[derive(Default)]
    struct X {
        a: u32,
        b: u32,
    }
    impl X {
        fn b(&self) -> u32 {
            555
        }
    }
    pub fn main() -> Result<(), &'static str> {
        let x = X {
            a: 1,
            ..Default::default()
        };
        assert_eq!(x.a, 1);
        assert_eq!(x.b, 0);
        assert_eq!(x.b(), 555);
        Ok(())
        // Err("Testing...")
    }
}
