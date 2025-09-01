use std::path::PathBuf;

use i_slint_backend_winit::{winit::{event::WindowEvent}, WinitWindowAccessor};
use crate::slint::winit_030::WinitWindowEventResult;
use slint;
use slint::ComponentHandle;
use anyhow::Result;

use crate::HasherDemo;

pub fn start_hasher() -> Result<()> {
    let d = HasherDemo::new().unwrap();
    let w = d.window();
    let d_c = d.clone_strong();
    w.on_winit_window_event(move |_win, ev| {
        match ev {
            WindowEvent::DroppedFile(pb) => {
                println!("We're getting an event! {:?}", ev);
                println!("We've got file: {:?}", pb);
                let mut s = slint::SharedString::new();
                s.push_str(pb.to_str().unwrap());
                d_c.set_file_path(s);
                let mut shash = slint::SharedString::new();
                let h = match do_hash(pb) {
                    Ok(s) => s,
                    Err(e) => e.to_string()
                };
                shash.push_str(h.as_str());
                d_c.set_file_hash(shash);
            }
            WindowEvent::HoveredFile(pb) => {
                println!("We've being tempted with file: {:?}", pb);
            }
            _ => {}
        }
        WinitWindowEventResult::Propagate
    });
    d.run().unwrap();
    Ok(())
}

pub fn do_hash(pb: &PathBuf) -> Result<String> {
                let mut hasher = blake3::Hasher::new();
                let hash = hasher.update_mmap_rayon(pb)?;
                Ok(hash.finalize().to_string())
}
