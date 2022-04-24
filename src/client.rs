use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;

use dbus::{arg, blocking::Connection, Message};
use dbus::strings::Path;

use crate::client_gen::{OrgFreedesktopDBusProperties, OrgFreedesktopGeoClue2Client, OrgFreedesktopGeoClue2ClientLocationUpdated};
use crate::manager_gen::OrgFreedesktopGeoClue2Manager;

const GEOCLUE2_BUS_NAME: &str = "org.freedesktop.GeoClue2";
const WAYDROD_LOCATION_PATH: &str = "/var/lib/waydroid/rootfs/tmp/location_data.json";

fn refarg_to_str(value: &dyn arg::RefArg) -> String {
    // TODO: Handle datetime object
    return
        if let Some(s) = value.as_str() {
            String::from(s)
        } else if let Some(i) = value.as_i64() {
            i.to_string()
        } else if let Some(i) = value.as_f64() {
            i.to_string()
        } else {
            String::from("UNKNOWN")
        };
}

fn dump(location_data_str: String) -> std::io::Result<()> {
    // TODO: Make this safe
    let mut file = File::create(WAYDROD_LOCATION_PATH)?;
    file.write_all(location_data_str.as_bytes())?;
    Ok(())
}

fn new_location_trigger(conn: &Connection, location_last: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("new location data available: {}", location_last);
    let location_proxy = conn.with_proxy(GEOCLUE2_BUS_NAME, location_last.clone(), Duration::from_millis(5000));

    let mut serial_location_props: HashMap<String, String> = HashMap::new();
    let location_props: arg::PropMap = location_proxy.get_all("org.freedesktop.GeoClue2.Location")?;
    for (k, v) in location_props.iter() {
        let key = String::from(k);
        let value = refarg_to_str(&v);
        serial_location_props.insert(key.clone(), value.clone());
    }

    let j = serde_json::to_string(&serial_location_props)?;
    println!("{}", j);

    dump(j)?;

    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // get dbus connection
    let conn = Connection::new_system()?;
    // get the manager
    let manager_proxy = conn.with_proxy(GEOCLUE2_BUS_NAME, "/org/freedesktop/GeoClue2/Manager", Duration::from_millis(5000));

    // make a new client/get our old client
    let client_path: Path = manager_proxy.get_client()?;

    println!("client path: {}", client_path);

    let client_proxy = conn.with_proxy(GEOCLUE2_BUS_NAME, client_path.clone(), Duration::from_millis(5000));
    client_proxy.set_desktop_id(String::from("waydroid_geoclue_bridge"))?;
    client_proxy.set_distance_threshold(0)?;

    // Let's start listening to signals.
    println!("setting listener");
    let _id = client_proxy.match_signal(|h: OrgFreedesktopGeoClue2ClientLocationUpdated, c: &Connection, m: &Message| {
        match m.path() {
            Some(client_path) => {
                let client_proxy = c.with_proxy(GEOCLUE2_BUS_NAME, client_path.clone(), Duration::from_millis(5000));
                match client_proxy.location() {
                    Ok(location) => {
                        let result = new_location_trigger(&c, &location);
                        match result {
                            Ok(_) => { println!("Location update parsed.") }
                            Err(e) => { eprintln!("Couldn't send location further: {}", e) }
                        }
                    }
                    Err(e) => {
                        eprintln!("Couldn't get the location data on signal trigger: {}", e)
                    }
                }
            }
            _ => {}
        }
        true
    });

    println!("starting client");
    client_proxy.start()?;

    // Listen to incoming signals forever.
    loop {
        conn.process(Duration::from_millis(1000))?;
    }
}
