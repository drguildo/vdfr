use std::{fs, io::Cursor};

fn main() {
    let appinfo_bytes = fs::read("appinfo.vdf").expect("Failed to read appinfo.vdf");
    let mut cursor = Cursor::new(appinfo_bytes);
    let appinfo = vdfr::AppInfo::load(&mut cursor);
    for (app_id, _) in &appinfo.apps {
        println!("{}", app_id);
    }
}
