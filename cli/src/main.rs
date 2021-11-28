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
                .arg(Arg::with_name("id").long("id").takes_value(true))
                .arg(
                    Arg::with_name("keys")
                        .long("keys")
                        .multiple(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("pkg")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("packageinfo.vdf"),
                )
                .arg(Arg::with_name("id").long("id").takes_value(true))
                .arg(
                    Arg::with_name("keys")
                        .long("keys")
                        .multiple(true)
                        .takes_value(true),
                ),
        )
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("app") {
        let path = matches.value_of("path").unwrap();
        let appinfo = read_appinfo(path);
        if let Some(id) = matches.value_of("id") {
            let id: u32 = id.parse().expect("Failed to convert ID to u32");
            let app = appinfo.apps.get(&id);
            if let Some(app) = app {
                if let Some(keys) = matches.values_of("keys") {
                    let keys: Vec<&str> = keys.collect();
                    println!("{:?}", app.get(&keys));
                } else {
                    println!("{:?}", app);
                }
            } else {
                eprintln!("Failed to find app with ID {}", id);
            }
        } else {
            if let Some(keys) = matches.values_of("keys") {
                let keys: Vec<&str> = keys.collect();
                for (id, app) in appinfo.apps {
                    println!("{}: {:?}", id, app.get(&keys));
                }
            } else {
                for (id, app) in appinfo.apps {
                    println!("{}: {:?}", id, app);
                }
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("pkg") {
        let path = matches.value_of("path").unwrap();
        let packageinfo = read_packageinfo(path);
        if let Some(id) = matches.value_of("id") {
            let id: u32 = id.parse().expect("Failed to convert ID to u32");
            let package = packageinfo.packages.get(&id);
            if let Some(package) = package {
                if let Some(keys) = matches.values_of("keys") {
                    let keys: Vec<&str> = keys.collect();
                    println!("{:?}", package.get(&keys));
                } else {
                    println!("{:?}", package);
                }
            } else {
                eprintln!("Failed to find package with ID {}", id);
            }
        } else {
            if let Some(keys) = matches.values_of("keys") {
                let keys: Vec<&str> = keys.collect();
                for (id, package) in packageinfo.packages {
                    println!("{}: {:?}", id, package.get(&keys));
                }
            } else {
                for (id, package) in packageinfo.packages {
                    println!("{}: {:?}", id, package);
                }
            }
        }
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
