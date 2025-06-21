use slint;
use i_slint_backend_winit::{winit::{event::WindowEvent, window::Window}, WinitWindowAccessor, WinitWindowEventResult}; // import the trait

slint::include_modules!();

use std::env;


fn main() {

    slint::platform::set_platform(Box::new(i_slint_backend_winit::Backend::new().unwrap())).unwrap();
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if let Some(first_arg) = args.get(1) {
        let first_str = first_arg.as_str();
        match first_str {
            "demo" => println!("We demoin'"),
            "hasher" =>  {
                let d = HasherDemo::new().unwrap();
                let w = d.window();
                w.on_winit_window_event(|win, ev| {
                    println!("We're getting an event! {:?}", ev);
                    match ev {
                        WindowEvent::DroppedFile(pb) => {
                            println!("We've got file: {:?}", pb);
                        },
                        WindowEvent::HoveredFile(pb) => {
                            println!("We've being tempted with file: {:?}", pb);
                        }
                        _ => {},
                    }
                    WinitWindowEventResult::Propagate
                });
                d.run().unwrap();

            },
            _ => panic!("never shoulda come here")
        }
    } else {
        MainLayoutDemo::new().unwrap().run().unwrap();
    }
    println!("Hello, world!");
}
