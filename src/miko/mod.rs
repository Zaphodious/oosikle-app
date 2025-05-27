use anyhow::{Error, Result};
use std::mem::replace;
use std::sync::mpsc;
use std::thread;
use uuid;

pub type RawMessenger<T> = Option<Box<dyn FnOnce(&T) -> Result<()> + Send + 'static>>;
type ShrineDestroyingFunction = Box<dyn FnOnce() -> Result<()> + 'static>;

#[derive(Debug, Clone)]
pub struct Miko<T> {
    #[allow(dead_code)]
    shrine: thread::Thread,
    chan: mpsc::Sender<RawMessenger<T>>,
}

pub struct ShrineDestroyer(Option<ShrineDestroyingFunction>);

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
            let kami = kami_summoner().expect("Failure getting the value");
            println!("Setting up thread {:?}", thread::current().name());
            for fn_package in rx {
                if let Some(the_fn) = fn_package {
                    match the_fn(&kami) {
                        Ok(_) => {println!("A function completed successfully");continue},
                        Err(_n) => break,
                    };
                } else {
                    break;
                };
            }
        })?;
        let chanclone = chan.clone();
        let shrine = shrine_handle.thread().clone();
        Ok((
            Miko { shrine, chan },
            ShrineDestroyer(Some(Box::new(move || {
                chanclone.send(None).unwrap();
                shrine_handle.join().unwrap();
                Ok(())
            }))),
        ))
    }

    pub fn send_raw_messenger(
        &self,
        the_fn: impl FnOnce(&T) -> Result<()> + Send + 'static,
    ) -> Result<()> {
        if let Err(n) = self.chan.send(Some(Box::new(the_fn))) {
            anyhow::bail!(n.to_string())
        } else {
            Ok(())
        }
    }

    pub fn send_messenger<R>(
        &self,
        messenger: impl FnOnce(&T) -> Result<R> + Send + 'static,
    ) -> Result<R>
    where
        R: Send + 'static,
    {
        let (tx, rx) = mpsc::channel::<R>();
        self.send_raw_messenger(move |kami| {
            let res = messenger(kami)?;
            tx.send(res).unwrap();
            Ok(())
        })
        .expect("Something went wrong in the messenger function");
        Ok(rx.recv()?)
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
