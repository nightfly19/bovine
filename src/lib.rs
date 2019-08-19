extern crate bincode;
extern crate serde;

pub mod childprocesschannel;
pub mod stdiochannel;

#[derive(Debug)]
pub struct SendError<T>(pub T);

#[derive(Debug)]
pub struct RecvError;

pub trait Sender<T> {
    fn send(&mut self, t: T) -> Result<(), SendError<T>>
        where T: serde::Serialize;
}

pub trait Receiver<T> {
    fn recv(&mut self) -> Result<T, RecvError>
        where for<'de> T: serde::Deserialize<'de>;
}

