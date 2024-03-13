//! copy from wgpu's example

use super::Example;
use app_surface::{AppSurface, SurfaceFrame};
use bytemuck::{Pod, Zeroable};
use rand::{
    distributions::{Distribution, Uniform},
    SeedableRng,
};
use std::{borrow::Cow, mem};
use wgpu::util::DeviceExt;

extern crate game;
use game::game::{sim::Sim, camera::Camera, control_state::ControlState, sim_renderer::{SimRenderer, RenderOptions}, demos};

pub struct GameTest {
    camera: Camera,
    control_state: ControlState,
    sim: Sim,
    renderer: SimRenderer,
}

impl GameTest {
    pub fn new(app_surface: &AppSurface) -> Self {
        let config = &app_surface.config;
        let device = &app_surface.device;
        let queue = &app_surface.queue;

        let camera = Camera::new(0.002);
        let control_state = ControlState::new();
    
        let state = demos::plant_survival();
        let sim = Sim::new(device, queue, state);
    
        let renderer = SimRenderer::new(device, queue, config, &sim);
        
        Self {
            camera,
            control_state,
            sim,
            renderer,
        }
    }
}

impl Example for GameTest {
    fn enter_frame(&mut self, app_surface: &AppSurface) {
        let device = &app_surface.device;
        let queue = &app_surface.queue;
        let (frame, view) = app_surface.get_current_frame_view(None);
        let options = RenderOptions {
            render_field: false,
            render_entities: u32::MAX,
            render_links: true,
        };
        self.sim.step(true, device, queue);
        self.renderer.render(device, queue, &self.sim, &self.camera, &view, options);
        frame.present();
    }
}
