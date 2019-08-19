use crate::{SendError,RecvError, Sender, Receiver};

pub struct StdIOChannel<T> {
    pub stdin: std::io::Stdin,
    pub stdout: std::io::Stdout,
    resource_type: std::marker::PhantomData<T>,
}

impl<T> StdIOChannel<T> {
    pub fn new () -> Option<StdIOChannel<T>> {
        Some(StdIOChannel {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            resource_type: std::marker::PhantomData,
        })
    }
}

impl<T> Receiver<T> for StdIOChannel<T> {
    fn recv(&mut self) -> Result<T, RecvError>
        where for<'de> T: serde::Deserialize<'de>
    {
        let stdin = self.stdin.lock();
        let answer: Result<T,std::boxed::Box<bincode::ErrorKind>> = bincode::deserialize_from(stdin);
        match answer {
            Ok(recv_value) => {
                Ok(recv_value)
            }
            Err(_) => {
                Err(RecvError)
            }
        }
    }
}

impl<T> Sender<T> for StdIOChannel<T> {
    fn send(&mut self, t: T) -> Result<(), SendError<T>>
        where T: serde::Serialize
    {
        let stdout = self.stdout.lock();
        match bincode::serialize_into(stdout, &t) {
            Ok(_) =>
                Ok(()),
            Err(_) => {
                Err(SendError(t))
            }
        }
    }
}
