use std::{fs, io::BufReader};

fn main() {
    read_appinfo();
}

fn read_appinfo() {
    let mut appinfo_file =
        BufReader::new(fs::File::open("appinfo.vdf").expect("Failed to read appinfo.vdf"));
    let appinfo = vdfr::AppInfo::load(&mut appinfo_file);
    for (app_id, _) in &appinfo.unwrap().apps {
        println!("{}", app_id);
    }
}

fn read_packageinfo() {
    let mut packageinfo_file =
        BufReader::new(fs::File::open("packageinfo.vdf").expect("Failed to read packageinfo.vdf"));
    let packageinfo = vdfr::PackageInfo::load(&mut packageinfo_file);
    for (package_id, _) in &packageinfo.unwrap().packages {
        println!("{}", package_id);
    }
}
