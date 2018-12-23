use std::{ thread, time };

use glutin;

use application_error::ApplicationError;
use graphics;


pub struct Application {
    stop: bool,
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow
}

impl Application {

    pub fn new(window_size: (f64, f64)) -> Result<Application, ApplicationError> {
        let events_loop = glutin::EventsLoop::new();
        let window = graphics::init_window(window_size, &events_loop)?;
        let app = Application {
            stop: false,
            events_loop: events_loop,
            window: window
        };
        Ok(app)
    }

    pub fn run(&mut self) -> Result<(), ApplicationError> {
        const SLEEP_TIME: time::Duration = time::Duration::from_millis(100);
        debug!("Starting application main loop");
        self.stop = false;
        while !self.stop {
            self.handle_events();
            self.render()?;
            thread::sleep(SLEEP_TIME);
        }
        debug!("Stopped application main loop");
        Ok(())
    }


    fn handle_events(&mut self) {
        let mut stop_requested = false;
        self.events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => { stop_requested = true; }
                        _ => { }
                    }
                },
                _ => { }
            }
        });
        if stop_requested {
            self.stop = true;
        }
    }

    fn render(&mut self) -> Result<(), graphics::GraphicsError> {
        self.window.swap_buffers()?;
        Ok(())
    }

}

