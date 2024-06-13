//! copy from wgpu's example

use super::Example;
use app_surface::{AppSurface, SurfaceFrame};
use glam::Vec2;

extern crate game;
use game::game::{sim::Sim, camera::Camera, control_state::ControlState, sim_renderer::{SimRenderer, RenderOptions}, demos};

pub struct GameTest {
    camera: Camera,
    control_state: ControlState,
    sim: Sim,
    renderer: SimRenderer,
    render_field: bool,
}

impl GameTest {
    pub fn new(app_surface: &AppSurface) -> Self {
        let config = &app_surface.config;
        let device = &app_surface.device;

        let camera = Camera::new(0.0018);
        let control_state = ControlState::new();
        let x = 512; // approximate size for iphone 11
        let y = 1024 + 96;
        let state = demos::mega_pods_and_queens(x, y); // demos::plant_survival_resizable(x, y); //
        let sim = Sim::new(device, state);
    
        let renderer = SimRenderer::new(device, config, &sim);
        
        Self {
            camera,
            control_state,
            sim,
            renderer,
            render_field: false,
        }
    }
}

impl Example for GameTest {
    fn enter_frame(&mut self, app_surface: &AppSurface) {
        // log::info!("render triggered!");
        let device = &app_surface.device;
        let queue = &app_surface.queue;
        let (frame, view) = app_surface.get_current_frame_view(None);
        let options = RenderOptions {
            physics_delta_t_remainder: 0.0,
            render_repulse_field: self.render_field,
            render_state_fields: 7, //u32::MAX,
            render_entities: u32::MAX,
            render_links: true,
            debug_mode: false,
            mouse_window_coords: Vec2 {x: 0.0, y: 0.0},
            window_dimensions: Vec2{ x: frame.texture.width() as f32, y: frame.texture.height() as f32 },
        };
        self.sim.step(true, false, device, queue);
        self.sim.step(true, false, device, queue);
        self.sim.step(true, false, device, queue);
        let mut encoder =
            device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor {
                    label: Some(
                        "sim render command encoder",
                    ),
                },
            );
        self.renderer.render(&mut encoder, device, queue, &self.sim, &self.camera, &view, options);
        queue.submit([encoder.finish()]);
        frame.present();
    }

    fn touch(&mut self, touch: app_surface::Touch) {
        //log::info!("touch triggered!");
        println!("touch!");
        self.control_state.left_mouse_down();
        self.render_field = true;
        //self.sim.step()
        self.sim.set_player_mouse(touch.position.x * 0.001, touch.position.y * 0.001);
    }
}
