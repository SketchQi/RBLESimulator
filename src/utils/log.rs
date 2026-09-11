use std::io::Write;
use std::fs::OpenOptions;

pub fn log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("/tmp/ble_app.log") {
        let _ = writeln!(f, "[{:?}] {msg}", std::time::SystemTime::now());
    }
    println!("{msg}");
}