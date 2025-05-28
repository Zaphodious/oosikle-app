use anyhow::{Error, Result};
use mlua::Thread;
use std::mem::replace;
use std::sync::mpsc;
use std::thread;
use uuid;

pub type RawMessenger<T> = Option<Box<dyn FnOnce(&mut T) -> Result<()> + Send + 'static>>;
type ShrineDestroyingFunction = Box<dyn FnOnce() -> Result<()> + 'static>;

#[derive(Debug)]
pub struct Miko<T> {
    chan: mpsc::Sender<RawMessenger<T>>,
}


impl<T> Clone for Miko<T> {
    fn clone(&self) -> Self {
        Miko {
            chan: self.chan.clone()
        }
    }
    
}

pub struct ShrineDestroyer(Option<ShrineDestroyingFunction>, thread::Thread);

impl<T> Miko<T>
where
    T: 'static,
{
    pub fn build_shrine(
        label: &str,
        kami_summoner: impl FnOnce() -> Result<T> + Send + 'static,
    ) -> Result<(Miko<T>, ShrineDestroyer)> {
        let (chan, rx) = mpsc::channel::<RawMessenger<T>>();
        let b =
            thread::Builder::new().name(format!("miko_shrine_{}_{}", label, uuid::Uuid::new_v4()));

        let shrine_handle: thread::JoinHandle<()> = b.spawn(move || {
            let mut kami = kami_summoner().expect("Failure getting the value");
            println!("Setting up thread {:?}", thread::current().name());
            for fn_package in rx {
                if let Some(the_fn) = fn_package {
                    match the_fn(&mut kami) {
                        Ok(_) => {
                            println!("A function completed successfully");
                            continue;
                        }
                        Err(n) => {
                            println!("There has been an error: {:?}", n);
                            continue;
                        },
                    };
                } else {
                    break;
                };
            }
        })?;
        let chanclone = chan.clone();
        let shrine = shrine_handle.thread().clone();
        Ok((
            Miko { chan },
            ShrineDestroyer(Some(Box::new(move || {
                chanclone.send(None).unwrap();
                shrine_handle.join().unwrap();
                Ok(())
            })), shrine),
        ))
    }

    pub fn send_raw_messenger(
        &self,
        the_fn: impl FnOnce(&mut T) -> Result<()> + Send + 'static,
    ) -> Result<()> {
        let res = self.chan.send(Some(Box::new(the_fn)));
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                // The error won't send, so we wrap the message
                Err(anyhow::anyhow!("There was an error: {:?}", e))
            }
        }
    }

    pub fn send_mutating_messenger_get_channel<R: Send + 'static>(
        &self,
        messenger: impl FnOnce(&mut T) -> Result<R> + Send + 'static,
    ) -> Result<mpsc::Receiver<R>> {
        let (tx, rx) = mpsc::channel::<R>();
        if let Err(e) = self.send_raw_messenger(move |kami| {
            let res = messenger(kami)?;
            tx.send(res).unwrap();
            Ok(())
        }) {
            // The error won't send, so we wrap the message
            Err(anyhow::anyhow!("We've created an error: {:?}", e))
        } else {
            Ok(rx)
        }
    }

    pub fn send_messenger_get_channel<R: Send + 'static>(
        &self,
        messenger: impl FnOnce(&T) -> Result<R> + Send + 'static,
    ) -> Result<mpsc::Receiver<R>> {
        let (tx, rx) = mpsc::channel::<R>();
        if let Err(e) = self.send_raw_messenger(move |kami| {
            let res = messenger(kami)?;
            tx.send(res).unwrap();
            Ok(())
        }) {
            // The error won't send, so we wrap the message
            Err(anyhow::anyhow!("We've made an error: {:?}", e))
        } else {
            Ok(rx)
        }
    }

    pub fn send_messenger<R: Send + 'static>(
        &self,
        messenger: impl FnOnce(&T) -> Result<R> + Send + 'static,
    ) -> Result<R> {
        Ok(self.send_messenger_get_channel(messenger)?.recv()?)
    }

    pub fn send_mutating_messenger<R: Send + 'static>(
        &self,
        messenger: impl FnOnce(&mut T) -> Result<R> + Send + 'static,
    ) -> Result<R> {
        Ok(self.send_mutating_messenger_get_channel(messenger)?.recv()?)
    }
}

impl Drop for ShrineDestroyer {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f().unwrap();
        }
    }
}

impl ShrineDestroyer {
    pub fn invoke(self) {
        drop(self);
    }
}

#[cfg(test)]
mod miko_tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn miko_roundtrips_without_error() -> Result<()> {
        let test_message = "This is a test";
        let (miko, _thing) = Miko::build_shrine("test1", || Ok(test_message.to_string()))?;
        let res = miko.send_messenger(|t| Ok(t.clone()))?;
        assert!(res == test_message.to_string());
        Ok(())
    }
}
