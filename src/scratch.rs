use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    // println!("here scratch");
    // let thing1 = Thing { id: 100 };
    // println!("{}", thing1.speak());
    // thing1.listen_to(&thing1);
    // let thing2 = Thing { id: 200 };
    // thing1.listen_to(&thing2);
    // thing2.listen_to(&thing1);
    // thing2.listen_to(&thing2);

    // let thing3: Thing = thing2.generate();
    // println!("Thing 3 id is {}", thing3.id);
    // let bizaro1: BizarroThing = thing1.generate();
    // println!("Bizarro 1 id is {}", bizaro1.id);

    // let baby: Thing = thing1.give_birth();
    // println!("Thing 1 gave birth to {}", baby.id);
    assert!(browse_wikipedia().is_ok());
}

use headless_chrome::{protocol::page::ScreenshotFormat, Browser};
fn browse_wikipedia() -> Result<(), failure::Error> {
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("https://www.wikipedia.org")?;
    tab.wait_for_element("input#searchInput")?.click()?;
    tab.type_str("WebKit")?.press_key("Enter")?;
    tab.wait_for_element("#firstHeading")?;
    assert!(tab.get_url().ends_with("WebKit"));

    let jpeg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;

    let mut pos = 0;
    let mut buffer = File::create("tmp.jpeg")?;

    while pos < jpeg_data.len() {
        let bytes_written = buffer.write(&jpeg_data[pos..])?;
        pos += bytes_written;
    }

    let _png_data = tab
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    Ok(())
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
