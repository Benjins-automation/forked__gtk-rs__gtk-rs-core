// This file was generated by gir (ce03df6) from gir-files (71d73f0)
// DO NOT EDIT

use Action;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SimpleAction(Object<ffi::GSimpleAction>): Action;

    match fn {
        get_type => || ffi::g_simple_action_get_type(),
    }
}

impl SimpleAction {
    pub fn new<'a, P: Into<Option<&'a glib::VariantTy>>>(name: &str, parameter_type: P) -> SimpleAction {
        let parameter_type = parameter_type.into();
        let parameter_type = parameter_type.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_simple_action_new(name.to_glib_none().0, parameter_type.0))
        }
    }

    pub fn new_stateful<'a, P: Into<Option<&'a glib::VariantTy>>>(name: &str, parameter_type: P, state: &glib::Variant) -> SimpleAction {
        let parameter_type = parameter_type.into();
        let parameter_type = parameter_type.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_simple_action_new_stateful(name.to_glib_none().0, parameter_type.0, state.to_glib_none().0))
        }
    }
}

pub trait SimpleActionExt {
    fn set_enabled(&self, enabled: bool);

    fn set_state(&self, value: &glib::Variant);

    #[cfg(feature = "v2_44")]
    fn set_state_hint<'a, P: Into<Option<&'a glib::Variant>>>(&self, state_hint: P);

    fn connect_activate<F: Fn(&Self, &Option<glib::Variant>) + 'static>(&self, f: F) -> u64;

    fn connect_change_state<F: Fn(&Self, &Option<glib::Variant>) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<SimpleAction> + IsA<glib::object::Object>> SimpleActionExt for O {
    fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::g_simple_action_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
    }

    fn set_state(&self, value: &glib::Variant) {
        unsafe {
            ffi::g_simple_action_set_state(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_44")]
    fn set_state_hint<'a, P: Into<Option<&'a glib::Variant>>>(&self, state_hint: P) {
        let state_hint = state_hint.into();
        let state_hint = state_hint.to_glib_none();
        unsafe {
            ffi::g_simple_action_set_state_hint(self.to_glib_none().0, state_hint.0);
        }
    }

    fn connect_activate<F: Fn(&Self, &Option<glib::Variant>) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Option<glib::Variant>) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_change_state<F: Fn(&Self, &Option<glib::Variant>) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Option<glib::Variant>) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "change-state",
                transmute(change_state_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GSimpleAction, parameter: *mut glib_ffi::GVariant, f: glib_ffi::gpointer)
where P: IsA<SimpleAction> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Option<glib::Variant>) + 'static> = transmute(f);
    f(&SimpleAction::from_glib_none(this).downcast_unchecked(), &from_glib_none(parameter))
}

unsafe extern "C" fn change_state_trampoline<P>(this: *mut ffi::GSimpleAction, value: *mut glib_ffi::GVariant, f: glib_ffi::gpointer)
where P: IsA<SimpleAction> {
    callback_guard!();
    let f: &Box_<Fn(&P, &Option<glib::Variant>) + 'static> = transmute(f);
    f(&SimpleAction::from_glib_none(this).downcast_unchecked(), &from_glib_none(value))
}
