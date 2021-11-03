use std::{fs, io::Cursor};

fn main() {
    let appinfo_bytes = fs::read("appinfo.vdf").expect("Failed to read appinfo.vdf");
    let mut cursor = Cursor::new(appinfo_bytes);
    vdfr::appinfo_loads(&mut cursor);
}
