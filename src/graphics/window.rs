use glutin;
use gfx;
use gfx_core;

use gfx::format::{ DepthStencil, Srgba8 };

use super::GraphicsError;

pub struct Window {
    events_loop: glutin::EventsLoop,
    gl_window: glutin::GlWindow,
    device: gfx_core::Device,
    factory: gfx_core::Factory,
    render_target_view: gfx::handle::RenderTargetView<gfx::Resources, Srgba8>,
    depth_stencil_view: gfx:handle::DepthStencil<gfx::Resources, DepthStencil>
}

impl Window {
    pub fn new(window_size: (f64, f64)) -> Result<Window, GraphicsError> {
        let events_loop = glutin::EventsLoop::new();
        let window_builder = glutin::WindowBuilder::new()
            .with_dimensions(glutin::dpi::LogicalSize::new(window_size.0, window_size.1));
        let context_builder = glutin::ContextBuilder::new()
            .with_vsync(true);
        let (window, device, factory, render_target_view, depth_stencil_view) =
            gfx_window_glutin::init::<Srgba8, DepthStencil>(window_builder, context_builder, &events_loop)?;
        Window {
            events_loop: events_loop,
            gl_window: window,
            device: device,
            factory: factory,
            render_target_view: render_target_view,
            depth_stencil_view: depth_stencil_view
        }
    }

    pub fn handle_events(&mut self) -> bool {
        let mut stop = false;
        self.events_loop.poll_events(|event| {
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
        stop
    }

    pub fn render(&mut self) -> Result<(), GraphicsError> {
        self.gl_window.swap_buffers()?;
    }
}