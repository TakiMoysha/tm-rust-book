use core::time;

use builder::PickBalancerBuilder;

#[derive(Debug, Default)]
pub struct BuildOptions;
#[derive(Debug, Default)]
pub struct ClientConnection;

#[derive(Debug, Clone, Default)]
pub struct SubConnection;
#[derive(Debug, Clone, Default)]
pub struct ConnectionAttributes {
    pub priority: usize,
}
#[derive(Debug, Clone, Default)]
pub struct SubConnectionInfo {
    pub attributes: ConnectionAttributes,
}
#[derive(Debug, Default)]
pub struct AddressMap;
#[derive(Debug, Default)]
pub struct ConnectivityStateEvaluator;
#[derive(Debug, Default, PartialEq)]
pub enum ConnectivityState {
    #[default]
    Idle,
    Ready,
    Shutdown,
    TransientFailure,
}

impl ConnectionAttributes {
    pub fn value(&self, key: &str) -> Option<&usize> {
        if key == "index" {
            Some(&self.priority)
        } else {
            None
        }
    }
}

mod picker {
    use std::collections::HashMap;

    use crate::{ClientConnection, ConnectivityState, SubConnection, SubConnectionInfo};

    #[derive(Debug)]
    pub struct State {
        pub connectivity_state: ConnectivityState,
        // pub picker: Option<Box<dyn Picker>>>,
    }
    impl ClientConnection {
        pub fn update_state(&self, state: State) {
            todo!();
        }
    }
    #[derive(Debug, Clone)]
    pub struct PickResult {
        pub sub_conn: Option<SubConnection>,
    }

    #[derive(Debug)]
    pub struct PickInfo;

    #[derive(Debug)]
    pub struct ClientConnectionState {
        pub resolver_state: ResolverState,
    }

    #[derive(Debug)]
    pub struct ResolverState {
        pub addresses: Vec<String>,
    }

    #[derive(Debug)]
    pub struct ErrNoSubConnAvailable;

    pub trait Picker {
        fn pick(&self, info: PickInfo) -> Result<PickResult, Option<&ErrNoSubConnAvailable>>;
    }

    impl<T: Picker + ?Sized> Picker for Box<T> {
        fn pick(&self, info: PickInfo) -> Result<PickResult, Option<&ErrNoSubConnAvailable>> {
            (*self).pick(info)
        }
    }

    pub struct FirstIndxPicker {
        result: PickResult,
        err: Option<ErrNoSubConnAvailable>,
    }

    impl Picker for FirstIndxPicker {
        fn pick(&self, info: PickInfo) -> Result<PickResult, Option<&ErrNoSubConnAvailable>> {
            if let Some(err) = &self.err {
                Err(Some(err.clone()))
            } else {
                Ok(self.result.clone())
            }
        }
    }

    pub struct PickerBuildInfo {
        pub ready_scs: HashMap<SubConnection, SubConnectionInfo>,
    }

    pub fn new_fi_picker(info: PickerBuildInfo) -> Box<impl Picker> {
        if info.ready_scs.is_empty() {
            return Box::new(FirstIndxPicker {
                result: PickResult { sub_conn: None },
                err: Some(ErrNoSubConnAvailable),
            });
        }

        let mut min_indx = usize::MAX;
        let mut selected_conn: Option<SubConnection> = None;

        for (sc, sc_info) in info.ready_scs.iter() {
            if let Some(indx) = sc_info.attributes.value("index") {
                if *indx < min_indx {
                    min_indx = *indx;
                    selected_conn = Some(sc.clone());
                }
            }
        }

        if let Some(conn) = selected_conn {
            return Box::new(FirstIndxPicker {
                result: PickResult {
                    sub_conn: Some(conn),
                },
                err: None,
            });
        }

        Box::new(FirstIndxPicker {
            result: PickResult { sub_conn: None },
            err: Some(ErrNoSubConnAvailable),
        })
    }
}

mod resolver {
    use std::error::Error;
    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::ClientConnection;

    #[derive(Debug, Clone)]
    pub struct Address {
        addr: String,
        priority: u32,
    }

    // Resolver responsibility:
    // * Dynamic addresses update;
    // * Providing addresses with priorities to the balancer;
    // * Error messages when addresses are unavailable.
    struct Resolver {
        ctx: Arc<Mutex<std::sync::mpsc::Receiver<()>>>,
        target: String,
        cc: ClientConnection,
        addresses_store: Vec<Address>,
    }

    struct ResolverBuilder {
        addresses: Vec<Address>,
    }

    impl ResolverBuilder {
        pub fn build(
            &self,
            target: String,
            cc: ClientConnection,
        ) -> Result<Resolver, Box<dyn Error>> {
            let (tx, rx) = std::sync::mpsc::channel();
            let ctx = Arc::new(Mutex::new(rx));

            let resolver = Resolver {
                ctx: ctx.clone(),
                target,
                cc,
                addresses_store: self.addresses.clone(),
            };

            if self.addresses.len() > 1 {}

            thread::spawn(move || {
                if let Err(_) = tx.send(()) {
                    // error handler
                }
            });

            Ok(resolver)
        }

        pub fn schema(&self) -> &'static str {
            "demo_schema"
        }
    }

    pub struct AddressResolver {}

    // accepts a list of addresses "192.168.1.1"
    pub fn init_resolver(addresses: Vec<&str>) {
        let addresses_store = addresses
            .iter()
            .enumerate()
            .map(|(i, value)| Address {
                addr: value.to_string(),
                priority: i as u32,
            })
            .collect();

        let resolver_builder = ResolverBuilder {
            addresses: addresses_store,
        };
        register_resolver(resolver_builder);
    }

    fn register_resolver(resolver_builder: ResolverBuilder) {
        println!("Resolver Registered with {:?}", resolver_builder.addresses);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn builder_should_create_resolver() {
            todo!()
        }
    }
}

mod balancer {
    use std::{collections::HashMap, error::Error};

    use crate::{
        builder::PickBalancerBuilder,
        picker::{ClientConnectionState, Picker, PickerBuildInfo, State},
        AddressMap, ClientConnection, ConnectivityState, ConnectivityStateEvaluator, SubConnection, SubConnectionInfo,
    };

    #[derive(Debug, Default)]
    pub struct PickBalancer {
        cc: ClientConnection,
        // sub_conns: AddressMap,
        sub_conns: HashMap<String, SubConnection>,
        sc_states: HashMap<SubConnection, ConnectivityState>,
        cs_evltr: ConnectivityStateEvaluator,
        state: ConnectivityState,
        picker: Option<dyn Picker>,
        resolve_err: Option<dyn Error>>,
        conn_err: Option<dyn Error>,
    }

    use std::collections::HashMap;


    impl PickBalancer {
        pub fn register(builder: PickBalancerBuilder) {}

        pub fn set_client_connection(&mut self, cc: ClientConnection) {
            self.cc = cc;
        }

        pub fn create_new_sub_connections(
            &self,
            _ccs: ClientConnectionState,
        ) -> Result<HashMap<String, SubConnection>, Box<dyn Error>> {
            Ok(HashMap::new())
        }

        // update state of existing sub-connection
        pub fn update_sub_conn_state(&mut self, sub_conn: SubConnection, state: ConnectivityState) {
            if let Some(&old_state) = self.sc_states.get(&sub_conn) {
                self.sc_states.insert(sub_conn.clone(), state.clone());
                match state {
                    ConnectivityState::Idle => {
                        // connection processing
                    },
                    ConnectivityState::Shutdown => {
                        self.sc_states.remove(&sub_conn);
                    },
                    ConnectivityState::TransientFailure => {
                        // connection error processing
                    },
                    _ => {}
                }
            }

            self.state = self.cs_evltr.record_transition(old_state, state.clone());

            if (state == ConnectivityState::Ready) != (old_state == ConnectivityState::Ready) || self.state == ConnectivityState::TransientFailure {
                self.regenerate_picker();
            }

            self.cc.upadte_state(State {
                connectivity_state: self.state.clone(),
                picker: self.picker.clone()
            });
        }

        // update picker while state changed
        pub fn regenerate_picker(&mut self) {
            if self.state == ConnectivityState::TransientFailure {
                self.picker = Some(Box::new(FirstIndxPicker {err: self.resolve_err.clone().or(self.conn_err.clone())}))
                return;
            }

            let mut ready_scs = HashMap::new();

            for (addr, sc) in &self.sub_conns {
                // if let Some(&state) == self.sc_states
                if let Some(&state) = self.sc_states.get(sc) {
                    if state == ConnectivityState::Ready {
                        ready_scs.insert(sc.clone(), SubConnectionInfo { address: addr.clone() });
                    }
                }
            }

            self.picker = Some(Box::new(NewFIPicker(PickerBuildInfo { ready_csc })));
        }

        // create new and remove inactive connections
        pub fn update_client_conn_state(
            &mut self,
            ccs: ClientConnectionState,
        ) -> Result<(), Box<dyn Error>> {
            self.resolver_err = None;

            let address_map = self.create_new_sub_connections(ccs)?;

            // filter - remove inactive connections
            let keys: Vec<String> = self.sub_conns.keys().cloned().collect();
            for addr in keys {
                if !address_map.contains_key(&addr) {
                    if let Some(sc) = self.sub_conns.get(&addr) {
                        self.sub_conns.remove(&addr);
                    }
                }
            }

            if ccs.resolver_state.addresses.is_empty() {
                self.resolver_err = Some(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Zero addresses",
                )));
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Bad resolver state",
                )));
            }

            self.regenerate_picker();
            self.cc.update_state(State {
                connectivity_state: self.state.clone(),
                picker: self.picker.clone(),
            });

            Ok(())
        }
    }
}
mod builder {
    use crate::{BuildOptions, ClientConnection, ConnectivityState};

    use super::balancer::PickBalancer;

    // Bancer responsibility:
    //  * Create and init PickBalancer
    //  * Set up interraction with ClientConnection
    //  * Registering a balancer for use by the client
    pub struct PickBalancerBuilder;

    impl PickBalancerBuilder {
        pub fn new() -> Self {
            Self {}
        }

        pub fn build(cc: ClientConnection, _options: BuildOptions) -> PickBalancer {
            let mut balancer = PickBalancer::default();
            balancer.set_client_connection(cc);
            return balancer;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_correct_create_builder() {
            todo!()
        }

        #[test]
        fn should_build_balancer() {
            todo!()
        }
    }
}

pub struct Config {
    addresses: Vec<&'static str>,
    retry_timeout: time::Duration,
    retry_max: u32,
}

fn main() {
    // aka init_config
    let config = Config {
        addresses: vec!["192.168.1.10", "192.168.1.11"],
        retry_timeout: time::Duration::from_millis(1000),
        retry_max: 10,
    };

    let balancer = PickBalancerBuilder::build(
        ClientConnection::default(),
        BuildOptions::default(),
    );
}
