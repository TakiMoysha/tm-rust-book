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

    use dbus::arg::Variant;
    use dbus::blocking::Connection;

    // https://github.com/KDE/liquidshell/blob/master/org.freedesktop.Notifications.xml
    struct NotifyPayload {
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: HashMap<String, Variant<String>>,
        expire_timeout: i32,
    }

    pub fn send_test_notification(conn: &Connection) -> Result<(), ()> {
        let service = (
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
        );
        let proxy = conn.with_proxy(service.0, service.1, Duration::from_millis(5000));

        let res: Result<(), _> =
            proxy.method_call("org.freedesktop.Notifications", "GetServerInformation", ());

        let payload = NotifyPayload {
            app_name: String::from("dbus-test"),
            replaces_id: 1010,
            app_icon: String::from(
                "$HOME/.local/share/icons/hicolor/32x32/apps/steam_icon_2590260.png",
            ),
            summary: String::from("test"),
            body: String::from("test"),
            actions: Vec::new(),
            hints: HashMap::new(),
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
        .map(|service| println!("{}", service));

    // =========================================
    let res = send_test_notification(&conn);

    Ok(())
}
