use std::{
    fmt::Debug,
    sync::{
        atomic::{self, AtomicBool},
        mpsc::{self, Receiver, TryRecvError},
        Arc,
    },
    thread::{self, JoinHandle},
};

use log::{debug, info, trace};

#[derive(Debug)]
pub struct Preloader<T>
where
    T: Debug + Send + Clone + 'static,
{
    child_thread: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
    receiver: Receiver<T>,
    default: T,
}

impl<T> Preloader<T>
where
    T: Debug + Send + Clone + 'static,
{
    pub fn new<F, G>(pool_size: usize, generator_fn: F, default: T) -> Self
    where
        F: FnOnce() -> G + Send + 'static,
        G: Generator<Output = T>,
    {
        let (sender, receiver) = mpsc::sync_channel(pool_size);
        let running = Arc::new(AtomicBool::new(true));
        let running_child = Arc::clone(&running);

        let child_thread = thread::spawn(move || {
            let mut generator = generator_fn();

            debug!(
                "Preloader child thread {:?} starting up",
                thread::current().id()
            );

            loop {
                if sender.send(generator.generate()).is_err() {
                    break;
                }

                if !running_child.load(atomic::Ordering::Relaxed) {
                    break;
                }

                trace!(
                    "Preloader child thread {:?} looping",
                    thread::current().id()
                );
            }

            debug!(
                "Preloader child thread {:?} shutting down",
                thread::current().id()
            );
        });

        debug!(
            "Parent thread {:?} spawned child preloader thread {:?}",
            thread::current().id(),
            child_thread.thread().id()
        );

        Self {
            child_thread: Some(child_thread),
            running,
            receiver,
            default,
        }
    }

    pub fn get_next(&self) -> T {
        self.receiver.recv().expect("Child thread disconnected")
    }

    pub fn get_next_or_default(&self) -> T {
        match self.receiver.try_recv() {
            Ok(item) => item,
            Err(TryRecvError::Empty) => self.default.clone(),
            Err(TryRecvError::Disconnected) => panic!("Child thread disconnected"),
        }
    }
}

impl<T> Drop for Preloader<T>
where
    T: Debug + Send + Clone + 'static,
{
    fn drop(&mut self) {
        info!("Shutting down preloader thread");
        if self.running.load(atomic::Ordering::Relaxed) {
            let child_thread = self.child_thread.take().unwrap();
            debug!(
                "Parent thread {:?} shutting down child preloader thread {:?}",
                thread::current().id(),
                child_thread.thread().id()
            );

            self.running.store(false, atomic::Ordering::Relaxed);

            loop {
                if self.receiver.try_recv().is_err() {
                    break;
                }
            }

            child_thread.join().unwrap();
        }
    }
}

pub trait Generator {
    type Output: Sized;

    fn generate(&mut self) -> Self::Output;
}
