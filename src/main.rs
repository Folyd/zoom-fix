use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let hosts = include_str!("hosts");
    println!("{}", hosts);

    if cfg!(target_os = "windows") {
        // C:\Windows\System32\drivers\etc\
        match OpenOptions::new().append(true).open("C:\\Windows\\System32\\drivers\\etc\\hosts") {
            Ok(mut file) => {
                file.write_all(hosts.as_bytes()).unwrap();
            }
            Err(err) => { println!("err: {}", err) }
        }
    } else {
        // /etc/hosts
        match OpenOptions::new().append(true).open("/etc/hosts") {
            Ok(mut file) => {
                file.write_all(hosts.as_bytes()).unwrap();
            }
            Err(err) => { println!("err: {}", err) }
        }
    }
}
