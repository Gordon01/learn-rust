#![allow(dead_code)]

use std::sync::{Arc, Mutex};

const POISON: &str = "BUG: Poisoned mutex";

#[test]
fn simple_mutex() {
    let m = Mutex::new(1);
    let m1 = m.lock().expect(POISON);
    println!("{m1}");

    // Uncomment to avoid hang
    // drop(m1);
    let mut m2 = m.lock().expect(POISON);
    *m2 = 2;
    println!("{m2}");
}

struct InteriorMut {
    inner: Mutex<u32>,
}

impl InteriorMut {
    pub fn new(value: u32) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }

    pub fn set(&self, value: u32) {
        *self.inner.lock().expect(POISON) = value
    }

    pub fn read(&self) -> u32 {
        *self.inner.lock().expect(POISON)
    }
}

#[test]
fn interior_mutability() {
    let im = InteriorMut::new(10);
    println!("{}", im.read());
    im.set(12);
    println!("{}", im.read());

    std::thread::spawn(move || println!("{}", im.read()))
        .join()
        .unwrap();
}

struct SharedState {
    inner: Arc<Mutex<bool>>,
}

impl SharedState {
    pub fn new(inner: Arc<Mutex<bool>>) -> Self {
        Self { inner }
    }

    pub fn read(&self) -> bool {
        *self.inner.lock().expect(POISON)
    }

    pub fn set(&self, state: bool) {
        *self.inner.lock().expect(POISON) = state
    }
}

#[test]
fn shared_state() {
    let initial = Arc::new(Mutex::new(false));
    let first = SharedState::new(initial.clone());
    let second = SharedState::new(initial.clone());

    drop(initial);

    println!("first = {}, second = {}", first.read(), second.read());

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
        second.set(true);
    })
    .join()
    .unwrap();

    println!("first = {}", first.read());
}

fn main() {}
