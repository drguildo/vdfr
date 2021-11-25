use std::{fs, io::BufReader};

use clap::{App, Arg, SubCommand};
use vdfr::{AppInfo, PackageInfo};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(
            SubCommand::with_name("app")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("appinfo.vdf"),
                )
                .arg(Arg::with_name("id").long("id").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("pkg")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("packageinfo.vdf"),
                )
                .arg(Arg::with_name("id").long("id").takes_value(true)),
        )
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("app") {
        let path = matches.value_of("path").unwrap();
        let appinfo = read_appinfo(path);
    }

    if let Some(matches) = matches.subcommand_matches("pkg") {
        let path = matches.value_of("path").unwrap();
        let packageinfo = read_packageinfo(path);
    }
}

fn read_appinfo(path: &str) -> AppInfo {
    let mut appinfo_file =
        BufReader::new(fs::File::open(path).expect(&format!("Failed to read {}", path)));
    let appinfo = vdfr::AppInfo::load(&mut appinfo_file);
    return appinfo.unwrap();
}

fn read_packageinfo(path: &str) -> PackageInfo {
    let mut packageinfo_file =
        BufReader::new(fs::File::open(path).expect(&format!("Failed to read {}", path)));
    let packageinfo = vdfr::PackageInfo::load(&mut packageinfo_file);
    return packageinfo.unwrap();
}
