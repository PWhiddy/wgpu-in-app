use app_surface::AppSurface;

pub trait Example {
    fn resize(&mut self, _app_surface: &AppSurface) {}
    fn enter_frame(&mut self, app_surface: &AppSurface);
    fn touch(&mut self, _touch: app_surface::Touch);
}

pub struct Empty;
impl Empty {
    pub fn new(_app_surface: &AppSurface) -> Self {
        Empty {}
    }
}
impl Example for Empty {
    fn enter_frame(&mut self, _app_surface: &AppSurface) {}
    fn touch(&mut self, _touch: app_surface::Touch) {}
}

mod msaa_line;
pub use msaa_line::MSAALine;

mod game_test;
pub use game_test::GameTest;