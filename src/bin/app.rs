#[macro_use]
extern crate log;
extern crate env_logger;

extern crate dwarfs;

use dwarfs::Application;

fn main() {
    const WINDOW_SIZE: (f64, f64) = (800.0, 600.0);

    env_logger::init();

    let result = Application::new(WINDOW_SIZE);
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
