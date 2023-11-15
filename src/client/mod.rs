use super::Size;
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
    let Ok((width, height)) = rdev::display_size() else {
        // TODO: Handle error
        eprintln!("Failed to get display size");
        return;
    };
    let client_screen_size = Size {
        width,
        height,
    };
    // TODO: Handle error
    let serialized_screen_size = bincode::serialize(&client_screen_size).unwrap();
    loop {
        let Ok(_) = stream.writable().await else {
            continue;
        };
        match stream.try_write(&serialized_screen_size) {
            Ok(_) => {
                break;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(_) => {
                continue;
            }
        }
    }
    loop {
        let mut buf = Vec::new();
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

        crate::event::send(&event.event_type);
    }
}
