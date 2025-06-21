
fn main() {
    // This only seems to recognize one compiled file at a time
    slint_build::compile("ui/compiled.slint").unwrap();
}
