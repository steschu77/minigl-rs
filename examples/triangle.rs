use minigl::opengl as gl;

const VS_COLOR: &str = r#"
#version 330 core
layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec3 aColor;
out vec3 vertexColor;
void main() {
    gl_Position = vec4(aPosition, 0.0, 1.0);
    vertexColor = aColor;
}"#;

const FS_COLOR: &str = r#"
#version 330 core
in vec3 vertexColor;
out vec4 FragColor;
void main() {
    FragColor = vec4(vertexColor, 1.0);
}"#;

const VS_TEXTURE: &str = r#"
#version 330 core
layout (location = 0) in vec2 aPosition;
layout (location = 1) in vec2 aTexCoord;
out vec2 TexCoord;
void main() {
    gl_Position = vec4(aPosition, 0.0, 1.0);
    TexCoord = aTexCoord;
}"#;

const FS_TEXTURE: &str = r#"
#version 330 core
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
        let color_vao = unsafe { create_color_vao(&gl) };
        let texture_vao = unsafe { create_texture_vao(&gl) };
        let color_program = unsafe { create_program(&gl, VS_COLOR, FS_COLOR) };
        let texture_program = unsafe { create_program(&gl, VS_TEXTURE, FS_TEXTURE) };
        let (fbo, texture) = unsafe { create_framebuffer(&gl) };

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

unsafe fn create_shader(
    gl: &gl::OpenGLFunctions,
    shader_type: gl::GLenum,
    source: &str,
) -> gl::GLuint {
    let source = source.to_string() + "\0";
    let shader = gl.CreateShader(shader_type);
    gl.ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
    gl.CompileShader(shader);
    shader
}

unsafe fn create_program(gl: &gl::OpenGLFunctions, vs: &str, fs: &str) -> gl::GLuint {
    let vs = create_shader(gl, gl::VERTEX_SHADER, vs);
    let fs = create_shader(gl, gl::FRAGMENT_SHADER, fs);

    let program = gl.CreateProgram();
    gl.AttachShader(program, vs);
    gl.AttachShader(program, fs);
    gl.LinkProgram(program);
    gl.DeleteShader(vs);
    gl.DeleteShader(fs);
    program
}

unsafe fn create_vbo(
    gl: &gl::OpenGLFunctions,
    idx: gl::GLuint,
    size: gl::GLint,
    data: Vec<gl::GLfloat>,
) {
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

unsafe fn create_color_vao(gl: &gl::OpenGLFunctions) -> gl::GLuint {
    let mut vao = 0;
    gl.GenVertexArrays(1, &mut vao);
    gl.BindVertexArray(vao);

    let verts = vec![0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
    let colors = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    create_vbo(gl, 0, 2, verts);
    create_vbo(gl, 1, 3, colors);

    vao
}

unsafe fn create_texture_vao(gl: &gl::OpenGLFunctions) -> gl::GLuint {
    let mut vao = 0;
    gl.GenVertexArrays(1, &mut vao);
    gl.BindVertexArray(vao);

    let verts = vec![-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
    let texcoords = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    create_vbo(gl, 0, 2, verts);
    create_vbo(gl, 1, 2, texcoords);

    vao
}

unsafe fn create_framebuffer(gl: &gl::OpenGLFunctions) -> (gl::GLuint, gl::GLuint) {
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

// Default impl for OpenGLContext without OS specific code.
// This will cause an OpenGLLoadError panic when used.
// This is useful for starting development on a new platform.
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

pub fn main() {
    none::main();
}
