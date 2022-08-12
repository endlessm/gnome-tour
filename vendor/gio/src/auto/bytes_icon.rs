// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::translate::*;
use std::fmt;
use Icon;
use LoadableIcon;

glib_wrapper! {
    pub struct BytesIcon(Object<gio_sys::GBytesIcon, BytesIconClass>) @implements Icon, LoadableIcon;

    match fn {
        get_type => || gio_sys::g_bytes_icon_get_type(),
    }
}

impl BytesIcon {
    pub fn new(bytes: &glib::Bytes) -> BytesIcon {
        unsafe { from_glib_full(gio_sys::g_bytes_icon_new(bytes.to_glib_none().0)) }
    }

    pub fn get_bytes(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_none(gio_sys::g_bytes_icon_get_bytes(self.to_glib_none().0)) }
    }
}

impl fmt::Display for BytesIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BytesIcon")
    }
}
