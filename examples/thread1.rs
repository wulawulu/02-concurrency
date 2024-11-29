use anyhow::Result;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    id: usize,
    value: usize,
}

impl Msg {
    fn new(id: usize, value: usize) -> Self {
        Msg { id, value }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer exit");
        42
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Thread join error: {:?}", e))?;

    println!("secret: {:?}", secret);

    Ok(())
}

fn producer(index: usize, tx: Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(index, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", index);
            break;
        }
    }

    Ok(())
}
