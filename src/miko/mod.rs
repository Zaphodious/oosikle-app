use std::sync::mpsc;
use std::thread;
use anyhow::Result;

type RawMessenger<T> = Box<dyn FnOnce(&T) -> Result<()> + Send + 'static>;

#[derive(Debug, Clone)]
pub struct Miko<T> {
    #[allow(dead_code)]
    shrine: thread::Thread,
    chan: mpsc::Sender<RawMessenger<T>>,
}

impl<T> Miko<T> where T: 'static {
    pub fn build_shrine<S, K>(label: &str, kami_summoner: S) -> Result<Miko<T>> where S: FnOnce() -> Result<T> + Send + 'static {
        let (chan, rx) = mpsc::channel::<RawMessenger<T>>();
        let b = thread::Builder::new().name(format!("miko_shrine_{}", label));

        let shrine = b.spawn(
            move || {
                let kami = kami_summoner().expect("Failure getting the value"); 
                for the_fn in rx {
                    the_fn(&kami).expect("There was a problem in the function");
                }
            }
            )?.thread().clone();
        Ok(Miko {shrine, chan })
    }

    pub fn send_raw_messenger(&self, the_fn: impl FnOnce(&T) -> Result<()> + Send + 'static) -> Result<()> {
        self.chan.send(Box::new(the_fn)).expect("There was a problem in the raw message function");
        Ok(())
    }

    pub fn send_messenger<R, M>(&self, messenger: impl FnOnce(&T) -> Result<R> + Send + 'static) -> Result<R> where R: Send + 'static {
        let (tx, rx) = mpsc::channel::<R>();
        self.send_raw_messenger(move |kami| {
            let res = messenger(kami)?;
            tx.send(res).unwrap();
            Ok(())
        }).expect("Something went wrong in the messenger function");
        Ok(rx.recv()?)
    }
}

