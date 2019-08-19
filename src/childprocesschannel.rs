use crate::{SendError,RecvError, Sender, Receiver};

pub struct ChildProcessChannel<T> {
    pub child_process: std::process::Child,
    resource_type: std::marker::PhantomData<T>,
}

impl<T> ChildProcessChannel<T> {
    pub fn new (child_process: std::process::Child) -> Option<ChildProcessChannel<T>> {
        Some(ChildProcessChannel {
            child_process: child_process,
            resource_type: std::marker::PhantomData,
        })
    }
}

impl<T> Receiver<T> for ChildProcessChannel<T> {
    fn recv(&mut self) -> Result<T, RecvError>
        where for<'de> T: serde::Deserialize<'de>
    {
        let stdout_option = self.child_process.stdout.as_mut();
        match stdout_option {
            None => {
                Err(RecvError)
            },
            Some(stdout) => {
                let answer: Result<T,std::boxed::Box<bincode::ErrorKind>> = bincode::deserialize_from(stdout);
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
    }
}

impl<T> Sender<T> for ChildProcessChannel<T> {
    fn send(&mut self, t: T) -> Result<(), SendError<T>>
        where T: serde::Serialize
    {
        let stdin_option = self.child_process.stdin.as_mut();
        match stdin_option {
            None => {
                Err(SendError(t))
            },
            Some(stdin) => {
                match bincode::serialize_into(stdin, &t) {
                    Ok(_) =>
                        Ok(()),
                    Err(_) => {
                        Err(SendError(t))
                    }
                }
            }
        }
    }
}
