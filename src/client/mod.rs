use rdev::Event;
use tokio::net::TcpStream;

pub async fn spawn(host: String, port: String) {
    println!("Client started on {}:{}", host, port);
    let Ok(adder) = format!("{host}:{port}").parse::<std::net::SocketAddr>() else {
        eprintln!("Failed to parse address: {host}:{port}");
        return;
    };
    eprintln!("aquire adder");
    let Ok(stream) = TcpStream::connect(adder).await else {
        eprintln!("Failed to connect");
        return;
    };
    let mut buf = Vec::new();
    loop {
        let Ok(_) = stream.readable().await else {
            continue;
        };

        let Ok(_) = stream.try_read_buf(&mut buf) else {
            continue;
        };

        if buf.is_empty() {
            continue;
        }

        let Ok(event) = bincode::deserialize::<Event>(&buf) else {
            eprintln!("Failed to deserialize event");
            continue;
        };
        buf.clear();

        crate::event::send(&event.event_type);
    }
}
