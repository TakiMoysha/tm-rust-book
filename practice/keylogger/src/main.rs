//
// Show current presset command in
//
// Show input devices: `cat /proc/bus/input/devices | less`
// we needed only where 'B: KEY=' have a '';

use std::io;
use std::os::fd::AsRawFd;

use argparse::{ArgumentParser, StoreTrue};
use evdev::{Device, EventSummary};
use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};

mod input;

struct TrackedDevice {
    device: Device,
    name: Option<String>,
    event_node: String,
}

fn event_device_from_handlers(handlers: &Vec<String>) -> Option<String> {
    for handler in handlers {
        if handler.starts_with("event") {
            return Some(handler.to_string());
        }
    }

    None
}

fn main() {
    let mut verbose = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Show current presset command in ");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "verbose mode");
        ap.parse_args_or_exit();
    }

    let keyboards = input::find_keyboards().expect("Cannot find keyboards");

    if verbose {
        println!("Found {} keyboards:", keyboards.len());
        for dev in &keyboards {
            println!(
                "\t- {:?}: {:?}",
                dev.name.as_deref().unwrap(),
                dev.event_path.as_deref().unwrap()
            );
        }
    }

    let mut poll = Poll::new().expect("failed to create mio::Poll");
    let mut events = Events::with_capacity(128);
    let mut tracked_devices: Vec<TrackedDevice> = Vec::new();

    for info in &keyboards {
        let Some(handlers) = info.handlers.as_ref() else {
            if verbose {
                eprintln!("Skip {:?}: missing handlers entry", info.name);
            }

            continue;
        };

        let Some(event_node) = event_device_from_handlers(handlers) else {
            if verbose {
                eprintln!(
                    "Skip {:?}: cannot determine event handler from '{:?}'",
                    info.name, handlers
                );
            }

            continue;
        };

        let device_path = format!("/dev/input/{}", event_node);
        match Device::open(&device_path) {
            Ok(device) => {
                if let Err(err) = device.set_nonblocking(true) {
                    eprintln!("Cannot set non-blocking mode for {}: {}", device_path, err);
                    continue;
                }

                let token = Token(tracked_devices.len());
                let fd = device.as_raw_fd();
                let mut source = SourceFd(&fd);

                if let Err(err) = poll
                    .registry()
                    .register(&mut source, token, Interest::READABLE)
                {
                    eprintln!("Cannot register {} with Poll: {}", device_path, err);
                    continue;
                }

                if verbose {
                    println!(
                        "Registered {:?} at {} with token {:?}",
                        info.name, device_path, token
                    );
                }

                tracked_devices.push(TrackedDevice {
                    device,
                    name: info.name.clone(),
                    event_node,
                });
            }
            Err(err) => {
                eprintln!("Cannot open {}: {}", device_path, err);
            }
        }
    }

    if tracked_devices.is_empty() {
        eprintln!("No accessible keyboard event devices were registered");
        return;
    }

    println!("Listening for keyboard events. Press Ctrl+C to exit.");

    loop {
        if let Err(err) = poll.poll(&mut events, None) {
            eprintln!("Poll failed: {}", err);
            break;
        }

        for event in events.iter() {
            if !event.is_readable() {
                continue;
            }

            let idx = event.token().0;
            if let Some(tracked) = tracked_devices.get_mut(idx) {
                loop {
                    match tracked.device.fetch_events() {
                        Ok(iter) => {
                            let mut produced = false;
                            for input_event in iter {
                                produced = true;
                                if let EventSummary::Key(_, key_code, value) =
                                    EventSummary::from(input_event)
                                {
                                    let state = match value {
                                        1 => "pressed",
                                        0 => "released",
                                        2 => "repeated",
                                        _ => "unknown",
                                    };

                                    let device_name =
                                        tracked.name.as_deref().unwrap_or(&tracked.event_node);

                                    println!("[{}] {:?} {}", device_name, key_code, state);
                                }
                            }

                            if !produced {
                                break;
                            }
                        }
                        Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                            break;
                        }
                        Err(err) => {
                            eprintln!(
                                "Error reading events from {}: {}",
                                tracked.name.as_deref().unwrap_or(&tracked.event_node),
                                err
                            );
                            break;
                        }
                    }
                }
            }
        }
    }
}
