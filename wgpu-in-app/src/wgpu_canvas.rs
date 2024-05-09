use crate::examples::*;
use app_surface::{AppSurface, SurfaceFrame};
pub use app_surface::Touch;
pub struct WgpuCanvas {
    pub app_surface: AppSurface,
    example: Box<dyn Example>,
}

#[allow(dead_code)]
impl WgpuCanvas {
    pub fn new(app_surface: AppSurface, idx: i32) -> Self {
        let example = Box::new(Empty::new(&app_surface));
        log::info!("example created");
        let mut instance = WgpuCanvas {
            app_surface,
            example,
        };
        instance.change_example(idx);

        if let Some(callback) = instance.app_surface.callback_to_app {
            callback(0);
        }
        instance
    }

    pub fn enter_frame(&mut self) {
        self.example.enter_frame(&self.app_surface);

        if let Some(_callback) = self.app_surface.callback_to_app {
            // callback(1);
        }
    }

    pub fn resize(&mut self) {
        self.app_surface.resize_surface();
    }

    pub fn touch(&mut self, touch: Touch) {
        self.example.touch(touch)
    }

    pub fn change_example(&mut self, index: i32) {
        self.example = Self::create_a_example(&mut self.app_surface, index);
    }

    fn create_a_example(app_surface: &mut AppSurface, index: i32) -> Box<dyn Example> {
        if index == 0 {
            Box::new(GameTest::new(app_surface))
        } else {
            Box::new(MSAALine::new(app_surface))
        }
    }
}
