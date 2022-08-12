// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use gtk;
use handy_sys;

pub fn ease_out_cubic(t: f64) -> f64 {
    assert_initialized_main_thread!();
    unsafe { handy_sys::hdy_ease_out_cubic(t) }
}

//pub fn enum_value_row_name(value: /*Ignored*/&EnumValueObject, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Option<GString> {
//    unsafe { TODO: call handy_sys:hdy_enum_value_row_name() }
//}

pub fn get_enable_animations<P: IsA<gtk::Widget>>(widget: &P) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(handy_sys::hdy_get_enable_animations(
            widget.as_ref().to_glib_none().0,
        ))
    }
}

pub fn init() {
    assert_initialized_main_thread!();
    unsafe {
        handy_sys::hdy_init();
    }
}