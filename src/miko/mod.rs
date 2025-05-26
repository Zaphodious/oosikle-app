use anyhow::Result;
use std::sync::mpsc; 
use std::thread;

type RawMessenger<T> = Option<Box<dyn FnOnce(&T) -> Result<()> + Send + 'static>>;
type MikoDestroyer = Box<dyn FnOnce() -> Result<()> + 'static>;

#[derive(Debug, Clone)]
pub struct Miko<T> {
    #[allow(dead_code)]
    shrine: thread::Thread,
    chan: mpsc::Sender<RawMessenger<T>>,
}

impl<T> Miko<T>
where
    T: 'static,
{
    pub fn build_shrine(
        label: &str,
        kami_summoner: impl FnOnce() -> Result<T> + Send + 'static,
    ) -> Result<(Miko<T>, MikoDestroyer)> {
        let (chan, rx) = mpsc::channel::<RawMessenger<T>>();
        let b = thread::Builder::new().name(format!("miko_shrine_{}", label));

        let shrine_handle = b.spawn(move || {
            let kami = kami_summoner().expect("Failure getting the value");
            for fn_package in rx {
                if let Some(the_fn) = fn_package {
                    the_fn(&kami).expect("There was a problem in the function");
                } else {
                    break;
                }
            }
        })?;
        let chanclone = chan.clone();
        let shrine = shrine_handle.thread().clone();
        Ok((Miko { shrine, chan }, Box::new(move || {
            chanclone.send(None).unwrap();
            shrine_handle.join().unwrap();
            Ok(())
        })))
    }

    pub fn send_raw_messenger(
        &self,
        the_fn: impl FnOnce(&T) -> Result<()> + Send + 'static,
    ) -> Result<()> {
        self.chan
            .send(Some(Box::new(the_fn)))
            .expect("There was a problem in the raw message function");
        Ok(())
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

#[cfg(test)]
mod miko_tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn miko_constructs_without_error() -> Result<()> {
        let test_message = "This is a test";
        let (miko, destroyer) = Miko::build_shrine("test1", || {Ok(test_message.to_string())})?;
        let res = miko.send_messenger(|t| {Ok(t.clone())})?;
        assert!(res == test_message.to_string());
        destroyer()?;
        Ok(())
    }
}
