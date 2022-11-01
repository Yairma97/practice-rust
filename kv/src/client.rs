mod pb;

use prost::Message;

use pb::{Request,RequestGet};

fn main() {
    let request = RequestGet{
        key: "hello world".to_string(),
    };
    let mut buf = Vec::new();
    request.encode(&mut buf).unwrap();
    println!("encoded:{:?}", buf);
}
