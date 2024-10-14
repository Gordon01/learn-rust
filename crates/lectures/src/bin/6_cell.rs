use std::cell::UnsafeCell;

struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: на значение внутри нет ссылок, что гарантируется нашим API,
        // которое не выдает ссылки на значение внутри наружу.
        unsafe { *self.value.get() }
    }

    fn set(&self, value: T) {
        // SAFETY: тип имеет маркет `!Send`, поэтому мы можем быть уверены
        // что только наш единственный поток изменяет значение.
        unsafe { *self.value.get() = value };
    }
}

fn main() {
    let cell = Cell::new(42);
    assert_eq!(42, cell.get());

    cell.set(69);
    assert_eq!(69, cell.get());
}
