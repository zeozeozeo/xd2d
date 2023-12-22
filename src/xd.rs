use crate::{App, Backend, Color, WindowSettings, XdResult};

pub enum Command {
    Clear(Color),
}

#[derive(Debug, Default)]
pub struct Xd {
    pub window_settings: WindowSettings,
    pub ignore_swapbuffers: bool,
}

impl Xd {
    pub fn run<A: App + 'static>(app: A) -> XdResult<()> {
        Backend::run(Self::default(), app)
    }
}
