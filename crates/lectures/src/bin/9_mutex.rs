use std::time::Duration;
use tracing::info;

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Sleep {
    Std,
    Tokio,
    TokioSpawnStd,
}

// Когда используйте `Sleep::Std` попробуйте поменять число потоков здесь
#[tokio::main(worker_threads = 1)]
async fn main() {
    // Инициализируем особый логгер, который лучше работает в async
    tracing_subscriber::fmt::init();

    // Попробуйте поменять способ реализации `sleep()`
    let sleep = Sleep::Std;
    // Запускаем `async_task()` каждую в отдельной задаче,
    // для того, чтобы добиться параллелизма
    let task1 = tokio::spawn(async_task(1, sleep));
    let task2 = tokio::spawn(async_task(2, sleep));
    let task3 = tokio::spawn(async_task(3, sleep));
    let task4 = tokio::spawn(async_task(4, sleep));

    // Ожидаем завершения всех задач
    let _ = tokio::join!(task1, task2, task3, task4);

    info!("Все задачи завершены");
}

async fn async_task(id: usize, flavor: Sleep) {
    info!("Задача {id} запускается");

    let dur = Duration::from_secs(1);

    match flavor {
        // Используем `std::thread::sleep` вместо асинхронного `sleep`.
        // Будет блокироваться поток, в котором работает задача, что нарушит
        // работу планировщика в `tokio`
        Sleep::Std => std::thread::sleep(dur),
        Sleep::Tokio => tokio::time::sleep(Duration::from_secs(1)).await,
        // Но что если блокирующий код все равно нужно использовать?
        // На этот случай есть `block_in_place()` и `spawn_blocking()`
        Sleep::TokioSpawnStd => tokio::task::spawn_blocking(move || std::thread::sleep(dur))
            .await
            .unwrap(),
    }

    info!("Задача {id} завершена");
}
