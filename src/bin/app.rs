#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

extern crate dwarfs;

use std::io::Write;
use log::Record;
use env_logger::{ Builder, fmt::Formatter };

fn main() {
    const WINDOW_SIZE: (f64, f64) = (800.0, 600.0);

    init_custom_logger();

    let result = dwarfs::Application::new(WINDOW_SIZE);
    match result {
        Ok(mut app) => {
            match app.run() {
                Ok(_) => { info!("Application exited sucessfully"); },
                Err(e) => { error!("{}", e); }
            }
        },
        Err(e) => { error!("{}", e);}
    }
}

fn init_custom_logger() {
    let format = |buf: &mut Formatter , record: &Record| {
        let time = chrono::Local::now();
        writeln!(buf, "[{} {:-5}] {}", time.format("%Y-%m-%d %H:%M:%S"), record.level(), record.args()) 
    };
    Builder::from_default_env()
        .format(format)
        .init();
}
