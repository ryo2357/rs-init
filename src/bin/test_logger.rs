use log::{debug, error, info, trace, warn};

fn main() {
    mylogger::init();

    trace!("trace message");
    debug!("debug message");
    info!("informational message");
    warn!("warning message");
    error!("this is an error {}", "message");
}
