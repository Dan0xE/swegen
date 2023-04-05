use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, mut _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reading, mut writing) = socket.split();

            let mut reading = BufReader::new(reading);
            let mut store_line = String::new();

            loop {
                tokio::select! {
                        result = reading.read_line(&mut store_line) => {
                                if result.unwrap() == 0 {
                                    break;
                                }

                        tx.send((store_line.clone(), addr)).unwrap();
                        store_line.clear();
                    }

                    result = rx.recv() => {
                        let (message, other_adress) = result.unwrap();

                        if addr != other_adress {
                             writing.write_all(message.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
