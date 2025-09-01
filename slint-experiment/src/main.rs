use anyhow::Result;
use slint;

slint::include_modules!();

use std::env;
mod hasher;
use hasher::start_hasher;

//use crate::hasher::start_hasher;
//mod hasher;

fn main() -> Result<()>{

    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap())).unwrap();
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if let Some(first_arg) = args.get(1) {
        let first_str = first_arg.as_str();
        match first_str {
            "demo" => println!("We demoin'"),
            "hasher" => start_hasher()?,
            "plistdemo" => PlistSelectorDemo::new().unwrap().run().unwrap(),
            _ => panic!("never shoulda come here")
        }
    } else {
        MainLayoutDemo::new().unwrap().run().unwrap();
    }
    println!("Hello, world!");
    Ok(())
}
