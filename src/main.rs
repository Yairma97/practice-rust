use std::{path::Path, ffi::OsStr};

use clap::Parser;

#[derive(Parser,Debug)]
#[command(name = "practice")]
#[command(author = "Yair. M <ma672832006@gmail.com>")]
#[command(version = "1.0")]
struct Cli {
    #[arg(short,long,default_value = "resource/snapshot.jpg",value_parser = valid_filename)]
    output: String,
    #[arg(long)]
    url: String,
}

fn get_file_ext(path: &Path) -> Option<String>{
    path.extension()
    .and_then(|path| OsStr::to_str(path))
    .and_then(|ext|{
        let ext = ext.to_lowercase();
        match ext.as_str() {
            "jpg"|"jepg"|"png" => Some(ext),
            _ => None
        }
    })
}
fn valid_filename(name : &str) -> Result<(),String> {
    let path = Path::new(name);
    let parent = path.parent().and_then(|path| path.is_dir().then(||path));
    let ext = get_file_ext(path);
    if parent.is_none() || ext.is_none(){
        return Err(format!("Invalid filename: {}",name));
    }
    Ok(())
}
fn main() {
    let cli:Cli = Cli::parse();

    println!("{:?}", cli.output);
}
