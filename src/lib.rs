use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use byteorder::{LittleEndian, ReadBytesExt};

const BIN_NONE: u8 = b'\x00';
const BIN_STRING: u8 = b'\x01';
const BIN_INT32: u8 = b'\x02';
const BIN_FLOAT32: u8 = b'\x03';
const BIN_POINTER: u8 = b'\x04';
const BIN_WIDESTRING: u8 = b'\x05';
const BIN_COLOR: u8 = b'\x06';
const BIN_UINT64: u8 = b'\x07';
const BIN_END: u8 = b'\x08';
const BIN_INT64: u8 = b'\x0A';
const BIN_END_ALT: u8 = b'\x0B';

#[derive(Debug)]
pub enum Value {
    StringType(String),
    WideStringType(String),
    Int32Type(i32),
    PointerType(i32),
    ColorType(i32),
    UInt64Type(u64),
    Int64Type(i64),
    Float32Type(f32),
    KeyValueType(KeyValue),
}

type KeyValue = HashMap<String, Value>;

pub struct App {
    pub size: u32,
    pub state: u32,
    pub last_update: u32,
    pub access_token: u64,
    pub checksum: [u8; 20],
    pub change_number: u32,
    pub key_values: KeyValue,
}

pub struct AppInfo {
    pub version: u32,
    pub universe: u32,
    pub apps: HashMap<u32, App>,
}

impl AppInfo {
    pub fn load<R: std::io::Read>(reader: &mut R) -> Result<AppInfo, Error> {
        let version = reader.read_u32::<LittleEndian>()?;
        let universe = reader.read_u32::<LittleEndian>()?;

        let mut appinfo = AppInfo {
            universe,
            version,
            apps: HashMap::new(),
        };

        loop {
            let app_id = reader.read_u32::<LittleEndian>()?;
            if app_id == 0 {
                break;
            }

            let size = reader.read_u32::<LittleEndian>()?;
            let state = reader.read_u32::<LittleEndian>()?;
            let last_update = reader.read_u32::<LittleEndian>()?;
            let access_token = reader.read_u64::<LittleEndian>()?;

            let mut checksum: [u8; 20] = [0; 20];
            reader.read_exact(&mut checksum)?;

            let change_number = reader.read_u32::<LittleEndian>()?;

            let key_values = binary_loads(reader, false)?;

            let app = App {
                size,
                state,
                last_update,
                access_token,
                checksum,
                change_number,
                key_values,
            };
            appinfo.apps.insert(app_id, app);
        }

        Ok(appinfo)
    }
}

pub struct Package {
    pub checksum: [u8; 20],
    pub change_number: u32,
    pub pics: u64,
    pub key_values: KeyValue,
}

pub struct PackageInfo {
    pub version: u32,
    pub universe: u32,
    pub packages: HashMap<u32, Package>,
}

impl PackageInfo {
    pub fn load<R: std::io::Read>(reader: &mut R) -> Result<PackageInfo, Error> {
        let version = reader.read_u32::<LittleEndian>()?;
        let universe = reader.read_u32::<LittleEndian>()?;
        println!("version: {:#x} universe: {}", version, universe);

        let mut packageinfo = PackageInfo {
            version,
            universe,
            packages: HashMap::new(),
        };

        loop {
            let package_id = reader.read_u32::<LittleEndian>()?;

            if package_id == 0xffffffff {
                break;
            }

            let mut checksum: [u8; 20] = [0; 20];
            reader.read_exact(&mut checksum)?;

            let change_number = reader.read_u32::<LittleEndian>()?;

            // XXX: No idea what this is. Seems to get ignored in vdf.py.
            let pics = reader.read_u64::<LittleEndian>()?;

            let key_values = binary_loads(reader, false)?;

            let package = Package {
                checksum,
                change_number,
                pics,
                key_values,
            };

            packageinfo.packages.insert(package_id, package);
        }

        Ok(packageinfo)
    }
}

fn binary_loads<R: std::io::Read>(reader: &mut R, alt_format: bool) -> Result<KeyValue, Error> {
    let current_bin_end = if alt_format { BIN_END_ALT } else { BIN_END };

    let mut node = KeyValue::new();

    loop {
        let t = reader.read_u8()?;
        if t == current_bin_end {
            return Ok(node);
        }

        let key = read_string(reader, false)?;

        if t == BIN_NONE {
            let subnode = binary_loads(reader, alt_format)?;
            node.insert(key, Value::KeyValueType(subnode));
        } else if t == BIN_STRING {
            let s = read_string(reader, false)?;
            node.insert(key, Value::StringType(s));
        } else if t == BIN_WIDESTRING {
            let s = read_string(reader, true)?;
            node.insert(key, Value::WideStringType(s));
        } else if [BIN_INT32, BIN_POINTER, BIN_COLOR].contains(&t) {
            let val = reader.read_i32::<LittleEndian>()?;
            if t == BIN_INT32 {
                node.insert(key, Value::Int32Type(val));
            } else if t == BIN_POINTER {
                node.insert(key, Value::PointerType(val));
            } else if t == BIN_COLOR {
                node.insert(key, Value::ColorType(val));
            }
        } else if t == BIN_UINT64 {
            let val = reader.read_u64::<LittleEndian>()?;
            node.insert(key, Value::UInt64Type(val));
        } else if t == BIN_INT64 {
            let val = reader.read_i64::<LittleEndian>()?;
            node.insert(key, Value::Int64Type(val));
        } else if t == BIN_FLOAT32 {
            let val = reader.read_f32::<LittleEndian>()?;
            node.insert(key, Value::Float32Type(val));
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Invalid type: 0x{:X}", t),
            ));
        }
    }
}

fn read_string<R: std::io::Read>(reader: &mut R, wide: bool) -> Result<String, Error> {
    if wide {
        let mut buf: Vec<u16> = vec![];
        loop {
            // Maybe this should be big-endian?
            let c = reader.read_u16::<LittleEndian>()?;
            if c == 0 {
                break;
            }
            buf.push(c);
        }
        return Ok(std::string::String::from_utf16_lossy(&buf).to_string());
    } else {
        let mut buf: Vec<u8> = vec![];
        loop {
            let c = reader.read_u8()?;
            if c == 0 {
                break;
            }
            buf.push(c);
        }
        return Ok(std::string::String::from_utf8_lossy(&buf).to_string());
    }
}
