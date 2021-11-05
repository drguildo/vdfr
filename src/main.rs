use std::{fs, io::Cursor};

fn main() {
    let mut packageinfo_file = std::fs::File::open("packageinfo.vdf").expect("Failed to read packageinfo.vdf");
    vdfr::packageinfo_loads(&mut packageinfo_file);
}

fn read_appinfo() {
    let appinfo_bytes = fs::read("appinfo.vdf").expect("Failed to read appinfo.vdf");
    let mut cursor = Cursor::new(appinfo_bytes);
    let appinfo = vdfr::AppInfo::load(&mut cursor);
    for (app_id, _) in &appinfo.apps {
        println!("{}", app_id);
    }
}
