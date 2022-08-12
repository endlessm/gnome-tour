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
    pub struct BaseSink(Object<gst_base_sys::GstBaseSink, gst_base_sys::GstBaseSinkClass, BaseSinkClass>) @extends gst::Element, gst::Object;

    match fn {
        get_type => || gst_base_sys::gst_base_sink_get_type(),
    }
}

unsafe impl Send for BaseSink {}
unsafe impl Sync for BaseSink {}

pub const NONE_BASE_SINK: Option<&BaseSink> = None;

pub trait BaseSinkExt: 'static {
    //fn do_preroll(&self, obj: /*Ignored*/&gst::MiniObject) -> gst::FlowReturn;

    fn get_blocksize(&self) -> u32;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_drop_out_of_segment(&self) -> bool;

    fn get_last_sample(&self) -> Option<gst::Sample>;

    fn get_latency(&self) -> gst::ClockTime;

    fn get_max_bitrate(&self) -> u64;

    fn get_max_lateness(&self) -> i64;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_processing_deadline(&self) -> gst::ClockTime;

    fn get_render_delay(&self) -> gst::ClockTime;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn get_stats(&self) -> Option<gst::Structure>;

    fn get_sync(&self) -> bool;

    fn get_throttle_time(&self) -> u64;

    fn get_ts_offset(&self) -> gst::ClockTimeDiff;

    fn is_async_enabled(&self) -> bool;

    fn is_last_sample_enabled(&self) -> bool;

    fn is_qos_enabled(&self) -> bool;

    fn set_async_enabled(&self, enabled: bool);

    fn set_blocksize(&self, blocksize: u32);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_drop_out_of_segment(&self, drop_out_of_segment: bool);

    fn set_last_sample_enabled(&self, enabled: bool);

    fn set_max_bitrate(&self, max_bitrate: u64);

    fn set_max_lateness(&self, max_lateness: i64);

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_processing_deadline(&self, processing_deadline: gst::ClockTime);

    fn set_qos_enabled(&self, enabled: bool);

    fn set_render_delay(&self, delay: gst::ClockTime);

    fn set_sync(&self, sync: bool);

    fn set_throttle_time(&self, throttle: u64);

    fn set_ts_offset(&self, offset: gst::ClockTimeDiff);

    fn get_property_async(&self) -> bool;

    fn set_property_async(&self, async: bool);

    fn get_property_enable_last_sample(&self) -> bool;

    fn set_property_enable_last_sample(&self, enable_last_sample: bool);

    fn get_property_qos(&self) -> bool;

    fn set_property_qos(&self, qos: bool);

    fn connect_property_async_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_enable_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_max_bitrate_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_max_lateness_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn connect_property_processing_deadline_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_render_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn connect_property_stats_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_sync_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_throttle_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_ts_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<BaseSink>> BaseSinkExt for O {
    //fn do_preroll(&self, obj: /*Ignored*/&gst::MiniObject) -> gst::FlowReturn {
    //    unsafe { TODO: call gst_base_sys:gst_base_sink_do_preroll() }
    //}

    fn get_blocksize(&self) -> u32 {
        unsafe { gst_base_sys::gst_base_sink_get_blocksize(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_drop_out_of_segment(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_sink_get_drop_out_of_segment(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_last_sample(&self) -> Option<gst::Sample> {
        unsafe {
            from_glib_full(gst_base_sys::gst_base_sink_get_last_sample(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_latency(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_base_sys::gst_base_sink_get_latency(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_max_bitrate(&self) -> u64 {
        unsafe { gst_base_sys::gst_base_sink_get_max_bitrate(self.as_ref().to_glib_none().0) }
    }

    fn get_max_lateness(&self) -> i64 {
        unsafe { gst_base_sys::gst_base_sink_get_max_lateness(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn get_processing_deadline(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_base_sys::gst_base_sink_get_processing_deadline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_render_delay(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_base_sys::gst_base_sink_get_render_delay(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn get_stats(&self) -> Option<gst::Structure> {
        unsafe {
            from_glib_full(gst_base_sys::gst_base_sink_get_stats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_sync(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_sink_get_sync(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_throttle_time(&self) -> u64 {
        unsafe { gst_base_sys::gst_base_sink_get_throttle_time(self.as_ref().to_glib_none().0) }
    }

    fn get_ts_offset(&self) -> gst::ClockTimeDiff {
        unsafe { gst_base_sys::gst_base_sink_get_ts_offset(self.as_ref().to_glib_none().0) }
    }

    fn is_async_enabled(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_sink_is_async_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_last_sample_enabled(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_sink_is_last_sample_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_qos_enabled(&self) -> bool {
        unsafe {
            from_glib(gst_base_sys::gst_base_sink_is_qos_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_async_enabled(&self, enabled: bool) {
        unsafe {
            gst_base_sys::gst_base_sink_set_async_enabled(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_blocksize(&self, blocksize: u32) {
        unsafe {
            gst_base_sys::gst_base_sink_set_blocksize(self.as_ref().to_glib_none().0, blocksize);
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_drop_out_of_segment(&self, drop_out_of_segment: bool) {
        unsafe {
            gst_base_sys::gst_base_sink_set_drop_out_of_segment(
                self.as_ref().to_glib_none().0,
                drop_out_of_segment.to_glib(),
            );
        }
    }

    fn set_last_sample_enabled(&self, enabled: bool) {
        unsafe {
            gst_base_sys::gst_base_sink_set_last_sample_enabled(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_max_bitrate(&self, max_bitrate: u64) {
        unsafe {
            gst_base_sys::gst_base_sink_set_max_bitrate(
                self.as_ref().to_glib_none().0,
                max_bitrate,
            );
        }
    }

    fn set_max_lateness(&self, max_lateness: i64) {
        unsafe {
            gst_base_sys::gst_base_sink_set_max_lateness(
                self.as_ref().to_glib_none().0,
                max_lateness,
            );
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn set_processing_deadline(&self, processing_deadline: gst::ClockTime) {
        unsafe {
            gst_base_sys::gst_base_sink_set_processing_deadline(
                self.as_ref().to_glib_none().0,
                processing_deadline.to_glib(),
            );
        }
    }

    fn set_qos_enabled(&self, enabled: bool) {
        unsafe {
            gst_base_sys::gst_base_sink_set_qos_enabled(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_render_delay(&self, delay: gst::ClockTime) {
        unsafe {
            gst_base_sys::gst_base_sink_set_render_delay(
                self.as_ref().to_glib_none().0,
                delay.to_glib(),
            );
        }
    }

    fn set_sync(&self, sync: bool) {
        unsafe {
            gst_base_sys::gst_base_sink_set_sync(self.as_ref().to_glib_none().0, sync.to_glib());
        }
    }

    fn set_throttle_time(&self, throttle: u64) {
        unsafe {
            gst_base_sys::gst_base_sink_set_throttle_time(self.as_ref().to_glib_none().0, throttle);
        }
    }

    fn set_ts_offset(&self, offset: gst::ClockTimeDiff) {
        unsafe {
            gst_base_sys::gst_base_sink_set_ts_offset(self.as_ref().to_glib_none().0, offset);
        }
    }

    fn get_property_async(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"async\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `async` getter")
                .unwrap()
        }
    }

    fn set_property_async(&self, async: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"async\0".as_ptr() as *const _,
                Value::from(&async).to_glib_none().0,
            );
        }
    }

    fn get_property_enable_last_sample(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"enable-last-sample\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `enable-last-sample` getter")
                .unwrap()
        }
    }

    fn set_property_enable_last_sample(&self, enable_last_sample: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"enable-last-sample\0".as_ptr() as *const _,
                Value::from(&enable_last_sample).to_glib_none().0,
            );
        }
    }

    fn get_property_qos(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"qos\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `qos` getter")
                .unwrap()
        }
    }

    fn set_property_qos(&self, qos: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"qos\0".as_ptr() as *const _,
                Value::from(&qos).to_glib_none().0,
            );
        }
    }

    fn connect_property_async_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_async_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::async\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_async_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_blocksize_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_blocksize_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::blocksize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_blocksize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_enable_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enable_last_sample_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enable-last-sample\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enable_last_sample_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_last_sample_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_last_sample_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::last-sample\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_last_sample_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_bitrate_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_bitrate_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-bitrate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_bitrate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_lateness_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_lateness_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-lateness\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_lateness_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_16", feature = "dox"))]
    fn connect_property_processing_deadline_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_processing_deadline_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::processing-deadline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_processing_deadline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_qos_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_qos_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::qos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_qos_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_render_delay_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_render_delay_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::render-delay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_render_delay_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_18", feature = "dox"))]
    fn connect_property_stats_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_stats_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::stats\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_stats_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_sync_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_sync_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sync\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sync_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_throttle_time_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_throttle_time_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::throttle-time\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_throttle_time_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ts_offset_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ts_offset_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_base_sys::GstBaseSink,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BaseSink>,
        {
            let f: &F = &*(f as *const F);
            f(&BaseSink::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ts-offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ts_offset_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
