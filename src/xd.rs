use crate::{App, Backend, Color, Rect, Vec2, WindowSettings, XdResult};

pub struct Vertex {
    position: Vec2,
    texcoord: Vec2,
}

pub enum Command {
    /// Clear the viewport with a given color.
    Clear(Color),
    /// Scissor the viewport to the given rectangle.
    Clip(Rect),
    /// Drawcall.
    Draw {
        vertex_index: u32,
        num_vertices: u32,
    },
}

#[derive(Default)]
pub struct Xd {
    pub window_settings: WindowSettings,
    pub ignore_swapbuffers: bool,
    pub commands: Vec<Command>,
}

impl Xd {
    pub fn run<A: App + 'static>(app: A) -> XdResult<()> {
        Backend::run(Self::default(), app)
    }
}
