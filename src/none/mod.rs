use super::opengl::*;

pub struct NoneGLContext {}

impl NoneGLContext {
    pub fn from_nothing() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }

    pub fn load(&self) -> std::result::Result<OpenGLFunctions, Box<dyn std::error::Error>> {
        OpenGLFunctions::load(|_| None)
    }

    pub fn swap_buffers(&self) {}
}
