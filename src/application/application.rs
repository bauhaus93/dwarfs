use std::{ thread, time, ops::{ Add, Sub } };

use glutin;
use gl;
use gl::types::GLsizei;

use super::ApplicationError;
use super::window;
use graphics;
use world;
use world::traits::Updatable;

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
                    glutin::WindowEvent::CloseRequested => { self.quit = true; },
                    glutin::WindowEvent::Resized(logical_size) => { self.handle_resize((logical_size.width as GLsizei, logical_size.height as GLsizei)); },
                    glutin::WindowEvent::KeyboardInput { input, .. } => { self.handle_keyboard_input(input) },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    fn handle_resize(&self, new_size: (GLsizei, GLsizei)) {
        unsafe {
            //gl::Viewport(0, 0, new_size.0, new_size.1);
        }
        info!("NOT! Updated viewport to {}/{}/{}/{}", 0, 0, new_size.0, new_size.1);
        match graphics::check_opengl_error("gl::Viewport") {
            Ok(_) => {},
            Err(e) => { warn!("{}", e); }
        } 
    }

    fn handle_keyboard_input(&mut self, input: glutin::KeyboardInput) {
        match (input.virtual_keycode, input.state) {
            (Some(keycode), glutin::ElementState::Pressed) => {
                match keycode {
                    glutin::VirtualKeyCode::A => self.world.move_camera([-1., 1., 0.]),
                    glutin::VirtualKeyCode::D => self.world.move_camera([1., -1., 0.]),
                    glutin::VirtualKeyCode::W => self.world.move_camera([1., 1., 0.]),
                    glutin::VirtualKeyCode::S => self.world.move_camera([-1., -1., 0.]),
                    glutin::VirtualKeyCode::R => self.world.move_camera([0., 0., 1.]),
                    glutin::VirtualKeyCode::F => self.world.move_camera([0., 0., -1.]),
                    _ => {}
                }
            },
            (_, _) => {}
        }
    }

    fn handle_sleep_time(&mut self) {
        const TARGET_FREQ: u32 = 30;
        let diff: i32 = (self.time_passed * TARGET_FREQ) as i32 - 1000;
        if diff.abs() as u32 > TARGET_FREQ {
            let adj = time::Duration::from_millis(std::cmp::min(std::cmp::max(diff.abs() as u64 / 100, 1), self.sleep_time.subsec_millis() as u64));
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

