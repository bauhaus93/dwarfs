use glutin;
use gl;
use glutin::GlContext;

use super::graphics_error::GraphicsError;

pub struct Window {
    events_loop: glutin::EventsLoop,
    gl_window: glutin::GlWindow
}

impl Window {
    pub fn new(window_size: (f64, f64)) -> Result<Window, GraphicsError> {
        info!("Creating window, size {}x{}", window_size.0, window_size.1);
        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_dimensions(glutin::dpi::LogicalSize::new(window_size.0, window_size.1));
        let context_builder = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_depth_buffer(32);
        let gl_window = glutin::GlWindow::new(window_builder, context_builder, &events_loop)?;
       
        let window = Window {
            events_loop: events_loop,
            gl_window: gl_window
        };
        gl::load_with(|s| window.gl_window.context().get_proc_address(s) as *const _);
        unsafe {
            window.gl_window.make_current()?;
        }
        window.gl_window.show();
        Ok(window)
    }

    pub fn handle_events(&mut self) -> bool {
        let mut stop = false;
        self.events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => { stop = true; }
                        _ => { }
                    }
                },
                _ => { }
            }
        });
        stop
    }

    pub fn render(&mut self) -> Result<(), GraphicsError> {
        self.gl_window.swap_buffers()?;
        Ok(())
    }
}
