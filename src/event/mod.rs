use rdev::{simulate, EventType, SimulateError};
// use std::{thread, time};
pub fn send(event_type: &EventType) {
    // let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => {
            println!("We sent {:?}", event_type);
        }
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    // thread::sleep(delay);
}
