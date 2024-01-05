use crate::{App, Backend, WindowSettings, XdResult};
pub struct Xd {
    pub(crate) width: u32,  // modified in backend
    pub(crate) height: u32, // modified in backend
    pub window_settings: WindowSettings,
    pub ignore_swapbuffers: bool,
}

impl Default for Xd {
    fn default() -> Self {
        Self::new()
    }
}

impl Xd {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            window_settings: WindowSettings::default(),
            ignore_swapbuffers: false,
        }
    }

    pub fn run<A: App + 'static>(app: A) -> XdResult<()> {
        Backend::run(Self::default(), app)
    }

    /// Window width in physical pixels.
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Window height in physical pixels.
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }
}
