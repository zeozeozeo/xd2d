use xd2d::{App, Xd, XdResult};

#[derive(Default)]
struct ExampleApp;

impl App for ExampleApp {
    fn update(&mut self) {}

    fn draw(&mut self) {}
}

fn main() -> XdResult<()> {
    env_logger::init();
    Xd::default().run(ExampleApp::default())
}
