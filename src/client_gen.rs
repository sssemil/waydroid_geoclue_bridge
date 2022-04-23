// This code was autogenerated with `dbus-codegen-rust -s -d org.freedesktop.GeoClue2 -p /org/freedesktop/GeoClue2/Client/1`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusProperties for blocking::Proxy<'a, C> {
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Get", (interface_name, property_name, ))
            .and_then(|r: (arg::Variant<Box<dyn arg::RefArg + 'static>>, )| Ok(r.0))
    }

    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "GetAll", (interface_name, ))
            .and_then(|r: (arg::PropMap, )| Ok(r.0))
    }

    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Properties", "Set", (interface_name, property_name, value, ))
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C> {
    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String, )| Ok(r.0))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopDBusPeer for blocking::Proxy<'a, C> {
    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String, )| Ok(r.0))
    }
}

pub trait OrgFreedesktopGeoClue2Client {
    fn start(&self) -> Result<(), dbus::Error>;
    fn stop(&self) -> Result<(), dbus::Error>;
    fn location(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn distance_threshold(&self) -> Result<u32, dbus::Error>;
    fn set_distance_threshold(&self, value: u32) -> Result<(), dbus::Error>;
    fn time_threshold(&self) -> Result<u32, dbus::Error>;
    fn set_time_threshold(&self, value: u32) -> Result<(), dbus::Error>;
    fn desktop_id(&self) -> Result<String, dbus::Error>;
    fn set_desktop_id(&self, value: String) -> Result<(), dbus::Error>;
    fn requested_accuracy_level(&self) -> Result<u32, dbus::Error>;
    fn set_requested_accuracy_level(&self, value: u32) -> Result<(), dbus::Error>;
    fn active(&self) -> Result<bool, dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopGeoClue2ClientLocationUpdated {
    pub old: dbus::Path<'static>,
    pub new: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopGeoClue2ClientLocationUpdated {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.old, i);
        arg::RefArg::append(&self.new, i);
    }
}

impl arg::ReadAll for OrgFreedesktopGeoClue2ClientLocationUpdated {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopGeoClue2ClientLocationUpdated {
            old: i.read()?,
            new: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopGeoClue2ClientLocationUpdated {
    const NAME: &'static str = "LocationUpdated";
    const INTERFACE: &'static str = "org.freedesktop.GeoClue2.Client";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgFreedesktopGeoClue2Client for blocking::Proxy<'a, C> {
    fn start(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.GeoClue2.Client", "Start", ())
    }

    fn stop(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.GeoClue2.Client", "Stop", ())
    }

    fn location(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Client", "Location")
    }

    fn distance_threshold(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Client", "DistanceThreshold")
    }

    fn time_threshold(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Client", "TimeThreshold")
    }

    fn desktop_id(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Client", "DesktopId")
    }

    fn requested_accuracy_level(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Client", "RequestedAccuracyLevel")
    }

    fn active(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.freedesktop.GeoClue2.Client", "Active")
    }

    fn set_distance_threshold(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.GeoClue2.Client", "DistanceThreshold", value)
    }

    fn set_time_threshold(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.GeoClue2.Client", "TimeThreshold", value)
    }

    fn set_desktop_id(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.GeoClue2.Client", "DesktopId", value)
    }

    fn set_requested_accuracy_level(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.freedesktop.GeoClue2.Client", "RequestedAccuracyLevel", value)
    }
}
