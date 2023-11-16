use crate::mouse::{ScreenSide, Mouse};
use super::Size;
use rdev::{EventType, Event, grab};
use tokio::{
    io::AsyncWriteExt,
    net::TcpListener,
    sync::mpsc::{unbounded_channel, UnboundedSender},
};

fn event_handler(tx: UnboundedSender<Event>, event: Event) -> Option<Event> {
    if tx.send(event.clone()).is_err() {
        eprintln!("Failed to send event");
    }
    Some(event)
}

fn create_event_handler(tx: UnboundedSender<Event>) -> impl Fn(Event) -> Option<Event> {
    move |evt: Event| {
         event_handler(tx.clone(), evt)
    }
}

fn create_event_thread(tx: UnboundedSender<Event>) -> impl FnOnce() {
    move || {
        if let Err(error) = grab(create_event_handler(tx)) {
            println!("Error: {:?}", error)
        }
    }
}

pub async fn spawn(host: String, port: String) {
    let lisener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
    // This is the serverside dimensions
    let Ok((width, height)) = rdev::display_size() else {
        // TODO: Handle error
        eprintln!("Failed to get display size");
        return;
    };
    let server_screen_size = Size {
        width,
        height,
    };
    eprintln!("server_screen_size: {:?}", server_screen_size);
    println!("Server started on {}:{}", host, port);
    let (tx, mut rx): (UnboundedSender<Event>, _) = unbounded_channel();
    std::thread::spawn(create_event_thread(tx.clone()));
    let Ok((mut socket, _addr)) = lisener.accept().await else {
        // TODO: Handle error
        eprintln!("Failed to accept connection");
        return;
    };
    let (reader, mut writer) = socket.split();
    // TODO: The loop should go into a thread
    // # Overview
    // Once we start handling more then one client/screen/os at a time
    // :enhancement:

    let size = loop {
        let mut buf: Vec<u8> = Vec::new();
        let Ok(_) = reader.readable().await else {
            continue;
        };
        let Ok(_) = reader.try_read_buf(&mut buf) else {
            continue;
        };

        if buf.is_empty() {
            continue;
        }

        let Ok(size) = bincode::deserialize::<Size>(&buf) else {
            eprintln!("Failed to deserialize event");
            continue;
        };
        break size;
    };
    eprintln!("size of client screen: w: {}, h: {}", size.width, size.height);

    let _off_screen_mouse: Option<Mouse> = None;
    // Some(Mouse {
    //     screen_side: ScreenSide::Left,
    //     x: 0.0,
    //     y: 0.0,
    //     width: size.width as f32,
    //     height: size.height as f32,
    // });

    loop {
        let Ok(current_event) = rx.try_recv() else {
            // TODO: Handle error
            continue;
        };
        match current_event.event_type {
            EventType::MouseMove { x, y, .. } if x == 0.0 => {
                println!("{:?} {x} {y}", ScreenSide::Left);
            }
            EventType::MouseMove { x, y, .. } if x == server_screen_size.width as f64 => {
                println!("{:?} {x} {y}", ScreenSide::Right);
            }
            EventType::MouseMove { x, y, .. } if y == 0.0 => {
                println!("{:?} {x} {y}", ScreenSide::Top);
            }
            EventType::MouseMove { x, y, .. } if y == server_screen_size.height as f64 => {
                println!("{:?} {x} {y}", ScreenSide::Bottom);
            }
            _ => {}
        }
        // eprintln!("Current event: {:?}", current_event);
        // let Ok(serialized) = bincode::serialize(&current_event) else {
        //     // TODO: Handle error
        //     eprintln!("Failed to serialize event");
        //     return;
        // };
        // eprintln!("{:?}", current_event.event_type);
        // let Ok(()) = writer.write_all(&serialized).await else {
        //     // TODO: Handle error
        //     return;
        // };
    }
}

