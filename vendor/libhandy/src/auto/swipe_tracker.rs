// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk;
use handy_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use NavigationDirection;
use Swipeable;

glib_wrapper! {
    pub struct SwipeTracker(Object<handy_sys::HdySwipeTracker, handy_sys::HdySwipeTrackerClass, SwipeTrackerClass>) @implements gtk::Orientable;

    match fn {
        get_type => || handy_sys::hdy_swipe_tracker_get_type(),
    }
}

impl SwipeTracker {
    pub fn new<P: IsA<Swipeable>>(swipeable: &P) -> SwipeTracker {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(handy_sys::hdy_swipe_tracker_new(
                swipeable.as_ref().to_glib_none().0,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct SwipeTrackerBuilder {
    allow_mouse_drag: Option<bool>,
    enabled: Option<bool>,
    reversed: Option<bool>,
    swipeable: Option<Swipeable>,
    orientation: Option<gtk::Orientation>,
}

impl SwipeTrackerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> SwipeTracker {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref allow_mouse_drag) = self.allow_mouse_drag {
            properties.push(("allow-mouse-drag", allow_mouse_drag));
        }
        if let Some(ref enabled) = self.enabled {
            properties.push(("enabled", enabled));
        }
        if let Some(ref reversed) = self.reversed {
            properties.push(("reversed", reversed));
        }
        if let Some(ref swipeable) = self.swipeable {
            properties.push(("swipeable", swipeable));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new(SwipeTracker::static_type(), &properties)
            .expect("object new")
            .downcast::<SwipeTracker>()
            .expect("downcast");
        ret
    }

    pub fn allow_mouse_drag(mut self, allow_mouse_drag: bool) -> Self {
        self.allow_mouse_drag = Some(allow_mouse_drag);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    pub fn reversed(mut self, reversed: bool) -> Self {
        self.reversed = Some(reversed);
        self
    }

    pub fn swipeable<P: IsA<Swipeable>>(mut self, swipeable: &P) -> Self {
        self.swipeable = Some(swipeable.clone().upcast());
        self
    }

    pub fn orientation(mut self, orientation: gtk::Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_SWIPE_TRACKER: Option<&SwipeTracker> = None;

pub trait SwipeTrackerExt: 'static {
    fn get_allow_mouse_drag(&self) -> bool;

    fn get_enabled(&self) -> bool;

    fn get_reversed(&self) -> bool;

    fn get_swipeable(&self) -> Option<Swipeable>;

    fn set_allow_mouse_drag(&self, allow_mouse_drag: bool);

    fn set_enabled(&self, enabled: bool);

    fn set_reversed(&self, reversed: bool);

    fn shift_position(&self, delta: f64);

    fn connect_begin_swipe<F: Fn(&Self, NavigationDirection, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_end_swipe<F: Fn(&Self, i64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_update_swipe<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_allow_mouse_drag_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reversed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SwipeTracker>> SwipeTrackerExt for O {
    fn get_allow_mouse_drag(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_swipe_tracker_get_allow_mouse_drag(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_enabled(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_swipe_tracker_get_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_reversed(&self) -> bool {
        unsafe {
            from_glib(handy_sys::hdy_swipe_tracker_get_reversed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_swipeable(&self) -> Option<Swipeable> {
        unsafe {
            from_glib_none(handy_sys::hdy_swipe_tracker_get_swipeable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_allow_mouse_drag(&self, allow_mouse_drag: bool) {
        unsafe {
            handy_sys::hdy_swipe_tracker_set_allow_mouse_drag(
                self.as_ref().to_glib_none().0,
                allow_mouse_drag.to_glib(),
            );
        }
    }

    fn set_enabled(&self, enabled: bool) {
        unsafe {
            handy_sys::hdy_swipe_tracker_set_enabled(
                self.as_ref().to_glib_none().0,
                enabled.to_glib(),
            );
        }
    }

    fn set_reversed(&self, reversed: bool) {
        unsafe {
            handy_sys::hdy_swipe_tracker_set_reversed(
                self.as_ref().to_glib_none().0,
                reversed.to_glib(),
            );
        }
    }

    fn shift_position(&self, delta: f64) {
        unsafe {
            handy_sys::hdy_swipe_tracker_shift_position(self.as_ref().to_glib_none().0, delta);
        }
    }

    fn connect_begin_swipe<F: Fn(&Self, NavigationDirection, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn begin_swipe_trampoline<
            P,
            F: Fn(&P, NavigationDirection, bool) + 'static,
        >(
            this: *mut handy_sys::HdySwipeTracker,
            direction: handy_sys::HdyNavigationDirection,
            direct: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SwipeTracker>,
        {
            let f: &F = &*(f as *const F);
            f(
                &SwipeTracker::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(direction),
                from_glib(direct),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"begin-swipe\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    begin_swipe_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_end_swipe<F: Fn(&Self, i64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn end_swipe_trampoline<P, F: Fn(&P, i64, f64) + 'static>(
            this: *mut handy_sys::HdySwipeTracker,
            duration: i64,
            to: libc::c_double,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SwipeTracker>,
        {
            let f: &F = &*(f as *const F);
            f(
                &SwipeTracker::from_glib_borrow(this).unsafe_cast_ref(),
                duration,
                to,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"end-swipe\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    end_swipe_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_update_swipe<F: Fn(&Self, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn update_swipe_trampoline<P, F: Fn(&P, f64) + 'static>(
            this: *mut handy_sys::HdySwipeTracker,
            progress: libc::c_double,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SwipeTracker>,
        {
            let f: &F = &*(f as *const F);
            f(
                &SwipeTracker::from_glib_borrow(this).unsafe_cast_ref(),
                progress,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"update-swipe\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    update_swipe_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_allow_mouse_drag_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_allow_mouse_drag_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdySwipeTracker,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SwipeTracker>,
        {
            let f: &F = &*(f as *const F);
            f(&SwipeTracker::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::allow-mouse-drag\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_allow_mouse_drag_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdySwipeTracker,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SwipeTracker>,
        {
            let f: &F = &*(f as *const F);
            f(&SwipeTracker::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_reversed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reversed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut handy_sys::HdySwipeTracker,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SwipeTracker>,
        {
            let f: &F = &*(f as *const F);
            f(&SwipeTracker::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reversed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reversed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SwipeTracker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SwipeTracker")
    }
}