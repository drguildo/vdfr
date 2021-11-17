use std::{fs, io::BufReader};

use vdfr::{AppInfo, PackageInfo};

fn main() {
    let appinfo = read_appinfo();
    for (id, app) in appinfo.apps {
        println!("{:?}", app.get(&["appinfo", "common", "library_assets"]));
    }
}

fn read_appinfo() -> AppInfo {
    let mut appinfo_file =
        BufReader::new(fs::File::open("appinfo.vdf").expect("Failed to read appinfo.vdf"));
    let appinfo = vdfr::AppInfo::load(&mut appinfo_file);
    return appinfo.unwrap();
}

fn read_packageinfo() -> PackageInfo {
    let mut packageinfo_file =
        BufReader::new(fs::File::open("packageinfo.vdf").expect("Failed to read packageinfo.vdf"));
    let packageinfo = vdfr::PackageInfo::load(&mut packageinfo_file);
    return packageinfo.unwrap();
}
