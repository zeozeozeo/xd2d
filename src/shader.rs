use glow::{Context, HasContext, NativeProgram};

pub enum ShaderType {
    /// Vertex shader.
    Vertex,
    /// Pixel (fragment) shader.
    Pixel,
}

pub struct Shader {
    program: NativeProgram,
}

unsafe fn create_program(gl: glow::Context, source: &str, typ: ShaderType) -> NativeProgram {
    let program = gl.create_program().expect("couldn't create program");

    let shader_type = match typ {
        ShaderType::Vertex => glow::VERTEX_SHADER,
        ShaderType::Pixel => glow::FRAGMENT_SHADER,
    };

    let shader = gl
        .create_shader(shader_type)
        .expect("couldn't create shader");
    gl.shader_source(shader, source);
    gl.compile_shader(shader);
    if !gl.get_shader_compile_status(shader) {
        panic!("{}", gl.get_shader_info_log(shader));
    }
    gl.attach_shader(program, shader);

    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
    }

    // the shader is now linked to the program, we can detach them from the program
    // and delete them
    gl.detach_shader(program, shader);
    gl.delete_shader(shader);

    program
}

impl Shader {
    pub fn new(gl: Context, source: &str, typ: ShaderType) -> Self {
        let program = unsafe { create_program(gl, source, typ) };
        Self { program }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        Context::new
    }
}
