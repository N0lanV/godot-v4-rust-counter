use std::sync::{
        Arc,
        atomic::{
                AtomicI64,
                Ordering
        }
};
use std::thread;
use std::time::Duration;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base = RefCounted)]
pub struct Counter {
        pub count: Arc<AtomicI64>,

        #[base]
        base: Base<RefCounted>,
}

#[godot_api]
impl Counter {
        #[func]
        fn get_count(&mut self) -> i64 {
                self.count.load(Ordering::Relaxed)
        }

        #[func]
        fn increment_count(&mut self, _amount: i64) {
                self.count.fetch_add(_amount, Ordering::Relaxed);
        }

        #[func]
        fn decrement_count(&mut self, _amount: i64) {
                self.count.fetch_sub(_amount, Ordering::Relaxed);
        }

        #[func]
        fn increment_count_async(&mut self, _amount: u64)  {
                let count = self.count.clone();

                thread::spawn(move || {
                        for _ in 0.._amount{
                                thread::sleep(Duration::from_millis(1));
                                count.fetch_add(1, Ordering::Relaxed);
                        }
                });
        }
}

#[godot_api]
impl IRefCounted for Counter {
        fn init(base: Base<RefCounted>) -> Self {

                let count = Arc::new(AtomicI64::new(0));

                Counter {
                        count: count.clone(),
                        base,
                }
        }
}