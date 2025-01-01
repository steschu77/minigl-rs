use super::opengl::*;
pub mod window;

use windows::core::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::{Foundation::*, Graphics::Gdi::*, Graphics::OpenGL::*};

const OPENGL32: &str = "opengl32.dll\0";

pub struct Win32GLContext {
    hwnd: HWND,
    hdc: HDC,
    hglrc: HGLRC,
}

impl Win32GLContext {
    pub fn from_hwnd(hwnd: HWND) -> std::result::Result<Self, Box<dyn std::error::Error>> {
        let hdc = unsafe { GetDC(hwnd) };

        let pfd = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 24,
            cDepthBits: 32,
            iLayerType: PFD_MAIN_PLANE.0 as u8,
            ..Default::default()
        };

        let pf = unsafe { ChoosePixelFormat(hdc, &pfd) };
        let _ = unsafe { SetPixelFormat(hdc, pf, &pfd) };

        let hglrc = unsafe { wglCreateContext(hdc)? };
        unsafe { wglMakeCurrent(hdc, hglrc)? };

        Ok(Self { hwnd, hdc, hglrc })
    }

    pub fn load(&self) -> std::result::Result<OpenGLFunctions, Box<dyn std::error::Error>> {
        let opengl32 = unsafe { LoadLibraryA(PCSTR(OPENGL32.as_ptr())) };
        let Ok(opengl32) = opengl32 else {
            return Err("LoadLibraryA failed".into());
        };
        OpenGLFunctions::load(|fn_name| {
            let fn_ptr_ogl1 = unsafe { GetProcAddress(opengl32, PCSTR(fn_name.as_ptr())) };
            let fn_ptr_ogl2 = unsafe { wglGetProcAddress(PCSTR(fn_name.as_ptr())) };
            fn_ptr_ogl1.or(fn_ptr_ogl2).map(|f| f as FnOpenGL)
        })
    }

    pub fn swap_buffers(&self) {
        let _ = unsafe { SwapBuffers(self.hdc) };
    }
}

impl Drop for Win32GLContext {
    fn drop(&mut self) {
        let _ = unsafe { wglMakeCurrent(None, None) };
        let _ = unsafe { wglDeleteContext(self.hglrc) };
        unsafe { ReleaseDC(self.hwnd, self.hdc) };
    }
}
