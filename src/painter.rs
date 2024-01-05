use crate::{vec2, Color, Mat2x3, Rect, Vec2};
use glow::{
    Context, HasContext, NativeBuffer, VertexArray, ARRAY_BUFFER, BLEND, CULL_FACE, DEPTH_TEST,
    FUNC_ADD, ONE, ONE_MINUS_SRC_ALPHA, SCISSOR_TEST, SRC_ALPHA, STENCIL_TEST,
};

const DEFAULT_MAX_VERTICES: usize = 65536;
const DEFAULT_MAX_COMMANDS: usize = 16384;

#[repr(C)]
pub struct Vertex {
    position: Vec2,
    texcoord: Vec2,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub enum PrimitiveType {
    #[default]
    Triangles,
    Points,
    Lines,
    TriangleStrip,
    LineStrip,
}

#[derive(Clone, Copy)]
pub enum Command {
    None,
    /// Clear the viewport with a given color.
    Clear(Color),
    /// Scissor the viewport to the given rectangle.
    Clip(Rect),
    /// Drawcall.
    Draw {
        prim: PrimitiveType,
        vertex_index: u32,
        num_vertices: u32,
    },
}

pub struct Painter {
    pub width: u32,
    pub height: u32,
    pub proj: Mat2x3,
    pub transform: Mat2x3,
    pub mvp: Mat2x3,
    pub commands: Vec<Command>,
    pub vertices: Vec<Vertex>,
    /// GL vertex buffer object.
    vbo: Option<NativeBuffer>,
}

impl Default for Painter {
    fn default() -> Self {
        Self::new(0, 0, DEFAULT_MAX_COMMANDS, DEFAULT_MAX_VERTICES)
    }
}

impl Painter {
    pub fn new(width: u32, height: u32, num_commands: usize, num_vertices: usize) -> Self {
        Self {
            width,
            height,
            proj: Mat2x3::default(),
            transform: Mat2x3::IDENTITY,
            mvp: Mat2x3::default(),
            commands: Vec::with_capacity(num_commands),
            vertices: Vec::with_capacity(num_vertices),
            vbo: None,
        }
    }

    pub fn begin(&mut self, width: u32, height: u32) {
        (self.width, self.height) = (width, height);
        self.commands.clear();
        self.vertices.clear();
        self.proj = Mat2x3::default_proj(self.width as f32, self.height as f32);
        self.transform = Mat2x3::IDENTITY;
        self.mvp = self.proj;
    }

    pub fn allocate_vertices(&mut self, num: usize) -> &mut [Vertex] {
        self.vertices.reserve(num);
        let len = self.vertices.len();
        &mut self.vertices[len..=len + num]
    }

    #[inline]
    pub fn clear(&mut self, color: impl Into<Color>) {
        self.commands.push(Command::Clear(color.into()));
    }

    pub fn queue_draw(&mut self, prim: PrimitiveType, vertex_index: usize, num_vertices: usize) {
        self.commands.push(Command::Draw {
            prim,
            vertex_index: vertex_index as u32,
            num_vertices: num_vertices as u32,
        });
    }

    pub fn filled_rects(&mut self, rects: &[Rect]) {
        let mvp = self.mvp.clone();
        let vertex_index = self.vertices.len();
        let num_vertices = rects.len() * 6;
        let vertices = self.allocate_vertices(num_vertices);
        for (i, rect) in rects.iter().enumerate() {
            let mut quad = [
                vec2(rect.x, rect.y + rect.h),          // bottom left
                vec2(rect.x + rect.w, rect.y + rect.h), // bottom right
                vec2(rect.x + rect.w, rect.y),          // top right
                vec2(rect.x, rect.y),                   // top left
            ];
            mvp.transform_vec2s(&mut quad);

            const TEXCOORD_QUAD: [Vec2; 4] = [
                vec2(0.0, 1.0), // bottom left
                vec2(1.0, 1.0), // bottom right
                vec2(1.0, 0.0), // top right
                vec2(0.0, 0.0), // top left
            ];

            // make a quad composed of 2 triangles
            vertices[0 * (i + 1)].position = quad[0];
            vertices[0 * (i + 1)].texcoord = TEXCOORD_QUAD[0];
            vertices[1 * (i + 1)].position = quad[1];
            vertices[1 * (i + 1)].texcoord = TEXCOORD_QUAD[1];
            vertices[2 * (i + 1)].position = quad[2];
            vertices[2 * (i + 1)].texcoord = TEXCOORD_QUAD[2];
            vertices[3 * (i + 1)].position = quad[3];
            vertices[3 * (i + 1)].texcoord = TEXCOORD_QUAD[3];
            vertices[4 * (i + 1)].position = quad[0];
            vertices[4 * (i + 1)].texcoord = TEXCOORD_QUAD[0];
            vertices[5 * (i + 1)].position = quad[2];
            vertices[5 * (i + 1)].texcoord = TEXCOORD_QUAD[2];
        }

        self.queue_draw(PrimitiveType::Triangles, vertex_index, num_vertices);
    }

    #[inline]
    pub fn filled_rect(&mut self, rect: Rect) {
        self.filled_rects(&[rect]);
    }

    pub unsafe fn gl_init(&mut self, gl: Context) {
        // construct vertex buffer object
        self.vbo = Some(gl.create_buffer().unwrap());
    }

    pub unsafe fn setup_gl_render_state(&mut self, gl: Context) {
        // enable alpha blending, disable face culling, disable depth testing, enable scissor
        gl.enable(BLEND);
        gl.blend_equation(FUNC_ADD);
        gl.blend_func_separate(SRC_ALPHA, ONE_MINUS_SRC_ALPHA, ONE, ONE_MINUS_SRC_ALPHA);
        gl.disable(CULL_FACE);
        gl.disable(DEPTH_TEST);
        gl.disable(STENCIL_TEST);
        gl.enable(SCISSOR_TEST);

        gl.bind_buffer(ARRAY_BUFFER, self.vbo);
        // gl.enable_vertex_attrib_array(0);
    }

    pub unsafe fn gl_render(gl: Context) {}
}
