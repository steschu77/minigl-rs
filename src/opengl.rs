#![allow(non_snake_case)]

pub type GLenum = std::os::raw::c_uint;
pub type GLboolean = std::os::raw::c_uchar;
pub type GLbitfield = std::os::raw::c_uint;
pub type GLvoid = std::os::raw::c_void;
pub type GLbyte = std::os::raw::c_char;
pub type GLshort = std::os::raw::c_short;
pub type GLint = std::os::raw::c_int;
pub type GLubyte = std::os::raw::c_uchar;
pub type GLushort = std::os::raw::c_ushort;
pub type GLuint = std::os::raw::c_uint;
pub type GLsizei = std::os::raw::c_int;
pub type GLfloat = std::os::raw::c_float;
pub type GLclampf = std::os::raw::c_float;
pub type GLdouble = std::os::raw::c_double;
pub type GLchar = std::os::raw::c_uchar;

pub const FALSE: GLboolean = 0;
pub const TRUE: GLboolean = 1;

pub const DEPTH_BUFFER_BIT: GLbitfield = 0x00000100;
pub const COLOR_BUFFER_BIT: GLbitfield = 0x00004000;

pub const POINTS: GLenum = 0x0000;
pub const LINES: GLenum = 0x0001;
pub const LINE_LOOP: GLenum = 0x0002;
pub const LINE_STRIP: GLenum = 0x0003;
pub const TRIANGLES: GLenum = 0x0004;
pub const TRIANGLE_STRIP: GLenum = 0x0005;
pub const TRIANGLE_FAN: GLenum = 0x0006;
pub const QUADS: GLenum = 0x0007;

pub const TEXTURE_1D: GLenum = 0x0DE0;
pub const TEXTURE_2D: GLenum = 0x0DE1;
pub const TEXTURE_3D: GLenum = 0x806F;

pub const BYTE: GLenum = 0x1400;
pub const UNSIGNED_BYTE: GLenum = 0x1401;
pub const SHORT: GLenum = 0x1402;
pub const UNSIGNED_SHORT: GLenum = 0x1403;
pub const INT: GLenum = 0x1404;
pub const UNSIGNED_INT: GLenum = 0x1405;
pub const FLOAT: GLenum = 0x1406;
pub const DOUBLE: GLenum = 0x140A;

pub const RGB: GLenum = 0x1907;
pub const RGBA: GLenum = 0x1908;

pub const RGB5: GLenum = 0x8050;
pub const RGB8: GLenum = 0x8051;
pub const RGB10: GLenum = 0x8052;
pub const RGB12: GLenum = 0x8053;
pub const RGB16: GLenum = 0x8054;
pub const RGBA2: GLenum = 0x8055;
pub const RGBA4: GLenum = 0x8056;
pub const RGB5_A1: GLenum = 0x8057;
pub const RGBA8: GLenum = 0x8058;
pub const RGB10_A2: GLenum = 0x8059;
pub const RGBA12: GLenum = 0x805A;
pub const RGBA16: GLenum = 0x805B;

pub const R16F: GLenum = 0x822D;
pub const RG16F: GLenum = 0x822F;
pub const RGB16F: GLenum = 0x881B;
pub const RGBA16F: GLenum = 0x881A;

pub const BLEND: GLenum = 0x0BE2;
pub const CULL_FACE: GLenum = 0x0B44;
pub const DEPTH_TEST: GLenum = 0x0B71;
pub const LINE_SMOOTH: GLenum = 0x0B20;
pub const PROGRAM_POINT_SIZE: GLenum = 0x8642;

pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const NEAREST: GLenum = 0x2600;
pub const LINEAR: GLenum = 0x2601;
pub const NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub const NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub const LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;

pub const TEXTURE_WRAP_R: GLenum = 0x8072;
pub const TEXTURE_WRAP_S: GLenum = 0x2802;
pub const TEXTURE_WRAP_T: GLenum = 0x2803;
pub const CLAMP_TO_EDGE: GLenum = 0x812F;

pub const TEXTURE0: GLenum = 0x84C0;

pub const ZERO: GLenum = 0;
pub const ONE: GLenum = 1;
pub const SRC_COLOR: GLenum = 0x0300;
pub const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const SRC_ALPHA: GLenum = 0x0302;
pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const DST_ALPHA: GLenum = 0x0304;
pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const DST_COLOR: GLenum = 0x0306;
pub const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const SRC_ALPHA_SATURATE: GLenum = 0x0308;

pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const STATIC_DRAW: GLenum = 0x88E4;
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const VERTEX_SHADER: GLenum = 0x8B31;

pub const FRAMEBUFFER: GLenum = 0x8D40;
pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;

pub const COLOR_ATTACHMENT: GLenum = 0x8CE0;
pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;

pub type FnViewport = unsafe fn(GLint, GLint, GLsizei, GLsizei);
pub type FnClearColor = unsafe fn(GLfloat, GLfloat, GLfloat, GLfloat);
pub type FnClear = unsafe fn(GLbitfield);
pub type FnEnable = unsafe fn(GLenum);
pub type FnDisable = unsafe fn(GLenum);
pub type FnAlphaFunc = unsafe fn(GLenum, GLclampf);
pub type FnBlendFunc = unsafe fn(GLenum, GLenum);
pub type FnPointSize = unsafe fn(GLfloat);
pub type FnLineWidth = unsafe fn(GLfloat);
pub type FnGenTextures = unsafe fn(GLsizei, *mut GLuint);
pub type FnBindTexture = unsafe fn(GLenum, GLuint);
pub type FnTexImage1D =
    unsafe fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const GLvoid);
pub type FnTexImage2D =
    unsafe fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const GLvoid);
pub type FnTexParameterf = unsafe fn(GLenum, GLenum, GLfloat);
pub type FnTexParameterfv = unsafe fn(GLenum, GLenum, *const GLfloat);
pub type FnTexParameteri = unsafe fn(GLenum, GLenum, GLint);
pub type FnTexParameteriv = unsafe fn(GLenum, GLenum, *const GLint);

pub type FnActiveTexture = unsafe extern "system" fn(GLenum);

pub type FnCreateProgram = unsafe extern "system" fn() -> GLuint;
pub type FnDeleteProgram = unsafe extern "system" fn(GLuint);
pub type FnValidateProgram = unsafe extern "system" fn(GLuint);
pub type FnLinkProgram = unsafe extern "system" fn(GLuint);
pub type FnUseProgram = unsafe extern "system" fn(GLuint);
pub type FnGetProgramiv = unsafe extern "system" fn(GLuint, GLenum, *mut GLint);

pub type FnCreateShader = unsafe extern "system" fn(GLenum) -> GLuint;
pub type FnDeleteShader = unsafe extern "system" fn(GLuint);
pub type FnCompileShader = unsafe extern "system" fn(GLuint);
pub type FnAttachShader = unsafe extern "system" fn(GLuint, GLuint);
pub type FnDetachShader = unsafe extern "system" fn(GLuint, GLuint);
pub type FnShaderSource =
    unsafe extern "system" fn(GLuint, GLsizei, *const *const GLchar, *const GLint);

pub type FnGenBuffers = unsafe extern "system" fn(GLsizei, *mut GLuint);
pub type FnBindBuffer = unsafe extern "system" fn(GLenum, GLuint);
pub type FnBufferData = unsafe extern "system" fn(GLenum, usize, *const GLvoid, GLenum);
pub type FnDeleteBuffers = unsafe extern "system" fn(GLsizei, *const GLuint);
pub type FnDrawBuffers = unsafe extern "system" fn(GLsizei, *const GLenum);
pub type FnDrawArrays = unsafe extern "system" fn(GLenum, GLint, GLsizei);

pub type FnEnableVertexAttribArray = unsafe extern "system" fn(GLuint);
pub type FnDisableVertexAttribArray = unsafe extern "system" fn(GLuint);
pub type FnGenVertexArrays = unsafe extern "system" fn(GLsizei, *mut GLuint);
pub type FnDeleteVertexArrays = unsafe extern "system" fn(GLsizei, *const GLuint);
pub type FnBindVertexArray = unsafe extern "system" fn(GLuint);
pub type FnGetAttribLocation = unsafe extern "system" fn(GLuint, *const GLchar) -> GLint;
pub type FnVertexAttribPointer =
    unsafe extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const GLvoid);

pub type FnBindFramebuffer = unsafe extern "system" fn(GLenum, GLuint);
pub type FnGenFramebuffers = unsafe extern "system" fn(GLsizei, *mut GLuint);
pub type FnDeleteFramebuffers = unsafe extern "system" fn(GLsizei, *const GLuint);
pub type FnFramebufferTexture2D = unsafe extern "system" fn(GLenum, GLenum, GLenum, GLuint, GLint);
pub type FnCheckFramebufferStatus = unsafe extern "system" fn(GLenum) -> GLenum;

pub type FnGetUniformLocation = unsafe extern "system" fn(GLuint, *const GLchar) -> GLint;
pub type FnUniform1i = unsafe extern "system" fn(GLint, GLint);
pub type FnUniform2i = unsafe extern "system" fn(GLint, GLint, GLint);
pub type FnUniform3i = unsafe extern "system" fn(GLint, GLint, GLint, GLint);
pub type FnUniform4i = unsafe extern "system" fn(GLint, GLint, GLint, GLint, GLint);
pub type FnUniform1iv = unsafe extern "system" fn(GLint, GLsizei, *const GLint);
pub type FnUniform2iv = unsafe extern "system" fn(GLint, GLsizei, *const GLint);
pub type FnUniform3iv = unsafe extern "system" fn(GLint, GLsizei, *const GLint);
pub type FnUniform4iv = unsafe extern "system" fn(GLint, GLsizei, *const GLint);
pub type FnUniform1f = unsafe extern "system" fn(GLint, GLfloat);
pub type FnUniform2f = unsafe extern "system" fn(GLint, GLfloat, GLfloat);
pub type FnUniform3f = unsafe extern "system" fn(GLint, GLfloat, GLfloat, GLfloat);
pub type FnUniform4f = unsafe extern "system" fn(GLint, GLfloat, GLfloat, GLfloat, GLfloat);
pub type FnUniform1fv = unsafe extern "system" fn(GLint, GLsizei, *const GLfloat);
pub type FnUniform2fv = unsafe extern "system" fn(GLint, GLsizei, *const GLfloat);
pub type FnUniform3fv = unsafe extern "system" fn(GLint, GLsizei, *const GLfloat);
pub type FnUniform4fv = unsafe extern "system" fn(GLint, GLsizei, *const GLfloat);
pub type FnUniformMatrix2fv = unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
pub type FnUniformMatrix3fv = unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);
pub type FnUniformMatrix4fv = unsafe extern "system" fn(GLint, GLsizei, GLboolean, *const GLfloat);

pub struct OpenGLFunctions {
    fnViewport: FnViewport,
    fnClearColor: FnClearColor,
    fnClear: FnClear,
    fnEnable: FnEnable,
    fnDisable: FnDisable,
    fnAlphaFunc: FnAlphaFunc,
    fnBlendFunc: FnBlendFunc,
    fnPointSize: FnPointSize,
    fnLineWidth: FnLineWidth,
    fnGenTextures: FnGenTextures,
    fnBindTexture: FnBindTexture,
    fnTexImage1D: FnTexImage1D,
    fnTexImage2D: FnTexImage2D,
    fnTexParameterf: FnTexParameterf,
    fnTexParameterfv: FnTexParameterfv,
    fnTexParameteri: FnTexParameteri,
    fnTexParameteriv: FnTexParameteriv,

    fnActiveTexture: FnActiveTexture,

    fnCreateProgram: FnCreateProgram,
    fnDeleteProgram: FnDeleteProgram,
    fnValidateProgram: FnValidateProgram,
    fnLinkProgram: FnLinkProgram,
    fnUseProgram: FnUseProgram,
    fnGetProgramiv: FnGetProgramiv,

    fnCreateShader: FnCreateShader,
    fnDeleteShader: FnDeleteShader,
    fnCompileShader: FnCompileShader,
    fnAttachShader: FnAttachShader,
    fnDetachShader: FnDetachShader,
    fnShaderSource: FnShaderSource,

    fnGenBuffers: FnGenBuffers,
    fnBindBuffer: FnBindBuffer,
    fnBufferData: FnBufferData,
    fnDeleteBuffers: FnDeleteBuffers,
    fnDrawBuffers: FnDrawBuffers,
    fnDrawArrays: FnDrawArrays,

    fnEnableVertexAttribArray: FnEnableVertexAttribArray,
    fnDisableVertexAttribArray: FnDisableVertexAttribArray,
    fnGenVertexArrays: FnGenVertexArrays,
    fnDeleteVertexArrays: FnDeleteVertexArrays,
    fnBindVertexArray: FnBindVertexArray,
    fnGetAttribLocation: FnGetAttribLocation,
    fnVertexAttribPointer: FnVertexAttribPointer,

    fnBindFramebuffer: FnBindFramebuffer,
    fnGenFramebuffers: FnGenFramebuffers,
    fnDeleteFramebuffers: FnDeleteFramebuffers,
    fnFramebufferTexture2D: FnFramebufferTexture2D,
    fnCheckFramebufferStatus: FnCheckFramebufferStatus,

    fnGetUniformLocation: FnGetUniformLocation,
    fnUniform1i: FnUniform1i,
    fnUniform2i: FnUniform2i,
    fnUniform3i: FnUniform3i,
    fnUniform4i: FnUniform4i,
    fnUniform1iv: FnUniform1iv,
    fnUniform2iv: FnUniform2iv,
    fnUniform3iv: FnUniform3iv,
    fnUniform4iv: FnUniform4iv,
    fnUniform1f: FnUniform1f,
    fnUniform2f: FnUniform2f,
    fnUniform3f: FnUniform3f,
    fnUniform4f: FnUniform4f,
    fnUniform1fv: FnUniform1fv,
    fnUniform2fv: FnUniform2fv,
    fnUniform3fv: FnUniform3fv,
    fnUniform4fv: FnUniform4fv,
    fnUniformMatrix2fv: FnUniformMatrix2fv,
    fnUniformMatrix3fv: FnUniformMatrix3fv,
    fnUniformMatrix4fv: FnUniformMatrix4fv,
}

#[derive(Debug)]

pub struct OpenGLLoadError {
    name: String,
}

impl std::fmt::Display for OpenGLLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to load '{}'", self.name)
    }
}

impl std::error::Error for OpenGLLoadError {}

pub type FnOpenGL = *const ();

// Macro for loading an OpenGL function pointer $fn_name and casting it to its function type $fn_type.
macro_rules! load_gl_fn {
    ( $load_fn:ident, $fn_name:expr => $fn_type:ty ) => {{
        $load_fn($fn_name)
            .map(|f| unsafe { std::mem::transmute::<FnOpenGL, $fn_type>(f) })
            .ok_or(OpenGLLoadError {
                name: $fn_name.to_string(),
            })
    }};
}

// Macro for implementing an OpenGL function $name by calling their function pointer $fn_name.
macro_rules! impl_gl_fn {
    // Variant for functions with a return value.
    ($fn_name:ident, $name:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty) => {
        #[allow(clippy::too_many_arguments)]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe fn $name(&self, $($arg: $arg_ty),*) -> $ret { unsafe {
            (self.$fn_name)($($arg),*)
        }}
    };

    // Variant for functions that return void.
    ($fn_name:ident, $name:ident($($arg:ident: $arg_ty:ty),*)) => {
        impl_gl_fn!($fn_name, $name($($arg: $arg_ty),*) -> ());
    };
}

impl OpenGLFunctions {
    pub fn load<F>(load_fn: F) -> std::result::Result<Self, Box<dyn std::error::Error>>
    where
        F: Fn(&'static str) -> Option<FnOpenGL>,
    {
        Ok(Self {
            fnViewport: load_gl_fn!(load_fn, "glViewport\0" => FnViewport)?,
            fnClearColor: load_gl_fn!(load_fn, "glClearColor\0" => FnClearColor)?,
            fnClear: load_gl_fn!(load_fn, "glClear\0" => FnClear)?,
            fnEnable: load_gl_fn!(load_fn, "glEnable\0" => FnEnable)?,
            fnDisable: load_gl_fn!(load_fn, "glDisable\0" => FnDisable)?,
            fnAlphaFunc: load_gl_fn!(load_fn, "glAlphaFunc\0" => FnAlphaFunc)?,
            fnBlendFunc: load_gl_fn!(load_fn, "glBlendFunc\0" => FnBlendFunc)?,
            fnPointSize: load_gl_fn!(load_fn, "glPointSize\0" => FnPointSize)?,
            fnLineWidth: load_gl_fn!(load_fn, "glLineWidth\0" => FnLineWidth)?,
            fnGenTextures: load_gl_fn!(load_fn, "glGenTextures\0" => FnGenTextures)?,
            fnBindTexture: load_gl_fn!(load_fn, "glBindTexture\0" => FnBindTexture)?,
            fnTexImage1D: load_gl_fn!(load_fn, "glTexImage1D\0" => FnTexImage1D)?,
            fnTexImage2D: load_gl_fn!(load_fn, "glTexImage2D\0" => FnTexImage2D)?,
            fnTexParameterf: load_gl_fn!(load_fn, "glTexParameterf\0" => FnTexParameterf)?,
            fnTexParameterfv: load_gl_fn!(load_fn, "glTexParameterfv\0" => FnTexParameterfv)?,
            fnTexParameteri: load_gl_fn!(load_fn, "glTexParameteri\0" => FnTexParameteri)?,
            fnTexParameteriv: load_gl_fn!(load_fn, "glTexParameteriv\0" => FnTexParameteriv)?,

            fnActiveTexture: load_gl_fn!(load_fn, "glActiveTexture\0" => FnActiveTexture)?,

            fnCreateProgram: load_gl_fn!(load_fn, "glCreateProgram\0" => FnCreateProgram)?,
            fnDeleteProgram: load_gl_fn!(load_fn, "glDeleteProgram\0" => FnDeleteProgram)?,
            fnValidateProgram: load_gl_fn!(load_fn, "glValidateProgram\0" => FnValidateProgram)?,
            fnLinkProgram: load_gl_fn!(load_fn, "glLinkProgram\0" => FnLinkProgram)?,
            fnUseProgram: load_gl_fn!(load_fn, "glUseProgram\0" => FnUseProgram)?,
            fnGetProgramiv: load_gl_fn!(load_fn, "glGetProgramiv\0" => FnGetProgramiv)?,

            fnCreateShader: load_gl_fn!(load_fn, "glCreateShader\0" => FnCreateShader)?,
            fnDeleteShader: load_gl_fn!(load_fn, "glDeleteShader\0" => FnDeleteShader)?,
            fnCompileShader: load_gl_fn!(load_fn, "glCompileShader\0" => FnCompileShader)?,
            fnAttachShader: load_gl_fn!(load_fn, "glAttachShader\0" => FnAttachShader)?,
            fnDetachShader: load_gl_fn!(load_fn, "glDetachShader\0" => FnDetachShader)?,
            fnShaderSource: load_gl_fn!(load_fn, "glShaderSource\0" => FnShaderSource)?,

            fnGenBuffers: load_gl_fn!(load_fn, "glGenBuffers\0" => FnGenBuffers)?,
            fnBindBuffer: load_gl_fn!(load_fn, "glBindBuffer\0" => FnBindBuffer)?,
            fnBufferData: load_gl_fn!(load_fn, "glBufferData\0" => FnBufferData)?,
            fnDeleteBuffers: load_gl_fn!(load_fn, "glDeleteBuffers\0" => FnDeleteBuffers)?,
            fnDrawBuffers: load_gl_fn!(load_fn, "glDrawBuffers\0" => FnDrawBuffers)?,
            fnDrawArrays: load_gl_fn!(load_fn, "glDrawArrays\0" => FnDrawArrays)?,

            fnEnableVertexAttribArray: load_gl_fn!(load_fn, "glEnableVertexAttribArray\0" => FnEnableVertexAttribArray)?,
            fnDisableVertexAttribArray: load_gl_fn!(load_fn, "glDisableVertexAttribArray\0" => FnDisableVertexAttribArray)?,
            fnGenVertexArrays: load_gl_fn!(load_fn, "glGenVertexArrays\0" => FnGenVertexArrays)?,
            fnDeleteVertexArrays: load_gl_fn!(load_fn, "glDeleteVertexArrays\0" => FnDeleteVertexArrays)?,
            fnBindVertexArray: load_gl_fn!(load_fn, "glBindVertexArray\0" => FnBindVertexArray)?,
            fnGetAttribLocation: load_gl_fn!(load_fn, "glGetAttribLocation\0" => FnGetAttribLocation)?,
            fnVertexAttribPointer: load_gl_fn!(load_fn, "glVertexAttribPointer\0" => FnVertexAttribPointer)?,

            fnBindFramebuffer: load_gl_fn!(load_fn, "glBindFramebuffer\0" => FnBindFramebuffer)?,
            fnGenFramebuffers: load_gl_fn!(load_fn, "glGenFramebuffers\0" => FnGenFramebuffers)?,
            fnDeleteFramebuffers: load_gl_fn!(load_fn, "glDeleteFramebuffers\0" => FnDeleteFramebuffers)?,
            fnFramebufferTexture2D: load_gl_fn!(load_fn, "glFramebufferTexture2D\0" => FnFramebufferTexture2D)?,
            fnCheckFramebufferStatus: load_gl_fn!(load_fn, "glCheckFramebufferStatus\0" => FnCheckFramebufferStatus)?,

            fnGetUniformLocation: load_gl_fn!(load_fn, "glGetUniformLocation\0" => FnGetUniformLocation)?,
            fnUniform1i: load_gl_fn!(load_fn, "glUniform1i\0" => FnUniform1i)?,
            fnUniform2i: load_gl_fn!(load_fn, "glUniform2i\0" => FnUniform2i)?,
            fnUniform3i: load_gl_fn!(load_fn, "glUniform3i\0" => FnUniform3i)?,
            fnUniform4i: load_gl_fn!(load_fn, "glUniform4i\0" => FnUniform4i)?,
            fnUniform1iv: load_gl_fn!(load_fn, "glUniform1iv\0" => FnUniform1iv)?,
            fnUniform2iv: load_gl_fn!(load_fn, "glUniform2iv\0" => FnUniform2iv)?,
            fnUniform3iv: load_gl_fn!(load_fn, "glUniform3iv\0" => FnUniform3iv)?,
            fnUniform4iv: load_gl_fn!(load_fn, "glUniform4iv\0" => FnUniform4iv)?,
            fnUniform1f: load_gl_fn!(load_fn, "glUniform1f\0" => FnUniform1f)?,
            fnUniform2f: load_gl_fn!(load_fn, "glUniform2f\0" => FnUniform2f)?,
            fnUniform3f: load_gl_fn!(load_fn, "glUniform3f\0" => FnUniform3f)?,
            fnUniform4f: load_gl_fn!(load_fn, "glUniform4f\0" => FnUniform4f)?,
            fnUniform1fv: load_gl_fn!(load_fn, "glUniform1fv\0" => FnUniform1fv)?,
            fnUniform2fv: load_gl_fn!(load_fn, "glUniform2fv\0" => FnUniform2fv)?,
            fnUniform3fv: load_gl_fn!(load_fn, "glUniform3fv\0" => FnUniform3fv)?,
            fnUniform4fv: load_gl_fn!(load_fn, "glUniform4fv\0" => FnUniform4fv)?,
            fnUniformMatrix2fv: load_gl_fn!(load_fn, "glUniformMatrix2fv\0" => FnUniformMatrix2fv)?,
            fnUniformMatrix3fv: load_gl_fn!(load_fn, "glUniformMatrix3fv\0" => FnUniformMatrix3fv)?,
            fnUniformMatrix4fv: load_gl_fn!(load_fn, "glUniformMatrix4fv\0" => FnUniformMatrix4fv)?,
        })
    }

    impl_gl_fn!(fnViewport, Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei));
    impl_gl_fn!(fnClearColor, ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat));
    impl_gl_fn!(fnClear, Clear(mask: GLbitfield));
    impl_gl_fn!(fnEnable, Enable(cap: GLenum));
    impl_gl_fn!(fnDisable, Disable(cap: GLenum));
    impl_gl_fn!(fnAlphaFunc, AlphaFunc(func: GLenum, ref_value: GLclampf));
    impl_gl_fn!(fnBlendFunc, BlendFunc(src: GLenum, dst: GLenum));
    impl_gl_fn!(fnPointSize, PointSize(size: GLfloat));
    impl_gl_fn!(fnLineWidth, LineWidth(width: GLfloat));
    impl_gl_fn!(fnGenTextures, GenTextures(n: GLsizei, textures: *mut GLuint));
    impl_gl_fn!(fnBindTexture, BindTexture(target: GLenum, texture: GLuint));
    impl_gl_fn!(fnTexImage1D, TexImage1D(target: GLenum, level: GLint, internal: GLint, width: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const GLvoid));
    impl_gl_fn!(fnTexImage2D, TexImage2D(target: GLenum, level: GLint, internal: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, r#type: GLenum, pixels: *const GLvoid));
    impl_gl_fn!(fnTexParameterf, TexParameterf(target: GLenum, pname: GLenum, param: GLfloat));
    impl_gl_fn!(fnTexParameterfv, TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat));
    impl_gl_fn!(fnTexParameteri, TexParameteri(target: GLenum, pname: GLenum, param: GLint));
    impl_gl_fn!(fnTexParameteriv, TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint));

    impl_gl_fn!(fnActiveTexture, ActiveTexture(texture: GLenum));

    impl_gl_fn!(fnCreateProgram, CreateProgram() -> GLuint);
    impl_gl_fn!(fnDeleteProgram, DeleteProgram(program: GLuint));
    impl_gl_fn!(fnValidateProgram, ValidateProgram(program: GLuint));
    impl_gl_fn!(fnLinkProgram, LinkProgram(program: GLuint));
    impl_gl_fn!(fnUseProgram, UseProgram(program: GLuint));
    impl_gl_fn!(fnGetProgramiv, GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint));

    impl_gl_fn!(fnCreateShader, CreateShader(shader_type: GLenum) -> GLuint);
    impl_gl_fn!(fnDeleteShader, DeleteShader(shader: GLuint));
    impl_gl_fn!(fnCompileShader, CompileShader(shader: GLuint));
    impl_gl_fn!(fnAttachShader, AttachShader(program: GLuint, shader: GLuint));
    impl_gl_fn!(fnDetachShader, DetachShader(program: GLuint, shader: GLuint));
    impl_gl_fn!(fnShaderSource, ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint));

    impl_gl_fn!(fnGenBuffers, GenBuffers(n: GLsizei, buffers: *mut GLuint));
    impl_gl_fn!(fnBindBuffer, BindBuffer(target: GLenum, buffer: GLuint));
    impl_gl_fn!(fnBufferData, BufferData(target: GLenum, size: usize, data: *const GLvoid, usage: GLenum));
    impl_gl_fn!(fnDeleteBuffers, DeleteBuffers(n: GLsizei, buffers: *const GLuint));

    impl_gl_fn!(fnDrawBuffers, DrawBuffers(n: GLsizei, bufs: *const GLenum));
    impl_gl_fn!(fnDrawArrays, DrawArrays(mode: GLenum, first: GLint, count: GLsizei));

    impl_gl_fn!(fnEnableVertexAttribArray, EnableVertexAttribArray(index: GLuint));
    impl_gl_fn!(fnDisableVertexAttribArray, DisableVertexAttribArray(index: GLuint));
    impl_gl_fn!(fnGenVertexArrays, GenVertexArrays(n: GLsizei, arrays: *mut GLuint));
    impl_gl_fn!(fnDeleteVertexArrays, DeleteVertexArrays(n: GLsizei, arrays: *const GLuint));
    impl_gl_fn!(fnBindVertexArray, BindVertexArray(array: GLuint));
    impl_gl_fn!(fnGetAttribLocation, GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint);
    impl_gl_fn!(fnVertexAttribPointer, VertexAttribPointer(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const GLvoid));

    impl_gl_fn!(fnBindFramebuffer, BindFramebuffer(target: GLenum, framebuffer: GLuint));
    impl_gl_fn!(fnGenFramebuffers, GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint));
    impl_gl_fn!(fnDeleteFramebuffers, DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint));
    impl_gl_fn!(fnFramebufferTexture2D, FramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint));
    impl_gl_fn!(fnCheckFramebufferStatus, CheckFramebufferStatus(target: GLenum) -> GLenum);

    impl_gl_fn!(fnGetUniformLocation, GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint);
    impl_gl_fn!(fnUniform1i, Uniform1i(location: GLint, v0: GLint));
    impl_gl_fn!(fnUniform2i, Uniform2i(location: GLint, v0: GLint, v1: GLint));
    impl_gl_fn!(fnUniform3i, Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint));
    impl_gl_fn!(fnUniform4i, Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint));
    impl_gl_fn!(fnUniform1iv, Uniform1iv(location: GLint, count: GLsizei, value: *const GLint));
    impl_gl_fn!(fnUniform2iv, Uniform2iv(location: GLint, count: GLsizei, value: *const GLint));
    impl_gl_fn!(fnUniform3iv, Uniform3iv(location: GLint, count: GLsizei, value: *const GLint));
    impl_gl_fn!(fnUniform4iv, Uniform4iv(location: GLint, count: GLsizei, value: *const GLint));
    impl_gl_fn!(fnUniform1f, Uniform1f(location: GLint, v0: GLfloat));
    impl_gl_fn!(fnUniform2f, Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat));
    impl_gl_fn!(fnUniform3f, Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat));
    impl_gl_fn!(fnUniform4f, Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat));
    impl_gl_fn!(fnUniform1fv, Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat));
    impl_gl_fn!(fnUniform2fv, Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat));
    impl_gl_fn!(fnUniform3fv, Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat));
    impl_gl_fn!(fnUniform4fv, Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat));
    impl_gl_fn!(fnUniformMatrix2fv, UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat));
    impl_gl_fn!(fnUniformMatrix3fv, UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat));
    impl_gl_fn!(fnUniformMatrix4fv, UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat));
}

mod tests {

	#[test]
	fn test_load_gl_fn() {
		assert_eq!(1, 1);
	}
}