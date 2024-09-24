use std::{
    io::{self, BufRead},
    thread,
    time::Duration,
};

fn main() {
    thread::spawn(|| {
        let mut vec = vec![42u8; 10 * 1024 * 1024];

        for gen in 0.. {
            assert!(gen < 5, "OOPS: gen >= 5");

            // Если ничего не делать с `vec`, то в релизной сборке он не будет аллоцирован
            vec.push(gen);
            println!("Итерация: {gen}");
            thread::sleep(Duration::from_secs(3));
        }
    });

    println!("Напишите что-нибудь!");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("Вы написали '{}'", line.unwrap());
    }
}
