use crate::{App, XdResult};
use glow::HasContext;

#[derive(Default)]
pub struct Xd;

impl Xd {
    pub fn run<A: App + 'static>(&mut self, app: A) -> XdResult<()> {
        #[cfg(not(target_arch = "wasm32"))]
        return self.run_winit(app);

        #[cfg(target_arch = "wasm32")]
        return self.run_wasm();
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn run_winit<A: App + 'static>(&mut self, mut app: A) -> XdResult<()> {
        use glutin::{
            event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
            ContextBuilder, GlRequest,
        };

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().with_title("xd2d");

        // create glutin gl context from the window builder
        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Latest)
            .build_windowed(window, &event_loop)?;

        // make it current and load function pointers
        let gl_context = unsafe { gl_context.make_current() }.map_err(|e| e.1)?;
        let gl = unsafe { glow::Context::from_loader_function(|s| gl_context.get_proc_address(s)) };

        // start event loop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::LoopDestroyed => return,
                Event::MainEventsCleared => gl_context.window().request_redraw(),
                Event::RedrawRequested(_) => {
                    unsafe {
                        gl.clear(glow::COLOR_BUFFER_BIT);
                    }
                    app.draw();
                    gl_context.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => gl_context.resize(*physical_size),
                    // TODO: free gl resources
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn run_wasm(&mut self) -> XdResult<()> {
        Ok(())
    }
}
