use glutin;
use gl;
use glutin::GlContext;

use graphics;

pub fn init_window(window_size: (f64, f64), events_loop: &glutin::EventsLoop) -> Result<glutin::GlWindow, graphics::GraphicsError> {
    info!("Creating window, size {}x{}", window_size.0, window_size.1);
    let window_builder = glutin::WindowBuilder::new()
        .with_dimensions(glutin::dpi::LogicalSize::new(window_size.0, window_size.1));
    let context_builder = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_depth_buffer(8);
    let window = glutin::GlWindow::new(window_builder, context_builder, &events_loop)?;
    gl::load_with(|s| window.context().get_proc_address(s) as *const _);

    debug!("Making window active");
    unsafe {
        window.make_current()?;
    }
    window.show();

    match graphics::get_opengl_version() {
        Ok(version) => { info!("opengl version: {}", version) },
        Err(e) => { warn!("Could not convert opengl version string: {}", e); }
    }
    Ok(window)
}
