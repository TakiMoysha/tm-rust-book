use std::time::Duration;

use dbus::blocking::Connection;
use notify::send_test_notification;

fn list_of_services(conn: &Connection) -> Result<Vec<String>, ()> {
    let proxy = conn.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
    let (names,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .expect("Failed to list names");

    Ok(names)
}

mod notify {
    use std::collections::HashMap;
    use std::time::Duration;

    use dbus::arg::{RefArg, Variant};
    use dbus::blocking::Connection;

    type DbusMap = HashMap<String, Variant<Box<dyn RefArg>>>;
    // https://github.com/KDE/liquidshell/blob/master/org.freedesktop.Notifications.xml
    struct NotifyPayload {
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: DbusMap,
        expire_timeout: i32,
    }

    #[derive(Clone, Copy)]
    enum Urgency {
        Low,
        Normal,
        Critical,
    }

    impl Urgency {
        fn to_mpris(self) -> &'static str {
            match self {
                Urgency::Low => "low",
                Urgency::Normal => "normal",
                Urgency::Critical => "critical",
            }
        }
    }

    fn to_variant(value: u8) -> Variant<Box<dyn RefArg>> {
        Variant(Box::new(value.to_string()))
    }

    pub fn send_test_notification(conn: &Connection) -> Result<(), ()> {
        let service = (
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
        );
        let proxy = conn.with_proxy(service.0, service.1, Duration::from_millis(5000));

        let res: Result<(), _> =
            proxy.method_call("org.freedesktop.Notifications", "GetServerInformation", ());

        let mut _hinst = HashMap::new();
        _hinst.insert(String::from("urgency"), to_variant(2));
        let payload = NotifyPayload {
            app_name: String::from("dbus-test"),
            replaces_id: 1010,
            app_icon: String::from("/usr/share/icons/hicolor/48x48/apps/org.xfce.about.png"),
            summary: String::from("test"),
            body: String::from("test"),
            actions: Vec::new(),
            hints: _hinst,
            expire_timeout: 5000,
        };

        let res: Result<(), _> = dbg!(proxy.method_call(
            "org.freedesktop.Notifications",
            "Notify",
            (
                payload.app_name,
                payload.replaces_id,
                payload.app_icon,
                payload.summary,
                payload.body,
                payload.actions,
                payload.hints,
                payload.expire_timeout,
            ),
        ));

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;

    // =========================================
    let active_services = list_of_services(&conn).unwrap();
    active_services
        .iter()
        .for_each(|service| println!("{}", service));

    // =========================================
    let res = send_test_notification(&conn);

    Ok(())
}
