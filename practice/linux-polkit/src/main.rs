// Polkit is na authorization framework installed on every linux.
// It have API for ask privileged from application.
//
// Communication with polkit happens over d-bus, polkit registered as `org.freedesktop.PolicyKit1`
//
// rule file use javascript lang and `subject` and `action` for define condition
// `subject` - it is object who perform action
// `action` - it is object involved action

use std::{collections::HashMap, time::Duration};

use dbus::{
    arg::{self, AppendAll, ReadAll, RefArg},
    blocking::Connection,
};

// #[interface()]
// struct DBusObject {}

#[derive(Debug)]
struct AuthorizationResult {
    is_authorized: bool,
    is_challenge: bool,
    details: HashMap<String, String>,
}

fn permitted_operation() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_system()?;

    let proxy = conn.with_proxy(
        "org.freedesktop.PolicyKit1",
        "/org/freedesktop/PolicyKit1/Authority",
        Duration::from_secs(5),
    );

    let mut subject_process_opts: HashMap<&str, arg::Variant<Box<dyn RefArg>>> = HashMap::new();
    subject_process_opts.insert("pid", arg::Variant(Box::new(std::process::id())));
    subject_process_opts.insert("start-time", arg::Variant(Box::new(0 as u64)));
    let subject_process = ("unix-process".to_string(), subject_process_opts);

    // let mut subject_opts: HashMap<String, arg::Variant<Box<dyn RefArg>>> = HashMap::new();
    // subject_opts.insert("session-id", arg::Variant("".to_string()));

    // let subject_session = ("unix-session".to_string(), subject_opts);

    let mut details: HashMap<String, String> = HashMap::new();
    details.insert("AllowUserInteraction".to_string(), "true".to_string());

    const CHECK_AUTH_ALLOW_USER_INTERACTION: u32 = 1;

    let (auth_result, _details): (bool, Vec<u8>) = proxy.method_call(
        "org.freedesktop.PolicyKit1.Authority",
        "CheckAuthorization",
        (
            subject_process,
            "dev.takimoysha.linuxdesktop.read-evdev".to_string(),
            details,
            CHECK_AUTH_ALLOW_USER_INTERACTION,
            "",
        ),
    )?;

    if auth_result {
        println!("Polkit ALLOW operation");
    // } else if auth_result.is_challenge {
    //     println!("Polkit CHALLENGE operation - required user accepti.");
    } else {
        println!("Polkit DENY operation");
    }

    Ok(())
}

mod gui {
    use std::error::Error;

    use smithay_client_toolkit::{
        delegate_output, delegate_registry,
        output::{OutputHandler, OutputInfo, OutputState},
        registry::{ProvidesRegistryState, RegistryState},
        registry_handlers,
    };
    use wayland_client::{
        Connection, QueueHandle, globals::registry_queue_init, protocol::wl_output,
    };

    pub struct ListOutputs {
        pub registry_state: RegistryState,
        pub output_state: OutputState,
    }

    delegate_output!(ListOutputs);
    impl OutputHandler for ListOutputs {
        fn output_state(&mut self) -> &mut OutputState {
            &mut self.output_state
        }

        fn new_output(
            &mut self,
            conn: &Connection,
            qh: &QueueHandle<Self>,
            output: wl_output::WlOutput,
        ) {
        }

        fn update_output(
            &mut self,
            conn: &Connection,
            qh: &QueueHandle<Self>,
            output: wl_output::WlOutput,
        ) {
        }

        fn output_destroyed(
            &mut self,
            conn: &Connection,
            qh: &QueueHandle<Self>,
            output: wl_output::WlOutput,
        ) {
        }
    }

    delegate_registry!(ListOutputs);
    impl ProvidesRegistryState for ListOutputs {
        fn registry(&mut self) -> &mut RegistryState {
            &mut self.registry_state
        }

        registry_handlers! {
            OutputState
        }
    }

    pub fn window() -> Result<(), Box<dyn Error>> {
        let conn = Connection::connect_to_env()?;

        let (globals, mut event_queue) = registry_queue_init(&conn).unwrap();
        let gh = event_queue.handle();

        let registry_state = RegistryState::new(&globals);
        let output_delegate = OutputState::new(&globals, &gh);

        let mut list_output = ListOutputs {
            registry_state,
            output_state: output_delegate,
        };

        event_queue.roundtrip(&mut list_output)?;

        for output in list_output.output_state.outputs() {
            // print_cbar(&output);
            || {
                &list_output
                    .output_state
                    .info(&output)
                    .ok_or_else(|| "output has no info".to_owned());
            };
        }

        Ok(())
    }
}

fn main() {
    // RUST_LOG=debug
    env_logger::init();

    let permission = permitted_operation();

    if permission.is_ok() {
        gui::window();
    } else {
        eprintln!("Error: {}", permission.unwrap_err());
    }
}
