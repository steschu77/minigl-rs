use minigl::opengl as gl;
use std::ffi::CString;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCString,
    ShaderCompileError { name: String, log: String },
    ShaderLinkError { name: String, log: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err = format!("{:?}", self);
        f.write_str(&err)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

const VS_COLOR: &str = r#"
#version 300 es
layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec3 aColor;
out vec3 vertexColor;
void main() {
    gl_Position = vec4(aPosition, 0.0, 1.0);
    vertexColor = aColor;
}"#;

const FS_COLOR: &str = r#"
#version 300 es
in mediump vec3 vertexColor;
out mediump vec4 FragColor;
void main() {
    FragColor = vec4(vertexColor, 1.0);
}"#;

const VS_TEXTURE: &str = r#"
#version 300 es
layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec2 aTexCoord;
out mediump vec2 TexCoord;
void main() {
    gl_Position = vec4(aPosition, 0.0, 1.0);
    TexCoord = aTexCoord;
}"#;

const FS_TEXTURE: &str = r#"
#version 300 es
precision mediump float;
in vec2 TexCoord;
out vec4 FragColor;
uniform sampler2D texture1;
float rand(vec2 n) {
    return fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453);
}
void main() {
    float n0 = rand( gl_FragCoord.xy) - 0.5;
    float n1 = rand(-gl_FragCoord.yx) - 0.5;
    vec2 noise = 0.05f * vec2(n0 * n0, n1 * n1);
    FragColor = texture(texture1, TexCoord.st + noise);
}"#;

struct Triangle {
    gl: gl::OpenGLFunctions,
    color_vao: gl::GLuint,
    texture_vao: gl::GLuint,
    color_program: gl::GLuint,
    texture_program: gl::GLuint,
    fbo: gl::GLuint,
    texture: gl::GLuint,
}

impl Triangle {
    fn new(gl: gl::OpenGLFunctions) -> Self {
        print_opengl_info(&gl);

        let color_vao = create_color_vao(&gl);
        let texture_vao = create_texture_vao(&gl);
        let color_program = create_program(&gl, "color", VS_COLOR, FS_COLOR)
            .expect("Failed to create color program");
        let texture_program = create_program(&gl, "texture", VS_TEXTURE, FS_TEXTURE)
            .expect("Failed to create texture program");

        let (fbo, texture) = create_framebuffer(&gl);

        Self {
            gl,
            color_vao,
            texture_vao,
            color_program,
            texture_program,
            fbo,
            texture,
        }
    }

    fn resize(&self, cx: i32, cy: i32) {
        unsafe { self.gl.Viewport(0, 0, cx, cy) };
    }

    fn render_1st_pass(&self) {
        let gl = &self.gl;

        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl.Disable(gl::DEPTH_TEST);

            gl.ClearColor(0.1, 0.1, 0.1, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.UseProgram(self.color_program);
            gl.BindVertexArray(self.color_vao);
            gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn render_2nd_pass(&self) {
        let gl = &self.gl;

        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl.Disable(gl::DEPTH_TEST);

            gl.UseProgram(self.texture_program);
            gl.BindVertexArray(self.texture_vao);
            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, self.texture);
            gl.DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    fn render(&self) {
        self.render_1st_pass();
        self.render_2nd_pass();
    }
}

fn print_opengl_info(gl: &gl::OpenGLFunctions) {
    unsafe {
        let version = std::ffi::CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_str()
            .unwrap();
        let vendor = std::ffi::CStr::from_ptr(gl.GetString(gl::VENDOR) as *const _)
            .to_str()
            .unwrap();
        let renderer = std::ffi::CStr::from_ptr(gl.GetString(gl::RENDERER) as *const _)
            .to_str()
            .unwrap();

        println!("OpenGL Version: {}", version);
        println!("Vendor: {}", vendor);
        println!("Renderer: {}", renderer);
    }
}

pub fn create_shader(
    gl: &gl::OpenGLFunctions,
    shader_type: gl::GLenum,
    name: &str,
    source: &str,
) -> Result<gl::GLuint> {
    unsafe {
        let Ok(csource) = CString::new(source) else {
            return Err(Error::InvalidCString);
        };
        let shader = gl.CreateShader(shader_type);
        let csource_ptr = csource.as_ptr();
        gl.ShaderSource(shader, 1, &csource_ptr, std::ptr::null());
        gl.CompileShader(shader);

        let mut is_compiled = 0;
        gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut is_compiled);
        if is_compiled == 0 {
            let mut log_length = 0;
            gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
            let mut log = vec![0; log_length as usize];
            gl.GetShaderInfoLog(
                shader,
                log_length,
                std::ptr::null_mut(),
                log.as_mut_ptr() as *mut _,
            );
            let log_str = String::from_utf8_lossy(&log);
            gl.DeleteShader(shader);
            return Err(Error::ShaderCompileError {
                name: name.to_string(),
                log: log_str.to_string(),
            });
        }

        Ok(shader)
    }
}

pub fn create_program(
    gl: &gl::OpenGLFunctions,
    name: &str,
    vs: &str,
    fs: &str,
) -> Result<gl::GLuint> {
    unsafe {
        let vs = create_shader(gl, gl::VERTEX_SHADER, format!("{name}/vertex").as_str(), vs)?;
        let fs = create_shader(gl, gl::FRAGMENT_SHADER, format!("{name}/frag").as_str(), fs)?;

        let program = gl.CreateProgram();
        gl.AttachShader(program, vs);
        gl.AttachShader(program, fs);
        gl.LinkProgram(program);
        gl.DeleteShader(vs);
        gl.DeleteShader(fs);

        let mut is_linked = 0;
        gl.GetProgramiv(program, gl::LINK_STATUS, &mut is_linked);
        if is_linked == 0 {
            let mut log_length = 0;
            gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);
            let mut log = vec![0; log_length as usize];
            gl.GetProgramInfoLog(
                program,
                log_length,
                std::ptr::null_mut(),
                log.as_mut_ptr() as *mut _,
            );
            let log_str = String::from_utf8_lossy(&log);
            gl.DeleteProgram(program);
            return Err(Error::ShaderLinkError {
                name: name.to_string(),
                log: log_str.to_string(),
            });
        }
        Ok(program)
    }
}

fn create_vbo(gl: &gl::OpenGLFunctions, idx: gl::GLuint, size: gl::GLint, data: Vec<gl::GLfloat>) {
    unsafe {
        let mut vbo = 0;
        gl.GenBuffers(1, &mut vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            data.len() * std::mem::size_of::<gl::GLfloat>(),
            data.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl.EnableVertexAttribArray(idx);
        gl.VertexAttribPointer(idx, size, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
    }
}

fn create_color_vao(gl: &gl::OpenGLFunctions) -> gl::GLuint {
    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        let verts = vec![0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
        let colors = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        create_vbo(gl, 0, 2, verts);
        create_vbo(gl, 1, 3, colors);

        vao
    }
}

fn create_texture_vao(gl: &gl::OpenGLFunctions) -> gl::GLuint {
    unsafe {
        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        gl.BindVertexArray(vao);

        let verts = vec![-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
        let texcoords = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        create_vbo(gl, 0, 2, verts);
        create_vbo(gl, 1, 2, texcoords);

        vao
    }
}

fn create_framebuffer(gl: &gl::OpenGLFunctions) -> (gl::GLuint, gl::GLuint) {
    unsafe {
        let mut fbo = 0;

        gl.GenFramebuffers(1, &mut fbo);
        gl.BindFramebuffer(gl::FRAMEBUFFER, fbo);

        let mut texture = 0;
        gl.GenTextures(1, &mut texture);
        gl.BindTexture(gl::TEXTURE_2D, texture);
        gl.TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as i32,
            800,
            600,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            std::ptr::null(),
        );
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl.FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT,
            gl::TEXTURE_2D,
            texture,
            0,
        );

        gl.DrawBuffers(1, [gl::COLOR_ATTACHMENT].as_ptr());

        let status = gl.CheckFramebufferStatus(gl::FRAMEBUFFER);
        if status != gl::FRAMEBUFFER_COMPLETE {
            panic!("Framebuffer is not complete");
        }

        (fbo, texture)
    }
}

// Default impl for OpenGLContext without OS specific code.
// This will cause an OpenGLLoadError panic when used.
// This is useful for starting development on a new platform.
#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "linux"))]
mod none {
    use super::*;
    use minigl::none::*;

    pub fn main() {
        let none = NoneGLContext::from_nothing().unwrap();
        let gl = none.load().unwrap();
        let triangle = Triangle::new(gl);
        triangle.resize(800, 600);

        loop {
            triangle.render();
            none.swap_buffers();
        }
    }
}

#[cfg(target_os = "windows")]
mod win32 {

    use minigl::win32::{window::*, *};
    use windows::Win32::{Foundation::*, Graphics::Gdi::*, UI::WindowsAndMessaging::*};

    struct WindowProcParams {}

    struct WindowProcData {
        win32: Win32GLContext,
        triangle: super::Triangle,
    }

    impl WindowProcTrait for WindowProcData {
        type Params = WindowProcParams;

        fn create(hwnd: HWND, _params: &WindowProcParams) -> Self {
            let win32 = Win32GLContext::from_hwnd(hwnd).unwrap();
            let gl = win32.load().unwrap();
            let triangle = super::Triangle::new(gl);
            Self { win32, triangle }
        }

        fn on_destroy(&mut self) -> LRESULT {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }

        fn on_size(&mut self, cx: i32, cy: i32) -> LRESULT {
            self.triangle.resize(cx, cy);
            LRESULT(0)
        }

        fn on_paint(&mut self, _hdc: HDC, _ps: &PAINTSTRUCT) {
            self.triangle.render();
            self.win32.swap_buffers();
        }
    }

    pub fn main() {
        let hwnd = WindowProc::<WindowProcData>::create(
            "OpenGL Triangle",
            "OpenGLWindow",
            WS_POPUP | WS_VISIBLE,
            &WindowProcParams {},
        );

        if hwnd.is_ok() {
            run_message_loop();
        }
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use super::*;
    use minigl::linux::*;

    pub fn main() {
        let display = unsafe { x11::xlib::XOpenDisplay(std::ptr::null()) };
        let screen = unsafe { x11::xlib::XDefaultScreen(display) };
        let root = unsafe { x11::xlib::XRootWindow(display, screen) };

        let win = unsafe { x11::xlib::XCreateSimpleWindow(display, root, 0, 0, 800, 600, 0, 0, 0) };

        unsafe {
            x11::xlib::XSelectInput(
                display,
                win,
                x11::xlib::ExposureMask | x11::xlib::KeyPressMask,
            );
            x11::xlib::XMapWindow(display, win);
        }

        let context = unsafe { LinuxGLContext::from_window(display, screen, win).unwrap() };
        let gl = context.load().unwrap();
        let triangle = Triangle::new(gl);
        triangle.resize(800, 600);

        loop {
            unsafe {
                let mut event: x11::xlib::XEvent = std::mem::zeroed();
                x11::xlib::XNextEvent(display, &mut event);
                match event.type_ {
                    x11::xlib::Expose => {
                        triangle.render();
                        context.swap_buffers();
                    }
                    x11::xlib::KeyPress => break,
                    _ => (),
                }
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "linux"))]
pub fn main() {
    none::main();
}

#[cfg(target_os = "windows")]
pub fn main() {
    win32::main();
}

#[cfg(target_os = "linux")]
pub fn main() {
    linux::main();
}
