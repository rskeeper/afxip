use std::collections::HashMap;
use std::fs::File;

fn getGeoDb() -> File {
    return File::open("/tmp/geoip.data").unwrap();
}

fn getIpInfo() -> String {
    let _file: File = getGeoDb();
    let mut _flist: HashMap<String, String> = HashMap::new();
    _flist.insert("a".to_string(), "a1".to_string());
    return "".to_string();
}
