use std::alloc::{alloc, dealloc, Layout};
use std::ops::Deref;
use std::ptr::NonNull;

struct MyBox<T> {
    ptr: NonNull<T>,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        // Выделяем память в куче
        let layout = Layout::new::<T>();
        let ptr = unsafe { alloc(layout) } as *mut T;

        if ptr.is_null() {
            panic!("Не удалось выделить память");
        }

        let ptr = NonNull::new(ptr).unwrap();
        unsafe {
            ptr.write(value);
        } // Инициализируем память значением
        MyBox { ptr }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            // Вызываем деструктор и освобождаем память
            std::ptr::drop_in_place(self.ptr.as_ptr());
            dealloc(self.ptr.as_ptr() as *mut u8, Layout::new::<T>());
        }
    }
}

fn main() {
    let my_box = MyBox::new(42);
    println!("Значение: {}", *my_box);
    let _has_drop = MyBox::new(HasDrop);
}

struct HasDrop;
impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
    }
}
