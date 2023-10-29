// use std::thread;
use std::sync::mpsc;

// mpsc it is library for multi-producer single-consumer
// that provides a way to create and manage message queues
// that can be used to communicate between different threads or tasks
//
pub fn run() {
    println!("START mpsc;");
    // new message queue with a capacity of 10 msg
    // tx - transmitter, rx - receiver
    let (tx, rx) = mpsc::channel();

    let message = String::from("<StateUpdate(&21494, &14jsdf)>");
    tx.send(message).unwrap();
    read_receiver(&rx);
    println!("END mpsc;")
}

fn read_receiver(receiver: &mpsc::Receiver<String>) {
    let message = receiver.recv().unwrap();
    println!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_my_mpsc() {
        run();
    }
}
