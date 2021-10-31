fn main() {
    let mut appinfo_file = std::fs::File::open("appinfo.vdf").unwrap();
    vdfr::appinfo_loads(&mut appinfo_file);
}
