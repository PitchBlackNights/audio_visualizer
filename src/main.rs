use audio_visualizer as audvis;
use audvis::cli_parser::Args;
use audvis::window;
use audvis::prelude::*;
use audvis::setup;

fn main() {
    let _args: Args = setup::setup_logger_and_args();
    info!("Starting Audio Visualizer...");

    window::run();
    info!("Exiting Audio Visualizer...");
}
