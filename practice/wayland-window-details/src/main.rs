use swayipc::{Connection, EventType, WorkspaceChange};

fn main() {
    let mut conn = Connection::new().unwrap();

    // Подписываемся на события workspace
    let events = conn.subscribe(&[EventType::Workspace]).unwrap();

    for event in events {
        if let Ok(swayipc::Event::Workspace(ws_event)) = event {
            match ws_event.change {
                WorkspaceChange::Init => {
                    println!("Init: {:#}", &ws_event.current.unwrap().name.unwrap());
                }
                WorkspaceChange::Focus => {
                    println!(
                        "Switched to: {:#}",
                        &ws_event.current.unwrap().name.unwrap()
                    );
                }
                _ => (),
            }
        }
    }
}

