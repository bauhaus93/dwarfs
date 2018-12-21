use glutin;
use gfx_window_glutin;
//use gfx;

use application_error::ApplicationError;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4];4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub fn run(window_size: (f64, f64)) -> Result<(), ApplicationError> {
    let mut events_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_dimensions(glutin::dpi::LogicalSize::new(window_size.0, window_size.1));
    let context_builder = glutin::ContextBuilder::new()
        .with_vsync(true);
    let (window, mut device, mut factory, color_view, mut depth_view) =
        gfx_window_glutin::init::<gfx::format::Srgba8, gfx::format::DepthStencil>(window_builder, context_builder, &events_loop)?;

    debug!("Starting application main loop");
    let mut stop = false;
    while !stop {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { window_id, event } => {
                    match event {
                        glutin::WindowEvent::CloseRequested => { stop = true; }
                        _ => { }
                    }
                },
                _ => { }
            }
        });

        window.swap_buffers()?;
    }
    debug!("Stopped application main loop");

    Ok(())
}
