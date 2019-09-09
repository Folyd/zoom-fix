use std::fs::OpenOptions;
use std::io::{Read, Write};

fn main() {
    let path = if cfg!(target_os = "windows") {
        "C:\\Windows\\System32\\drivers\\etc\\hosts"
    } else {
        "/etc/hosts"
    };

    let hosts = include_str!("hosts");
    println!("{}", hosts);

    match OpenOptions::new().read(true).append(true).open(path) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            if !content.contains("# ZOOM IP list") {
                file.write_all(hosts.as_bytes()).unwrap();
            }
        }
        Err(err) => { println!("err: {}", err) }
    }
}
