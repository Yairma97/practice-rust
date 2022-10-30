use std::{path::Path, ffi::OsStr};

use clap::Parser;
use image::ImageFormat;
use url::Url;
mod web2image;
use web2image::web2image;
#[derive(Parser,Debug)]
#[command(name = "practice")]
#[command(author = "Yair. M <ma672832006@gmail.com>")]
#[command(version = "1.0")]
struct Cli {
    #[arg(short,long,default_value = "resource/snapshot.jpg",value_parser = valid_filename)]
    output: String,
    #[arg(long,value_parser = valid_url)]
    url: String,
}

fn get_img_format(path: &Path) -> Option<ImageFormat>{
    path.extension()
    .and_then(|path| OsStr::to_str(path))
    .and_then(|ext|{
        let ext = ext.to_lowercase();
        match ext.as_str() {
            "jpg"|"jepg" => Some(ImageFormat::Jpeg),
            "png" =>Some(ImageFormat::Png),
            _ => None
        }
    })
}
fn valid_filename(name : &str) -> Result<String,String> {
    let path = Path::new(name);
    let parent = path.parent().and_then(|path| path.is_dir().then(||path));
    let ext = get_img_format(path);
    if parent.is_none() || ext.is_none(){
        return Err(format!("Invalid filename: {}",name));
    }
    Ok(name.to_string())
}
fn valid_url(url: &str ) -> Result<String, String>{
    match Url::parse(url){
        Ok(_) => Ok(url.to_string()),
        Err(_) => Err(format!("Invalid url: {}",url))
    }
}

fn main() {
    let cli:Cli = Cli::parse();

    println!("{:?}", cli);

    let format = get_img_format(Path::new(&cli.output)).unwrap();

    web2image(&cli.url,&cli.output,format).unwrap();
}
