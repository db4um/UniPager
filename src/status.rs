use std::sync::{Mutex, RwLock};
use pocsag::TimeSlots;
use frontend::Responder;

lazy_static! {
    pub static ref STATUS: RwLock<Status> = RwLock::new(Status::new());
    pub static ref RESPONDER: Mutex<Option<Responder>> = Mutex::new(None);
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct Status {
    pub connected: bool,
    pub transmitting: bool,
    pub timeslots: TimeSlots
}

impl Status {
    pub fn new() -> Status {
        Status {
            connected: false,
            transmitting: false,
            timeslots: TimeSlots::new()
        }
    }
}

pub fn subscribe(responder: Responder) {
    *RESPONDER.lock().unwrap() = Some(responder);
}

pub fn get() -> Status {
    STATUS.read().unwrap().clone()
}

macro_rules! status {
    ($key:ident: $value:expr) => ({
        let mut status = $crate::status::STATUS.write().unwrap();
        if status.$key != $value {
            status.$key = $value;
            let res = $crate::status::RESPONDER.lock().unwrap();
            if let Some(ref res) = *res {
                res.send($crate::frontend::Response::Status(status.clone()));
            }
        }
    });
}
