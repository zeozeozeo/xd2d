use xd2d::{App, Xd, XdResult};

struct ExampleApp;

impl App for ExampleApp {
    fn update(&mut self, _xd: &mut Xd) {}

    fn draw(&mut self, _xd: &mut Xd) {}
}

fn main() -> XdResult<()> {
    env_logger::init();
    Xd::run(ExampleApp)
}
