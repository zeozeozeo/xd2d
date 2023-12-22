use crate::Xd;

/// An application trait that defines callbacks for initializing, updating,
/// and drawing to the screen.
pub trait App {
    /// Called just after the window gets opened.
    #[allow(unused_variables)]
    fn init(&mut self, xd: &mut Xd) {}

    fn update(&mut self, xd: &mut Xd);

    fn draw(&mut self, xd: &mut Xd);
}
