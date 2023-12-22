use glutin::{
    dpi::{PhysicalSize, Position, Size},
    window::{Fullscreen, Icon},
};

#[derive(Debug, Clone)]
pub struct WindowSettings {
    pub inner_size: Option<Size>,
    pub min_inner_size: Option<Size>,
    pub max_inner_size: Option<Size>,
    pub position: Option<Position>,
    pub resizable: bool,
    pub title: String,
    pub fullscreen: Option<Fullscreen>,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub decorations: bool,
    pub always_on_top: bool,
    pub window_icon: Option<Icon>,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            inner_size: Some(Size::Physical(PhysicalSize::new(1280, 720))),
            min_inner_size: None,
            max_inner_size: None,
            position: None,
            resizable: true,
            title: "xd2d".to_string(),
            fullscreen: None,
            maximized: false,
            visible: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            window_icon: None,
        }
    }
}
