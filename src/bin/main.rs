#[macro_use]
extern crate log;
extern crate env_logger;

extern crate dwarfs;

use dwarfs::application;

fn main() {
    const WINDOW_SIZE: (f64, f64) = (800.0, 600.0);

    env_logger::init();

    let result = application::run(WINDOW_SIZE);
    match result {
        Ok(_) => { info!("Application exited successfully") },
        Err(e) => { error!("{}", e) }
    }
}
