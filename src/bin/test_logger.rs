use log::{debug, error, info, warn};

extern crate mylogger;
fn main() {
    mylogger::init();

    debug!("debug message");
    info!("informational message");
    warn!("warning message");
    error!("this is an error {}", "message");
}
