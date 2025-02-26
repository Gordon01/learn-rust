use std::{
    collections::VecDeque,
    fmt::Write,
    sync::{Arc, RwLock},
};

use tokio::sync::broadcast::{self, Receiver};
use tracing::{
    error,
    field::{Field, Visit},
};
use tracing::{Event, Subscriber};
use tracing_subscriber::layer::{Context, Layer};

pub type TraceReceiver = Arc<Receiver<String>>;

pub struct BroadcastLayer {
    sender: broadcast::Sender<String>,
}

impl BroadcastLayer {
    pub fn new() -> (Self, TraceReceiver) {
        let (sender, rx) = broadcast::channel(100);
        let tracing_rx = Arc::new(rx);

        (Self { sender }, tracing_rx)
    }
}

impl<S> Layer<S> for BroadcastLayer
where
    S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut output = String::new();
        let metadata = event.metadata();
        let _ = write!(&mut output, "[{} {}] ", metadata.level(), metadata.target());

        event.record(&mut FieldRecorder {
            output: &mut output,
        });
        // Удалить лишний пробел в конце
        output.pop();
        let _ = self.sender.send(output);
    }
}

struct FieldRecorder<'a> {
    output: &'a mut String,
}

impl Visit for FieldRecorder<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        let _ = write!(self.output, "{}={:?} ", field.name(), value);
    }
}

#[derive(Clone)]
pub struct LogTail {
    messages: Arc<RwLock<VecDeque<String>>>,
}

impl LogTail {
    pub fn new(rx: TraceReceiver) -> Self {
        let messages = Arc::new(RwLock::new(VecDeque::with_capacity(100)));
        let messages_clone = Arc::clone(&messages);
        tokio::spawn(async move {
            let mut rx = rx.resubscribe();
            loop {
                match rx.recv().await {
                    Ok(msg) => {
                        let mut msgs = messages_clone.write().unwrap();
                        if msgs.len() == 100 {
                            msgs.pop_front();
                        }
                        msgs.push_back(msg);
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        error!("BUG: sender has been dropped");
                        break;
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                }
            }
        });

        Self { messages }
    }

    /// Returns the last `n` messages as a single `String` joined with newline characters.
    ///
    /// If there are fewer than `n` messages, all messages are returned.
    pub fn tail(&self, n: usize) -> String {
        let msgs = self.messages.read().unwrap();
        let start = if msgs.len() > n { msgs.len() - n } else { 0 };
        msgs.iter()
            .skip(start)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    }
}
