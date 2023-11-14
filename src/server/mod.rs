use rdev::{listen, Event};
use tokio::{
    io::AsyncWriteExt,
    net::TcpListener,
    sync::mpsc::unbounded_channel,
};

pub async fn spawn(host: String, port: String) {
    let lisener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
    println!("Server started on {}:{}", host, port);
    let (tx, mut rx) = unbounded_channel();
    std::thread::spawn(move || {
        if let Err(error) = listen(move |evt: Event| {
            tx.send(evt).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }) {
            println!("Error: {:?}", error)
        }
    });
    let Ok((mut socket, _addr)) = lisener.accept().await else {
        // TODO: Handle error
        eprintln!("Failed to accept connection");
        return;
    };
    let (_reader, mut writer) = socket.split();
    loop {
        let Ok(current_event) = rx.try_recv() else {
            // TODO: Handle error
            continue;
        };
        eprintln!("Current event: {:?}", current_event);
        let Ok(serialized) = bincode::serialize(&current_event) else {
            // TODO: Handle error
            return;
        };
        let Ok(()) = writer.write_all(&serialized).await else {
            // TODO: Handle error
            eprintln!("Failed to write");
            return;
        };
        eprintln!("Sent event: {:?}", current_event.event_type);
    }
}

