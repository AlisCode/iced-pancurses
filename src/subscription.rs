use std::collections::{VecDeque, HashMap, HashSet};
use std::hash::Hasher;
use std::sync::{Arc, Mutex};
use iced_native::{Event, Hasher as IcedHasher, Subscription};

struct Handle {
    _cancel: futures::channel::oneshot::Sender<()>,
    sender: Option<futures::channel::mpsc::Sender<Event>>,
}

#[derive(Default)]
pub struct SubscriptionPool {
    alive: HashMap<u64, Handle>,
}

impl SubscriptionPool {
    pub fn update<Message: Send + 'static>(
        &mut self, 
        subscription: Subscription<Message>,
        thread_pool: &mut futures::executor::ThreadPool,
        event_queue: Arc<Mutex<Option<VecDeque<Message>>>>,
    ) {
        use futures::{future::FutureExt, stream::StreamExt};
        let recipes = subscription.recipes();
        let mut alive = HashSet::new();
        
        for recipe in recipes {
            let hashed = { 
                let mut hasher = IcedHasher::default(); recipe.hash(&mut hasher); hasher.finish() 
            };
            let _ = alive.insert(hashed);
            if !self.alive.contains_key(&hashed) {
                let (cancel, cancelled) = futures::channel::oneshot::channel();
                let (tx, rx) = futures::channel::mpsc::channel(100);
                let stream = recipe.stream(rx.boxed());
                let evt_queue = event_queue.clone();
                let fut = futures::future::select(cancelled, stream.for_each(move |msg| {
                    let mut lock = evt_queue.lock().expect("Poisoned lock");
                    let mut queue = lock.take().unwrap();
                    queue.push_back(msg);
                    *lock = Some(queue);
                    futures::future::ready(())
                })).map(|_| ());

                thread_pool.spawn_ok(fut);

                self.alive.insert(
                    hashed, 
                    Handle {
                        _cancel: cancel,
                        sender: if tx.is_closed() { None} else { Some (tx) }, 
                    }
                );
            }
        }
    }

    pub fn broadcast(&mut self, event: Event) {
        self.alive
            .values_mut()
            .filter_map(|connection| connection.sender.as_mut())
            .for_each(|listener| {
                if let Err(error) = listener.try_send(event) {
                    panic!("Failed to communicate with sender {}", error);
                }
            });
    }
}
