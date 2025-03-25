use crossbeam::channel::{bounded, Receiver, Sender};

use super::Control;

pub struct DroneState {
    pub controls_sender: Sender<Control>,
    pub controls_receiver: Receiver<Control>,
}

impl DroneState {
    pub fn new() -> Self {
        let (controls_sender, controls_receiver) = bounded(1);

        return Self {
            controls_sender,
            controls_receiver,
        };
    }
}
