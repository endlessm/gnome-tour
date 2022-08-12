// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use DBusInterfaceInfo;
use DBusObject;

glib_wrapper! {
    pub struct DBusInterface(Interface<gio_sys::GDBusInterface>);

    match fn {
        get_type => || gio_sys::g_dbus_interface_get_type(),
    }
}

pub const NONE_DBUS_INTERFACE: Option<&DBusInterface> = None;

pub trait DBusInterfaceExt: 'static {
    fn get(&self) -> Option<DBusObject>;

    fn get_info(&self) -> Option<DBusInterfaceInfo>;

    fn set_object<P: IsA<DBusObject>>(&self, object: Option<&P>);
}

impl<O: IsA<DBusInterface>> DBusInterfaceExt for O {
    fn get(&self) -> Option<DBusObject> {
        unsafe {
            from_glib_full(gio_sys::g_dbus_interface_dup_object(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_info(&self) -> Option<DBusInterfaceInfo> {
        unsafe {
            from_glib_none(gio_sys::g_dbus_interface_get_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_object<P: IsA<DBusObject>>(&self, object: Option<&P>) {
        unsafe {
            gio_sys::g_dbus_interface_set_object(
                self.as_ref().to_glib_none().0,
                object.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for DBusInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBusInterface")
    }
}
