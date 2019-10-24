pub fn main() -> std::io::Result<()> {
    println!("states");
    let _ = main1();
    println!("=================");

    let _ = main2();
    Ok(())
}

pub fn main1() -> std::io::Result<()> {
    println!("states1");
    Ok(())
}

struct State1 {}
impl State1 {}

fn start() -> State1 {
    State1 {}
}

impl State1 {
    // consumes self
    fn next(self) -> State2 {
        State2 {}
    }

    fn report(&self) {
        println!("S1");
    }
}

struct State2 {}
impl State2 {
    fn s1(self) -> State1 {
        State1 {}
    }
    fn end(self) {}
    fn report(&self) {
        println!("S2");
    }
}

fn transition() {
    let s1 = start();
    s1.report();

    let s2 = s1.next();
    s2.report();
    // s1.report();
    s2.end();
    // s2.report();
}
#[test]
fn test_transition() {
    transition()
}

// ===================================================

/*
    Builder pattern allows going from various builder states to other builder states,
    Accumulating data used for the building of an artifact at the teminal state

    * -> Builer1

    Builer1 -> Builer1a
    Builer1 -> Builer1b

    Builer1a -> Builer1b
    Builer1a -> Builer2
    Builer1b -> Builer2
    // note that bulder1b doesn't transition back to builder1a

    // The only way to get to an artifact is through Builder2
    Builder2 -> Artifact

*/
struct Builder1 {}
impl Builder1 {
    fn new() -> Builder1 {
        Builder1 {}
    }
    // consumes current state (self) and returns an owned next state
    fn into_builder1a(self) -> Builder1a {
        Builder1a {}
    }
    // Clippy: methods called `to_*` usually take self by reference; consider choosing a less ambiguous name
    // -> _to assumes that you are not consuming, _into assumes consumption.
    // fn to_builder1a(self) -> Builder1a {
    //     Builder1a {}
    // }

    fn into_builder1b(self) -> Builder1b {
        Builder1b {}
    }
}

struct Builder1a {}
impl Builder1a {
    // Equiv to Builder1::into_buider1a
    // We won't use this, I'm including just just
    // to show it's validity in this model.
    fn from(b: Builder1) -> Builder1a {
        Builder1a {}
    }
    fn into_builder1b(self) -> Builder1b {
        Builder1b {}
    }
    fn into_builder2(self) -> Builder2 {
        Builder2 {}
    }
}
struct Builder1b {}
impl Builder1b {
    fn into_builder2(self) -> Builder2 {
        Builder2 {}
    }
}

struct Builder2 {}
impl Builder2 {
    fn into_artifact(self) -> Artifact {
        Artifact {}
    }
}

struct Artifact {}

/*
  Now only the "correct" ways are possible *at compile time*.
  We cannot successfully compile incorrect code.
*/
pub fn main2() -> std::io::Result<()> {
    println!("states2: builder pattern");

    // ok
    let artifact = Builder1::new()
        .into_builder1a()
        .into_builder1b()
        .into_builder2()
        .into_artifact();

    // cannot compile
    // Builder2::new().into_artifact();
    // Builder1::new().into_builder().into_builder1b();
    // Builder1::new().into_builder1b().into_builder1b();

    // ok
    let artifact = Builder1::new()
        .into_builder1a()
        .into_builder1b()
        .into_builder2()
        .into_artifact();

    Ok(())
}

/*
    Let's consider this a moment. We've eliminated a whole class of runtime errors by stricturing
    the code like this.
*/
#[test]
fn test_main2() {
    let _ = main2();
}
