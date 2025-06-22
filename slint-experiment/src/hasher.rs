use i_slint_backend_winit::{winit::{event::WindowEvent, window::Window}, WinitWindowAccessor, WinitWindowEventResult}; // import the trait
use slint;
use slint::Model;
use slint::ComponentHandle;

use crate::HasherDemo;

pub fn start_hasher() {
    let d = HasherDemo::new().unwrap();
    let w = d.window();
    let d_c = d.clone_strong();
    w.on_winit_window_event(move |win, ev| {
        println!("We're getting an event! {:?}", ev);
        match ev {
            WindowEvent::DroppedFile(pb) => {
                println!("We've got file: {:?}", pb);
                let mut s = slint::SharedString::new();
                s.push_str(pb.to_str().unwrap());
                d_c.set_file_path(s);
            }
            WindowEvent::HoveredFile(pb) => {
                println!("We've being tempted with file: {:?}", pb);
            }
            _ => {}
        }
        WinitWindowEventResult::Propagate
    });
    d.run().unwrap();
}
