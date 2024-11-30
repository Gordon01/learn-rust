#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

#[test]
#[should_panic]
fn simple_refcell() {
    let rc = RefCell::new(1);
    let r1 = rc.borrow();
    println!("{r1}");
    let r2 = rc.borrow();
    // *r2 = 2;
    println!("{r2}");

    // Uncomment it to avoid panic
    // drop(r1);
    // drop(r2);

    let mut f = rc.borrow_mut();
    *f = 2;
    println!("{f}");
}

struct InteriorMut {
    inner: RefCell<u32>,
}

impl InteriorMut {
    pub fn new(value: u32) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    pub fn set(&self, value: u32) {
        *self.inner.borrow_mut() = value;
    }

    pub fn read(&self) -> u32 {
        *self.inner.borrow()
    }
}

#[test]
fn interior_mutability() {
    let im = InteriorMut::new(10);
    println!("{}", im.read());
    im.set(12);
    println!("{}", im.read());
}

struct SharedState {
    inner: Rc<RefCell<bool>>,
}

impl SharedState {
    pub fn new(inner: Rc<RefCell<bool>>) -> Self {
        Self { inner }
    }

    pub fn read(&self) -> bool {
        *self.inner.borrow()
    }

    pub fn set(&self, state: bool) {
        *self.inner.borrow_mut() = state
    }
}

#[test]
fn shared_state() {
    let initial = Rc::new(RefCell::new(false));
    let first = SharedState::new(initial.clone());
    let second = SharedState::new(initial.clone());
    println!("first = {}, second = {}", first.read(), second.read());

    drop(initial);
    println!("first = {}, second = {}", first.read(), second.read());

    first.set(true);
    println!("first = {}, second = {}", first.read(), second.read());
}

fn main() {}
