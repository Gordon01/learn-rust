use tokio::{
    sync::mpsc,
    time::{sleep, Duration, Instant},
};
use tracing::info;

type Message = (String, Instant);

async fn sender(tx: mpsc::Sender<Message>) {
    let messages = vec![
        "Привет".to_string(),
        "Как дела?".to_string(),
        "До свидания".to_string(),
    ];

    for msg in messages {
        info!("Отправляю сообщение: {}", msg);
        let start = Instant::now();
        tx.send((msg, start)).await.unwrap();
        sleep(Duration::from_secs(1)).await;
    }
}

async fn receiver(mut rx: mpsc::Receiver<Message>) {
    while let Some((msg, start)) = rx.recv().await {
        info!("Получено сообщение: {}, за {:?}", msg, start.elapsed());
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().compact().init();

    // Создаем канал с буфером на 10 сообщений
    let (tx, rx) = mpsc::channel(10);

    let sender_task = tokio::spawn(sender(tx));
    let receiver_task = tokio::spawn(receiver(rx));

    // Ожидаем завершения задач
    let _ = tokio::join!(sender_task, receiver_task);
}
