fn main() {
    let mut data = Some(vec![1, 2, 3]);
    let element_num = 6;
    let mut counter = 0;

    loop {
        if element_num == counter {
            // Явно покажем что здесь тип &mut Option
            let data_ref = &mut data;
            if let Some(d) = data_ref.take() {
                // Option::take() возвращает владение над значением
                process_data(d);
            }
        }

        counter += 1;
    }
}

fn process_data(data: Vec<i32>) {
    println!("Processing data of size: {}", data.len());
}
