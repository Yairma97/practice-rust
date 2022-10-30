use std::{fmt::Display};

use anyhow::{anyhow, Ok, Result};
use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};
use image::{DynamicImage, Luma, load_from_memory, imageops::overlay, ImageFormat};
use qrcode::QrCode;

#[allow(dead_code)]
pub fn url2image(url: &str) -> Result<DynamicImage> {
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
        Ok(load_from_memory(&png_data)?)
}
#[allow(dead_code)]
fn gen_qrcode(url: &str) -> Result<DynamicImage> {
    let code = QrCode::new(url.as_bytes())?;
    let buf = code.render::<Luma<u8>>().build();
    Ok(DynamicImage::ImageLuma8(buf))
}
#[allow(dead_code)]
fn do_overlay(bottom:&mut DynamicImage,top:&DynamicImage){
    overlay(bottom, top, 0, 0)
}
#[allow(dead_code)]
pub fn web2image(url: &str, output: &str,format:ImageFormat) -> Result<()> {
    let mut bottom = url2image(url)?;
    let qrcode = gen_qrcode(url)?;
    do_overlay(&mut bottom, &qrcode);
    bottom.save_with_format(output, format)?;
    Ok(())
}
