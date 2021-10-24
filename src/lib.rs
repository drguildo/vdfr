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

pub fn appinfo_loads<R: std::io::Read>(reader: &mut R) {
    let universe_version_struct = structure!("<II");
    let (universe, version) = universe_version_struct.unpack_from(reader).unwrap();
    println!("universe: 0x{:X}, version: {}", universe, version);

    let appid_struct = structure!("<I");
    let app_struct = structure!("<3IQ20sI");
    loop {
        let app_id = appid_struct.unpack_from(reader).unwrap();
        if app_id.0 == 0 {
            println!("last appinfo application");
            break;
        }

        let (size, state, last_update, access_token, checksum, change_number) =
            app_struct.unpack_from(reader).unwrap();
        println!("size: {}, state: {}, last_update: {}, access_token: {}, checksum: {:?}, change_number: {}", size, state, last_update, access_token, checksum, change_number);

        binary_loads(reader);
    }
}

fn binary_loads<R: std::io::Read>(reader: &mut R) {
    let int32_struct = structure!("<i");
    let uint64_struct = structure!("<Q");
    let int64_struct = structure!("<q");
    let float32_struct = structure!("<f");

    let mut stack_size = 1;
    loop {
        let t = reader.read_u8().unwrap();
        if t == BIN_END {
            println!("BIN_END");
            if stack_size > 1 {
                stack_size -= 1;
                continue;
            }
            break;
        }

        let key = read_string(reader, false);
        println!("KEY: {}", key);

        if t == BIN_NONE {
            println!("BIN_NONE");
            stack_size += 1;
        } else if t == BIN_STRING {
            println!("BIN_STRING: {}", read_string(reader, false));
        } else if t == BIN_WIDESTRING {
            println!("BIN_WIDESTRING: {}", read_string(reader, true));
        } else if [BIN_INT32, BIN_POINTER, BIN_COLOR].contains(&t) {
            let val = int32_struct.unpack_from(reader).unwrap();
            if t == BIN_INT32 {
                println!("BIN_INT32: {}", val.0);
            } else if t == BIN_POINTER {
                println!("BIN_POINTER: {}", val.0);
            } else if t == BIN_COLOR {
                println!("BIN_COLOR: {}", val.0);
            }
        } else if t == BIN_UINT64 {
            let val = uint64_struct.unpack_from(reader).unwrap();
            println!("BIN_UINT64: {}", val.0);
        } else if t == BIN_INT64 {
            let val = int64_struct.unpack_from(reader).unwrap();
            println!("BIN_INT64: {}", val.0);
        } else if t == BIN_FLOAT32 {
            let val = float32_struct.unpack_from(reader).unwrap();
            println!("BIN_FLOAT32: {}", val.0);
        } else {
            eprintln!("!! SYNTAX ERROR !!: 0x{:X}", t);
            std::process::exit(1);
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
