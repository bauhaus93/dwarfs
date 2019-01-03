use std::{ thread, time, ops::{ Add, Sub } };

use glutin;
use gl;

use super::ApplicationError;
use super::window;
use graphics;
use world;
use world::Updatable;

pub struct Application {
    events_loop: glutin::EventsLoop,
    window: glutin::GlWindow,
    shader_program: graphics::ShaderProgram,
    world: world::World,
    quit: bool,
    time_passed: u32,
    sleep_time: time::Duration
}

impl Application {
    pub fn new(window_size: (f64, f64)) -> Result<Application, ApplicationError> {
        let events_loop = glutin::EventsLoop::new();
        let window = window::init_window(window_size, &events_loop)?;
        let shader_program = graphics::ShaderProgramBuilder::new()
            .add_vertex_shader("resources/shader/VertexShader.glsl")
            .add_fragment_shader("resources/shader/FragmentShader.glsl")
            .finish()?;
        let world = world::World::new()?;
        let app = Self {
            events_loop: events_loop,
            window: window,
            shader_program: shader_program,
            world: world,
            quit: false,
            time_passed: 0,
            sleep_time: time::Duration::from_millis(50)
        };
        Ok(app)
    }

    pub fn run(mut self) -> Result<(), ApplicationError> {
        self.shader_program.use_program();
        let mut last_time = time::Instant::now();
        while !self.quit {
            self.handle_events();
            self.world.tick(self.time_passed);
            self.render()?;
            self.time_passed = last_time.elapsed().as_secs() as u32 * 1000 + last_time.elapsed().subsec_millis();
            last_time = time::Instant::now();
            self.handle_sleep_time();
            thread::sleep(self.sleep_time);
        }
        Ok(())
    }

    fn handle_events(&mut self) {
        let mut events: Vec<glutin::Event> = Vec::new();
        self.events_loop.poll_events(|event| { events.push(event); });
        for event in events {
            self.handle_event(event);
        }
    }

    fn handle_event(&mut self, event: glutin::Event) {
        match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::CloseRequested => { self.quit = true; }
                    _ => {}
                }
            },
            _ => {}
        }
    }

    fn handle_sleep_time(&mut self) {
        const TARGET_FREQ: u32 = 30;
        let diff: i32 = (self.time_passed * TARGET_FREQ) as i32 - 1000;
        if diff.abs() as u32 > TARGET_FREQ {
            let adj = time::Duration::from_millis(std::cmp::min(std::cmp::max(diff.abs() as u64 / 100, 1), 20));
            match diff.signum() {
                1 => self.sleep_time = self.sleep_time.sub(adj),
                -1 => self.sleep_time = self.sleep_time.add(adj),
                _ => {}
            }
        } 
    }

    fn render(&mut self) -> Result<(), graphics::GraphicsError> {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) }        
        self.world.render(&self.shader_program)?;
        self.window.swap_buffers()?;
        Ok(())
    }
}

