use std::collections::HashMap;

use structure::{
    byteorder::{LittleEndian, ReadBytesExt},
    structure, structure_impl,
};

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

enum Value {
    StringType(String),
    WideStringType(String),
    Int32Type(i32),
    PointerType(i32),
    ColorType(i32),
    UInt64Type(u64),
    Int64Type(i64),
    Float32Type(f32),
    KeyValueType(Node),
}

struct Node {
    data: HashMap<String, Value>,
}

pub fn appinfo_loads<R: std::io::Read>(reader: &mut R) {
    let universe = reader.read_u32::<LittleEndian>().unwrap();
    let version = reader.read_u32::<LittleEndian>().unwrap();
    println!("universe: 0x{:X}, version: {}", universe, version);

    let app_struct = structure!("<3IQ20sI");
    loop {
        let app_id = reader.read_u32::<LittleEndian>().unwrap();
        if app_id == 0 {
            println!("last appinfo application");
            break;
        }

        let (size, state, last_update, access_token, checksum, change_number) =
            app_struct.unpack_from(reader).unwrap();
        println!("size: {}, state: {}, last_update: {}, access_token: {}, checksum: {:?}, change_number: {}", size, state, last_update, access_token, checksum, change_number);

        binary_loads(reader, false);
    }
}

fn binary_loads<R: std::io::Read>(reader: &mut R, alt_format: bool) -> Node {
    let current_bin_end = if alt_format { BIN_END_ALT } else { BIN_END };

    let mut node = Node {
        data: HashMap::new(),
    };

    loop {
        let t = reader.read_u8().unwrap();
        if t == current_bin_end {
            println!("BIN_END");
            return node;
        }

        let key = read_string(reader, false);
        println!("KEY: {}", key);

        if t == BIN_NONE {
            println!("BIN_NONE");
            let subnode = binary_loads(reader, alt_format);
            node.data.insert(key, Value::KeyValueType(subnode));
        } else if t == BIN_STRING {
            let s = read_string(reader, false);
            println!("BIN_STRING: {}", s);
            node.data.insert(key, Value::StringType(s));
        } else if t == BIN_WIDESTRING {
            let s = read_string(reader, true);
            println!("BIN_WIDESTRING: {}", s);
            node.data.insert(key, Value::WideStringType(s));
        } else if [BIN_INT32, BIN_POINTER, BIN_COLOR].contains(&t) {
            let val = reader.read_i32::<LittleEndian>().unwrap();
            if t == BIN_INT32 {
                println!("BIN_INT32: {}", val);
                node.data.insert(key, Value::Int32Type(val));
            } else if t == BIN_POINTER {
                println!("BIN_POINTER: {}", val);
                node.data.insert(key, Value::PointerType(val));
            } else if t == BIN_COLOR {
                println!("BIN_COLOR: {}", val);
                node.data.insert(key, Value::ColorType(val));
            }
        } else if t == BIN_UINT64 {
            let val = reader.read_u64::<LittleEndian>().unwrap();
            println!("BIN_UINT64: {}", val);
            node.data.insert(key, Value::UInt64Type(val));
        } else if t == BIN_INT64 {
            let val = reader.read_i64::<LittleEndian>().unwrap();
            println!("BIN_INT64: {}", val);
            node.data.insert(key, Value::Int64Type(val));
        } else if t == BIN_FLOAT32 {
            let val = reader.read_f32::<LittleEndian>().unwrap();
            println!("BIN_FLOAT32: {}", val);
            node.data.insert(key, Value::Float32Type(val));
        } else {
            // FIXME: Function should return a Result, and this should be an
            // error.
            panic!("Invalid type: 0x{:X}", t);
        }
    }
}

fn read_string<R: std::io::Read>(reader: &mut R, wide: bool) -> String {
    if wide {
        let mut buf: Vec<u16> = vec![];
        loop {
            // Maybe this should be big-endian?
            let c = reader.read_u16::<LittleEndian>().unwrap();
            if c == 0 {
                break;
            }
            buf.push(c);
        }
        return std::string::String::from_utf16_lossy(&buf).to_string();
    } else {
        let mut buf: Vec<u8> = vec![];
        loop {
            let c = reader.read_u8().unwrap();
            if c == 0 {
                break;
            }
            buf.push(c);
        }
        return std::string::String::from_utf8_lossy(&buf).to_string();
    }
}
