use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    println!("headless_screenshot");
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
