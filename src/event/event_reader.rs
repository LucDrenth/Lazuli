use std::sync::mpsc::TryRecvError;

pub struct EventReader<T: Clone + Sync> {
    bus_reader: bus::BusReader<T>,
}

impl<T: Clone + Sync> EventReader<T> {
    pub fn new(bus_reader: bus::BusReader<T>) -> Self {
        Self { bus_reader }
    }

    pub fn read(&mut self) -> Vec<T> {
        let mut result = Vec::<T>::new();

        loop {
            match self.bus_reader.try_recv() {
                Ok(value) => {
                    result.push(value);
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
                Err(TryRecvError::Disconnected) => {} // the sender has hungup, maybe the event system got destroyed?
            }
        }

        return result;
    }
}