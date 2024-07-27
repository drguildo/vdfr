use std::{fs, io::BufReader};

use clap::{value_parser, Arg, Command};
use vdfr::{AppInfo, PackageInfo};

fn main() {
    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(
            Command::new("app")
                .arg(Arg::new("path").long("path").default_value("appinfo.vdf"))
                .arg(
                    Arg::new("id")
                        .long("id")
                        .value_parser(value_parser!(String)),
                )
                .arg(Arg::new("keys").long("keys").value_delimiter(',')),
        )
        .subcommand(
            Command::new("pkg")
                .arg(
                    Arg::new("path")
                        .long("path")
                        .default_value("packageinfo.vdf"),
                )
                .arg(
                    Arg::new("id")
                        .long("id")
                        .value_parser(value_parser!(String)),
                )
                .arg(Arg::new("keys").long("keys").value_delimiter(',')),
        )
        .arg_required_else_help(true)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("app") {
        let path = matches.get_one::<String>("path").unwrap();
        let appinfo = read_appinfo(path);
        if let Some(id) = matches.get_one::<String>("id") {
            let id: u32 = id.parse().expect("Failed to convert ID to u32");
            let app = appinfo.apps.get(&id);
            if let Some(app) = app {
                if let Some(values) = matches.get_many::<String>("keys") {
                    let keys: Vec<&str> = values.map(|s| s.as_str()).collect();
                    println!("{:?}", app.get(keys.as_slice()));
                } else {
                    println!("{:?}", app);
                }
            } else {
                eprintln!("Failed to find app with ID {}", id);
            }
        } else if let Some(keys) = matches.get_many::<String>("keys") {
            let keys: Vec<&str> = keys.map(|s| s.as_str()).collect();
            for (id, app) in appinfo.apps {
                println!("{}: {:?}", id, app.get(&keys));
            }
        } else {
            for (id, app) in appinfo.apps {
                println!("{}: {:?}", id, app);
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("pkg") {
        let path = matches.get_one::<String>("path").unwrap();
        let packageinfo = read_packageinfo(path);
        if let Some(id) = matches.get_one::<String>("id") {
            let id: u32 = id.parse().expect("Failed to convert ID to u32");
            let package = packageinfo.packages.get(&id);
            if let Some(package) = package {
                if let Some(values) = matches.get_many::<String>("keys") {
                    let keys: Vec<&str> = values.map(|s| s.as_str()).collect();
                    println!("{:?}", package.get(keys.as_slice()));
                } else {
                    println!("{:?}", package);
                }
            } else {
                eprintln!("Failed to find package with ID {}", id);
            }
        } else if let Some(keys) = matches.get_many::<String>("keys") {
            let keys: Vec<&str> = keys.map(|s| s.as_str()).collect();
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

fn read_appinfo(path: &str) -> AppInfo {
    let appinfo_file = fs::File::open(path).unwrap_or_else(|_| panic!("Failed to read {}", path));
    let mut appinfo_reader = BufReader::new(appinfo_file);
    let appinfo = vdfr::AppInfo::read(&mut appinfo_reader);
    appinfo.unwrap()
}

fn read_packageinfo(path: &str) -> PackageInfo {
    let packageinfo_file =
        fs::File::open(path).unwrap_or_else(|_| panic!("Failed to read {}", path));
    let mut packageinfo_reader = BufReader::new(packageinfo_file);
    let packageinfo = vdfr::PackageInfo::read(&mut packageinfo_reader);
    packageinfo.unwrap()
}
