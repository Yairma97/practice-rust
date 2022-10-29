use std::{fmt::Display, fs};

use anyhow::{anyhow, Ok, Result};
use headless_chrome::{
    protocol::page::ScreenshotFormat,
    Browser, LaunchOptionsBuilder,
};

#[allow(dead_code)]
pub fn url2image(url: &str) -> Result<Vec<u8>> {

    fn to_anyhow(e: impl Display) -> anyhow::Error {
        anyhow!(e.to_string())
    }

    let browser =
        Browser::new(LaunchOptionsBuilder::default().build().unwrap()).map_err(to_anyhow)?;
    let tab = browser.wait_for_initial_tab().map_err(to_anyhow)?;
    let viewport = tab
        .navigate_to(url)
        .map_err(to_anyhow)?
        .wait_for_element("body")
        .map_err(to_anyhow)?
        .get_box_model()
        .map_err(to_anyhow)?
        .margin_viewport();
    let png_data = tab
        .capture_screenshot(ScreenshotFormat::PNG, Some(viewport), true)
        .map_err(to_anyhow)?;
    Ok(png_data)
}

#[allow(dead_code)]
pub fn web2image(url: &str, output: &str) -> Result<()> {
    let png_data = url2image(url)?;
    fs::write(&output, &png_data)?;
    Ok(())
}
