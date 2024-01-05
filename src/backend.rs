use crate::{App, WindowSettings, Xd, XdResult};
use glow::HasContext;

pub struct Backend;

impl Backend {
    pub fn run<A: App + 'static>(xd: Xd, app: A) -> XdResult<()> {
        let mut backend = Self;

        #[cfg(not(target_arch = "wasm32"))]
        return backend.run_winit(app, xd);

        #[cfg(target_arch = "wasm32")]
        return backend.run_wasm(app, xd);
    }

    fn create_window_builder(settings: WindowSettings) -> glutin::window::WindowBuilder {
        let mut builder = glutin::window::WindowBuilder::new()
            .with_resizable(settings.resizable)
            .with_title(settings.title)
            .with_fullscreen(settings.fullscreen)
            .with_maximized(settings.maximized)
            .with_visible(settings.visible)
            .with_transparent(settings.transparent)
            .with_decorations(settings.decorations)
            .with_always_on_top(settings.always_on_top)
            .with_window_icon(settings.window_icon);
        if let Some(inner_size) = settings.inner_size {
            builder = builder.with_inner_size(inner_size);
        }
        if let Some(min_inner_size) = settings.min_inner_size {
            builder = builder.with_min_inner_size(min_inner_size);
        }
        if let Some(max_inner_size) = settings.max_inner_size {
            builder = builder.with_max_inner_size(max_inner_size);
        }
        if let Some(position) = settings.position {
            builder = builder.with_position(position);
        }
        builder
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn run_winit<A: App + 'static>(&mut self, mut app: A, mut xd: Xd) -> XdResult<()> {
        use glutin::{
            event::{Event, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            ContextBuilder, GlRequest,
        };

        let event_loop = EventLoop::new();
        let window = Self::create_window_builder(xd.window_settings.clone());

        // create glutin gl context from the window builder
        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Latest)
            .build_windowed(window, &event_loop)?;

        // make it current and load function pointers
        let gl_context = unsafe { gl_context.make_current() }.map_err(|e| e.1)?;
        let gl = unsafe { glow::Context::from_loader_function(|s| gl_context.get_proc_address(s)) };

        // set the initial window size
        {
            let inner_size = gl_context.window().inner_size();
            (xd.width, xd.height) = (inner_size.width, inner_size.height);
        }

        // call init()
        app.init(&mut xd);

        // start event loop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::LoopDestroyed => (),
                Event::MainEventsCleared => gl_context.window().request_redraw(),
                Event::RedrawRequested(_) => {
                    unsafe {
                        gl.clear(glow::COLOR_BUFFER_BIT);
                    }
                    app.draw(&mut xd);
                    if !xd.ignore_swapbuffers {
                        gl_context.swap_buffers().unwrap();
                    }
                    // gl_context.window().set_visible(true);
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        // resize window
                        gl_context.resize(*physical_size);

                        // update size, we grab it again incase glutin has changed it
                        let inner_size = gl_context.window().inner_size();
                        (xd.width, xd.height) = (inner_size.width, inner_size.height);
                    }
                    // TODO: free gl resources
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn run_wasm(&mut self, mut app: A, mut xd: Xd) -> XdResult<()> {
        Ok(())
    }
}
