//! copy from wgpu's example

use super::Example;
use app_surface::{AppSurface, SurfaceFrame, TouchPhase};
use glam::Vec2;

extern crate game;
use game::game::{sim::Sim, camera::Camera, 
    control_state::ControlState,
    sim_audio::{SimAudio, SoundEventsMessage},
    sim_renderer::{SimRenderer, RenderOptions}, 
    demos, shared_constants,
};

pub struct GameTest {
    camera: Camera,
    control_state: ControlState,
    sim: Sim,
    renderer: SimRenderer,
    audio: SimAudio,
    dims: Vec2,
}

impl GameTest {
    pub fn new(app_surface: &AppSurface) -> Self {
        let config = &app_surface.config;
        let device = &app_surface.device;
        let (frame, view) = app_surface.get_current_frame_view(None);

        let camera = Camera::new(0.0018, 0.125, 0.004);
        //camera.zoom(1.2);
        let x = 896; //512; // approximate size for iphone 11
        let y = 1024 + 96;
        let state = demos::membranes_v1(x, y);  //demos::mega_pods_and_queens_turbo(x, y); 
          //demos::mega_pods_and_queens(x, y); // demos::plant_survival_resizable(x, y); //
        let control_state = ControlState::from_params(state.params);
        let sim = Sim::new(device, state);
        let renderer = SimRenderer::new(
            device,
            config.format,
            config.width,
            config.height,
            &sim
        );
        let audio = SimAudio::new();
        
        Self {
            camera,
            control_state,
            sim,
            renderer,
            audio,
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
            render_state_fields: 0,//104,//40,//7, //u32::MAX,
            render_entities: u32::MAX,
            render_links: true,
            debug_mode: false,
            mouse_window_coords: Vec2 {x: 0.0, y: 0.0},
            window_dimensions: Vec2{ x: frame.texture.width() as f32, y: frame.texture.height() as f32 },
        };

        self.control_state.update_current_sim_params(&self.camera);
        self.sim.set_sim_params(self.control_state.current_sim_params);

        self.sim.step(false, device, queue);

        self.control_state.current_sim_params = self.sim.get_sim_params_copy();
 
        let wbr = self.sim.get_latest_sim_gpu_write_back_result();
        let (short_sound_events, other_events) = wbr.step_events
            .into_iter()
            .take(wbr.step_event_count as usize)
            .fold((Vec::new(), Vec::new()), |(mut v1, mut v2), item| {
                match item.event_type {
                    shared_constants::EVENT_SHORT_SOUND_TRIGGER => v1.push(item),
                    _ => v2.push(item),
                }
                (v1, v2)
            });
        
        self.audio.queue_events(
            SoundEventsMessage {
                events: short_sound_events,
                time_interval: 0.016,
                main_volume: None,
            }
        );

        //self.sim.step(true, false, device, queue);
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
        let pix_scale = 2.0;
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
