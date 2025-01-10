use audio_visualizer as audvis;
use audvis::cli_parser::Args;
use audvis::prelude::*;
use audvis::setup;
use audvis::window;

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Audio Visualizer...");

    pollster::block_on(window::run());
    info!("Exiting Audio Visualizer...");
}
