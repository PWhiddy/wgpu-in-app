//! copy from wgpu's example

use super::Example;
use app_surface::{AppSurface, SurfaceFrame, TouchPhase};
use glam::Vec2;

extern crate game;
use game::game::{sim::Sim, camera::Camera, control_state::ControlState, sim_renderer::{SimRenderer, RenderOptions}, demos};

pub struct GameTest {
    camera: Camera,
    control_state: ControlState,
    sim: Sim,
    renderer: SimRenderer,
    render_field: bool,
    dims: Vec2,
}

impl GameTest {
    pub fn new(app_surface: &AppSurface) -> Self {
        let config = &app_surface.config;
        let device = &app_surface.device;
        let (frame, view) = app_surface.get_current_frame_view(None);

        let camera = Camera::new(0.0018);
        let control_state = ControlState::new();
        let x = 512; // approximate size for iphone 11
        let y = 1024 + 96;
        let state = demos::mega_pods_and_queens_turbo(x, y); //demos::mega_pods_and_queens(x, y); // demos::plant_survival_resizable(x, y); //
        let sim = Sim::new(device, state);
    
        let renderer = SimRenderer::new(device, config, &sim);
        
        Self {
            camera,
            control_state,
            sim,
            renderer,
            render_field: false,
            dims: Vec2 { x: frame.texture.width() as f32, y: frame.texture.height() as f32 },
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

        self.sim.set_player_mouse(
            self.control_state.mouse_x
                / self.camera.scale
                + self.camera.position.x,
            self.control_state.mouse_y
                / self.camera.scale
                + self.camera.position.y,
        );

        self.sim.set_player_control_modes(
            self.control_state.control_modes,
        );

        self.sim.step(true, false, device, queue);
        self.sim.step(true, false, device, queue);
        //self.sim.step(true, false, device, queue);
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
        println!("touch - {:?}", touch);
        if touch.phase == TouchPhase::Started {
            self.control_state.left_mouse_down();
        }
        if touch.phase == TouchPhase::Ended {
            self.control_state.left_mouse_up();
        }
        //self.render_field = true;
        //self.sim.step()

        let aspect_ratio =
            self.dims.x as f32 / self.dims.y as f32;
        let pix_scale = 3.0;
        let normalized_x = (2.0
            * (pix_scale * touch.position.x as f32 / self.dims.x as f32)
            - 1.0)
            * aspect_ratio;
        let normalized_y = 1.0
            - 2.0
                * (pix_scale * touch.position.y as f32
                    / self.dims.y as f32);
        self.control_state
            .set_mouse_position(normalized_x, normalized_y);

        //self.control_state.set_player_mouse(touch.position.x, touch.position.y);
    }
}
