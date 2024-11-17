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
#[derive(Debug, Default)]
pub enum ConnectivityState {
    #[default]
    GRPC,
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

    use crate::{SubConnection, SubConnectionInfo};

    #[derive(Debug, Clone)]
    pub struct PickResult {
        pub sub_conn: Option<SubConnection>,
    }

    #[derive(Debug)]
    pub struct PickInfo;

    #[derive(Debug)]
    pub struct ErrNoSubConnAvailable;

    pub trait Picker {
        fn pick(&self, info: PickInfo) -> Result<PickResult, Option<&ErrNoSubConnAvailable>>;
    }

    pub struct FirstIdxPicker {
        result: PickResult,
        err: Option<ErrNoSubConnAvailable>,
    }

    impl Picker for FirstIdxPicker {
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
            return Box::new(FirstIdxPicker {
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
            return Box::new(FirstIdxPicker {
                result: PickResult {
                    sub_conn: Some(conn),
                },
                err: None,
            });
        }

        Box::new(FirstIdxPicker {
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
    use std::collections::HashMap;

    use crate::{
        builder::PickBalancerBuilder, AddressMap, ClientConnection, ConnectivityState,
        ConnectivityStateEvaluator, SubConnection,
    };

    #[derive(Debug, Default)]
    pub struct PickBalancer {
        cc: ClientConnection,
        sub_conns: AddressMap,
        sc_states: HashMap<SubConnection, ConnectivityState>,
        cs_evltr: ConnectivityStateEvaluator,
        state: ConnectivityState,
    }

    impl PickBalancer {
        pub fn register(builder: PickBalancerBuilder) {}

        pub(crate) fn set_client_connection(&mut self, cc: ClientConnection) {
            self.cc = cc;
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

        pub fn build(&self, cc: ClientConnection, _options: BuildOptions) -> PickBalancer {
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
}

fn main() {
    // aka init_config
    let config = Config {
        addresses: vec!["192.168.1.10", "192.168.1.11"],
    };
}
