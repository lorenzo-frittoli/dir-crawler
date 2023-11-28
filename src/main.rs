use std::ffi::OsString;
use std::fs;
use std::collections::BTreeSet;


// fn listdir(dir: &str, set: &mut BTreeSet<&str>) {
//     let paths = fs::read_dir(dir).unwrap();
//     for path in paths {
//         let path = path.unwrap();
//         if path.metadata().unwrap().is_dir() {
//             // listdir(path.as_ref().unwrap().path().to_str().unwrap().to_string());
//             // listdir(path.as_ref().unwrap().path().to_str().unwrap());
//             listdir(path.path().to_str().unwrap(), set);
//         } else {
//             set.insert(path.file_name().clone().to_str().unwrap());
//         }
//     }
// }

fn listdir_string(dir: OsString, set: &mut BTreeSet<OsString>) {
    let paths = fs::read_dir(dir);
    for path in paths.unwrap() {
        let path = path.unwrap();
        let isdir = path.metadata().unwrap().is_dir();
        let filepath = path.path().into_os_string();
        let filename = path.file_name();
        if isdir {
            listdir_string(filepath, set)
        } else {
            set.insert(filename);
        }
    }
}

fn main() {
    let mut set: BTreeSet<OsString>  = BTreeSet::new();
    listdir_string(OsString::from("./"), &mut set);
    for f in set {println!("{:?}", f)}
}