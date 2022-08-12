// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst;
use gst_base_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct BaseParse(Object<gst_base_sys::GstBaseParse, gst_base_sys::GstBaseParseClass, BaseParseClass>) @extends gst::Element, gst::Object;

    match fn {
        get_type => || gst_base_sys::gst_base_parse_get_type(),
    }
}

unsafe impl Send for BaseParse {}
unsafe impl Sync for BaseParse {}

pub const NONE_BASE_PARSE: Option<&BaseParse> = None;

pub trait BaseParseExt: 'static {
    fn add_index_entry(&self, offset: u64, ts: gst::ClockTime, key: bool, force: bool) -> bool;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn drain(&self);

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode);

    fn set_average_bitrate(&self, bitrate: u32);

    fn set_has_timing_info(&self, has_timing: bool);

    fn set_infer_ts(&self, infer_ts: bool);

    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime);

    fn set_min_frame_size(&self, min_size: u32);

    fn set_passthrough(&self, passthrough: bool);

    fn set_pts_interpolation(&self, pts_interpolate: bool);

    fn set_syncable(&self, syncable: bool);

    fn set_ts_at_offset(&self, offset: usize);

    fn get_property_disable_passthrough(&self) -> bool;

    fn set_property_disable_passthrough(&self, disable_passthrough: bool);

    fn connect_property_disable_passthrough_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<BaseParse>> BaseParseExt for O {
    fn add_index_entry(&self, offset: u64, ts: gst::ClockTime, key: bool, force: bool) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_parse_add_index_entry(
                self.as_ref().to_glib_none().0,
                offset,
                ts.to_glib(),
                key.to_glib(),
                force.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn drain(&self) {
        unsafe {
            gst_base_sys::gst_base_parse_drain(self.as_ref().to_glib_none().0);
        }
    }

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            gst_base_sys::gst_base_parse_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn set_average_bitrate(&self, bitrate: u32) {
        unsafe {
            gst_base_sys::gst_base_parse_set_average_bitrate(
                self.as_ref().to_glib_none().0,
                bitrate,
            );
        }
    }

    fn set_has_timing_info(&self, has_timing: bool) {
        unsafe {
            gst_base_sys::gst_base_parse_set_has_timing_info(
                self.as_ref().to_glib_none().0,
                has_timing.to_glib(),
            );
        }
    }

    fn set_infer_ts(&self, infer_ts: bool) {
        unsafe {
            gst_base_sys::gst_base_parse_set_infer_ts(
                self.as_ref().to_glib_none().0,
                infer_ts.to_glib(),
            );
        }
    }

    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime) {
        unsafe {
            gst_base_sys::gst_base_parse_set_latency(
                self.as_ref().to_glib_none().0,
                min_latency.to_glib(),
                max_latency.to_glib(),
            );
        }
    }

    fn set_min_frame_size(&self, min_size: u32) {
        unsafe {
            gst_base_sys::gst_base_parse_set_min_frame_size(
                self.as_ref().to_glib_none().0,
                min_size,
            );
        }
    }

    fn set_passthrough(&self, passthrough: bool) {
        unsafe {
            gst_base_sys::gst_base_parse_set_passthrough(
                self.as_ref().to_glib_none().0,
                passthrough.to_glib(),
            );
        }
    }

    fn set_pts_interpolation(&self, pts_interpolate: bool) {
        unsafe {
            gst_base_sys::gst_base_parse_set_pts_interpolation(
                self.as_ref().to_glib_none().0,
                pts_interpolate.to_glib(),
            );
        }
    }

    fn set_syncable(&self, syncable: bool) {
        unsafe {
            gst_base_sys::gst_base_parse_set_syncable(
                self.as_ref().to_glib_none().0,
                syncable.to_glib(),
            );
        }
    }

    fn set_ts_at_offset(&self, offset: usize) {
        unsafe {
            gst_base_sys::gst_base_parse_set_ts_at_offset(self.as_ref().to_glib_none().0, offset);
        }
    }

    fn get_property_disable_passthrough(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"disable-passthrough\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `disable-passthrough` getter")
                .unwrap()
        }
    }

    fn set_property_disable_passthrough(&self, disable_passthrough: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"disable-passthrough\0".as_ptr() as *const _,
                Value::from(&disable_passthrough).to_glib_none().0,
            );
        }
    }

    fn connect_property_disable_passthrough_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_disable_passthrough_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_base_sys::GstBaseParse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseParse>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseParse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::disable-passthrough\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_disable_passthrough_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
