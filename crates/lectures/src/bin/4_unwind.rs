use std::panic;

fn main() {
    for size in 0..5 {
        match panic::catch_unwind(|| real_main(size)) {
            Ok(_) => println!("Ok({size})"),
            Err(e) => eprintln!("Ошибка {:?} при размере {size}", e.type_id()),
        }
    }
}

fn real_main(size: usize) {
    let array = Vec::from_iter(0..size);
    let _ = array[1];
}
