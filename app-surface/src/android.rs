use core::ffi::c_void;
use jni::sys::jobject;
use jni::JNIEnv;
use raw_window_handle::{AndroidNdkHandle, HasRawWindowHandle, RawWindowHandle};
use std::sync::Arc;

pub struct AppSurface {
    native_window: NativeWindow,
    pub scale_factor: f32,
    pub sdq: crate::SurfaceDeviceQueue,
    pub callback_to_app: Option<extern "C" fn(arg: i32)>,
}

impl AppSurface {
    pub fn new(env: *mut JNIEnv, surface: jobject) -> Self {
        let native_window = unsafe {
            NativeWindow::new(ndk_sys::ANativeWindow_fromSurface(
                env as *mut _,
                surface as *mut _,
            ))
        };
        let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(|| wgpu::Backends::GL);
        let instance = wgpu::Instance::new(backend);
        let surface = unsafe { instance.create_surface(&native_window) };
        let (_adapter, device, queue) =
            pollster::block_on(crate::request_device(&instance, backend, &surface));

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width: native_window.get_width(),
            height: native_window.get_height(),
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            native_window,
            scale_factor: 1.0,
            sdq: crate::SurfaceDeviceQueue {
                surface: surface,
                config,
                device: Arc::new(device),
                queue: Arc::new(queue),
            },
            callback_to_app: None,
        }
    }

    pub fn get_view_size(&self) -> (u32, u32) {
        (
            self.native_window.get_width(),
            self.native_window.get_height(),
        )
    }
}

struct NativeWindow {
    a_native_window: *mut ndk_sys::ANativeWindow,
}

impl NativeWindow {
    unsafe fn new(window: *mut ndk_sys::ANativeWindow) -> Self {
        ndk_sys::ANativeWindow_acquire(window);
        Self {
            a_native_window: window,
        }
    }

    fn get_width(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getWidth(self.a_native_window) as u32 }
    }

    fn get_height(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getHeight(self.a_native_window) as u32 }
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        unsafe {
            ndk_sys::ANativeWindow_release(self.a_native_window);
        }
    }
}

unsafe impl HasRawWindowHandle for NativeWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = AndroidNdkHandle::empty();
        handle.a_native_window = self.a_native_window as *mut _ as *mut c_void;
        RawWindowHandle::AndroidNdk(handle)
    }
}
