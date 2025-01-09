// use winit::{
//     event::*,
//     event_loop::EventLoopBuilder,
//     keyboard::{KeyCode, PhysicalKey},
//     window::WindowBuilder
// };
use audio_visualizer as audvis;
use audvis::cli_parser::Args;
use audvis::prelude::*;
use audvis::setup;

fn main() {
    let _args: Args = setup::setup_program();

    debug!("CWD = {:?}", std::env::current_dir().unwrap());
}
