use rdev::{simulate, EventType, SimulateError};
pub fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => {}
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
}
