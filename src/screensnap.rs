#![windows_subsystem = "windows"]

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    keyboard::{Key, NamedKey},
};

mod context;
use context::AppContext;

pub struct Drag {
    start: (f64, f64),
    end: Option<(f64, f64)>,
}

impl Drag {
    fn coords(&self) -> Option<((u32, u32), (u32, u32))> {
        let end = self.end?;
        let (start_x, start_y) = (self.start.0 as u32, self.start.1 as u32);
        let (end_x, end_y) = (end.0 as u32, end.1 as u32);

        let (min_x, max_x) = (start_x.min(end_x), start_x.max(end_x));
        let (min_y, max_y) = (start_y.min(end_y), start_y.max(end_y));
        Some(((min_x, min_y), (max_x, max_y)))
    }
}

pub struct Selection {
    start: (f64, f64),
    end: (f64, f64),
}

impl Selection {
    

    fn coords(&self) -> ((u32, u32), (u32, u32)) {
        let (start_x, start_y) = (self.start.0, self.start.1);
        let (end_x, end_y) = (self.end.0, self.end.1);

        let (min_x, max_x) = (start_x.min(end_x).ceil(), start_x.max(end_x).floor());
        let (min_y, max_y) = (start_y.min(end_y).ceil(), start_y.max(end_y).floor());
        ((min_x as u32, min_y as u32), (max_x as u32, max_y as u32))
    }
}

struct App {
    context: Option<AppContext>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let context = AppContext::new(event_loop).expect("Could not start context");
        self.context = Some(context);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(context) = &mut self.context else {
            return;
        };
        if id != context.window_id() {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                context.draw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                context.update_mouse_position(position.x, position.y);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        logical_key: key,
                        ..
                    },
                ..
            } => match (state, key) {
                (ElementState::Pressed, Key::Named(NamedKey::Escape)) => {
                    event_loop.exit();
                    context.destroy();
                }

                // space to copy to clipboard
                (ElementState::Pressed, Key::Named(NamedKey::Space)) => {
                    context.hide_window();
                    context.save_locally();
                    event_loop.exit();
                }
                _ => {}
            },
            WindowEvent::MouseInput { state, button, .. } => match (state, button) {
                (ElementState::Pressed, MouseButton::Left) => context.start_drag(),
                (ElementState::Released, MouseButton::Left) => context.end_drag(),
                (_, MouseButton::Right) => context.cancel_drag(),
                _ => {}
            },
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

fn screensnap() -> anyhow::Result<()> {
    let mut app = App { context: None };
    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.run_app(&mut app)?;
    Ok(())
}
