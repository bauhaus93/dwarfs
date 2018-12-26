use std::{ thread, time };

use glutin;

use super::ApplicationError;
use super::window;
use graphics;

pub fn run(window_size: (f64, f64)) -> Result<(), ApplicationError> {
    const SLEEP_TIME: time::Duration = time::Duration::from_millis(100);
    let (mut events_loop, mut window, shader_program, texture_array) = init(window_size)?;
 
    shader_program.use_program();
    debug!("Starting application main loop");
    loop {
        let stop = handle_events(&mut events_loop);
        if stop {
            break
        }
        render(&mut window)?;
        thread::sleep(SLEEP_TIME);
    }
    debug!("Stopped application main loop");
    Ok(())
}

fn init(window_size: (f64, f64)) -> Result<(glutin::EventsLoop, glutin::GlWindow, graphics::ShaderProgram, graphics::TextureArray), ApplicationError> {
    let events_loop = glutin::EventsLoop::new();
    let window = window::init_window(window_size, &events_loop)?;
    let program = graphics::ShaderProgramBuilder::new()
        .add_vertex_shader("resources/shader/VertexShader.glsl")
        .add_fragment_shader("resources/shader/FragmentShader.glsl")
        .finish()?;

    let texture_array = graphics::TextureArrayBuilder::new("resources/tex.png", (64, 64))
        .add_texture((0, 0))
        .add_texture((0, 64))
        .finish()?;
    Ok((events_loop, window, program, texture_array))
}

fn handle_events(events_loop: &mut glutin::EventsLoop) -> bool {
    let mut stop_requested = false;
    events_loop.poll_events(|event| {
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
    stop_requested
}

fn render(window: &mut glutin::GlWindow) -> Result<(), graphics::GraphicsError> {
    window.swap_buffers()?;
    Ok(())
}
