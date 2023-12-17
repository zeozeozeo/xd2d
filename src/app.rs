pub trait App {
    fn init(&mut self) {}

    fn update(&mut self);

    fn draw(&mut self);
}
