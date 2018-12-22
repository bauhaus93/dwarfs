
use application_error::ApplicationError;
use graphics;

pub fn run(window_size: (f64, f64)) -> Result<(), ApplicationError> {
    let mut wnd = graphics::Window::new(window_size);

    debug!("Starting application main loop");
    loop {
        let stop = wnd.handle_events();
        if stop {
            break;
        }
        wnd.render();

    }
    debug!("Stopped application main loop");

    Ok(())
}
