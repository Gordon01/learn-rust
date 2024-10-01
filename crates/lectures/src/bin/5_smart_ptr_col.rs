use std::ops::Deref;

struct Doubled<T: Copy> {
    inner: Vec<(T, T)>,
}

impl<T: Copy> Doubled<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, v: T) {
        self.inner.push((v, v));
    }
}

impl<T: Copy> Deref for Doubled<T> {
    type Target = [(T, T)];

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

fn main() {
    let mut d = Doubled::new();
    d.push(2);
    d.push(4);

    let slice: &[(u32, u32)] = &d;
    println!("Slice - d: {:?}", slice);

    // Или есть способ лучше: временная изменяемость
    let d1 = {
        let mut d = Doubled::new();
        d.push(3);
        d.push(4);
        d
    };
    // d1.push(5);

    let slice: &[(u32, u32)] = &d1;
    println!("Slice - d1: {:?}", slice);

    // Или можно еще так:
    let mut d2 = Doubled::new();
    d2.push(5);
    d2.push(6);
    let d2 = d2;
    // d2.push(10);

    let slice: &[(u32, u32)] = &d2;
    println!("Slice - d2: {:?}", slice);
}
