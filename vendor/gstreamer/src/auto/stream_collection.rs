// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use glib::GString;
use gst_sys;
use Object;
#[cfg(any(feature = "v1_10", feature = "dox"))]
use Stream;

glib_wrapper! {
    pub struct StreamCollection(Object<gst_sys::GstStreamCollection, gst_sys::GstStreamCollectionClass, StreamCollectionClass>) @extends Object;

    match fn {
        get_type => || gst_sys::gst_stream_collection_get_type(),
    }
}

impl StreamCollection {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_size(&self) -> u32 {
        unsafe { gst_sys::gst_stream_collection_get_size(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_stream(&self, index: u32) -> Option<Stream> {
        unsafe {
            from_glib_none(gst_sys::gst_stream_collection_get_stream(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    pub fn get_upstream_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gst_sys::gst_stream_collection_get_upstream_id(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn connect_stream_notify<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored p0: GObject.ParamSpec
    //}
}

unsafe impl Send for StreamCollection {}
unsafe impl Sync for StreamCollection {}
