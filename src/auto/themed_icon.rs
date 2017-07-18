// This file was generated by gir (ce03df6) from gir-files (71d73f0)
// DO NOT EDIT

use Icon;
use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ThemedIcon(Object<ffi::GThemedIcon>): Icon;

    match fn {
        get_type => || ffi::g_themed_icon_get_type(),
    }
}

impl ThemedIcon {
    pub fn new(iconname: &str) -> ThemedIcon {
        unsafe {
            from_glib_full(ffi::g_themed_icon_new(iconname.to_glib_none().0))
        }
    }

    pub fn new_from_names(iconnames: &[&str]) -> ThemedIcon {
        let len = iconnames.len() as i32;
        unsafe {
            from_glib_full(ffi::g_themed_icon_new_from_names(iconnames.to_glib_none().0, len))
        }
    }

    pub fn new_with_default_fallbacks(iconname: &str) -> ThemedIcon {
        unsafe {
            from_glib_full(ffi::g_themed_icon_new_with_default_fallbacks(iconname.to_glib_none().0))
        }
    }
}

pub trait ThemedIconExt {
    fn append_name(&self, iconname: &str);

    fn get_names(&self) -> Vec<String>;

    fn prepend_name(&self, iconname: &str);

    fn get_property_use_default_fallbacks(&self) -> bool;
}

impl<O: IsA<ThemedIcon> + IsA<glib::object::Object>> ThemedIconExt for O {
    fn append_name(&self, iconname: &str) {
        unsafe {
            ffi::g_themed_icon_append_name(self.to_glib_none().0, iconname.to_glib_none().0);
        }
    }

    fn get_names(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_themed_icon_get_names(self.to_glib_none().0))
        }
    }

    fn prepend_name(&self, iconname: &str) {
        unsafe {
            ffi::g_themed_icon_prepend_name(self.to_glib_none().0, iconname.to_glib_none().0);
        }
    }

    fn get_property_use_default_fallbacks(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-default-fallbacks".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }
}
