use super::opengl::*;

use x11::xlib::*;

pub struct LinuxGLContext {
    display: *mut Display,
    window: Window,
    context: x11::glx::GLXContext,
}

impl LinuxGLContext {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_window(
        display: *mut Display,
        screen: std::os::raw::c_int,
        window: Window,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut attribs = [
            x11::glx::GLX_RGBA,
            x11::glx::GLX_DOUBLEBUFFER,
            x11::glx::GLX_DEPTH_SIZE,
            24,
            0,
        ];        
        let visual_info = x11::glx::glXChooseVisual(display, screen, attribs.as_mut_ptr());
        let context = x11::glx::glXCreateContext(display, visual_info, std::ptr::null_mut(), 1);
        x11::glx::glXMakeCurrent(display, window, context);
        Ok(Self {
            display,
            window,
            context
        })
    }

    pub fn load(&self) -> std::result::Result<OpenGLFunctions, Box<dyn std::error::Error>> {
        OpenGLFunctions::load(|fn_name| {
            let fn_ptr = unsafe { x11::glx::glXGetProcAddress(fn_name.as_ptr() as *const _) };
            fn_ptr.map(|f| f as FnOpenGL)
        })
    }

    pub fn swap_buffers(&self) {
        unsafe { x11::glx::glXSwapBuffers(self.display, self.window) };
    }
}

impl Drop for LinuxGLContext {
    fn drop(&mut self) {
        unsafe { x11::glx::glXDestroyContext(self.display, self.context) };
    }
}
