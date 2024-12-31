use super::opengl::*;

pub struct NoneGLContext {}

impl NoneGLContext {
    pub fn from_nothing() -> std::result::Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }

    pub fn load(&self) -> std::result::Result<OpenGLFunctions, Box<dyn std::error::Error>> {
        let gl1_1 = OpenGLFunctions1_1 {
            fnViewport: |_, _, _, _| {},
            fnClearColor: |_, _, _, _| {},
            fnClear: |_| {},
            fnEnable: |_| {},
            fnDisable: |_| {},
            fnPointSize: |_| {},
            fnLineWidth: |_| {},
            fnGenTextures: |_, _| {},
            fnBindTexture: |_, _| {},
            fnTexImage1D: |_, _, _, _, _, _, _, _| {},
            fnTexImage2D: |_, _, _, _, _, _, _, _, _| {},
            fnTexParameterf: |_, _, _| {},
            fnTexParameterfv: |_, _, _| {},
            fnTexParameteri: |_, _, _| {},
            fnTexParameteriv: |_, _, _| {},
        };

        OpenGLFunctions::load(gl1_1, |_| None)
    }

    pub fn swap_buffers(&self) {}
}
