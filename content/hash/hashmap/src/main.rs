mod hashmap;

use hashmap::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let mut hm = HashMap::<&str, i32>::new(20);

    hm.push("hello", 10).unwrap();
    hm.push("bye", 12).unwrap();

    println!("{:#?}", hm);

    println!("hello: {:#?}", hm.get("hello").unwrap());
    println!("bye: {:#?}", hm.get("bye").unwrap());

    Ok(())
}

