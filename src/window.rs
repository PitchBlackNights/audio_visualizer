#![allow(unused_must_use)]

use crate::prelude::*;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

pub fn run() {
    trace!("Initializing EventLoop");
    let event_loop = EventLoop::new().unwrap();
    trace!("Initializing Window");
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    trace!("Running EventLoop");
    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                trace!("Exiting EventLoop");
                control_flow.exit()
            }
            _ => {}
        },
        _ => {}
    });
}
