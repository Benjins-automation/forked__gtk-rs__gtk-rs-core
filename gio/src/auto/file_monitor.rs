// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::File;
use crate::FileMonitorEvent;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct FileMonitor(Object<ffi::GFileMonitor, ffi::GFileMonitorClass>);

    match fn {
        type_ => || ffi::g_file_monitor_get_type(),
    }
}

pub const NONE_FILE_MONITOR: Option<&FileMonitor> = None;

pub trait FileMonitorExt: 'static {
    #[doc(alias = "g_file_monitor_cancel")]
    fn cancel(&self) -> bool;

    #[doc(alias = "g_file_monitor_emit_event")]
    fn emit_event<P: IsA<File>, Q: IsA<File>>(
        &self,
        child: &P,
        other_file: &Q,
        event_type: FileMonitorEvent,
    );

    #[doc(alias = "g_file_monitor_is_cancelled")]
    fn is_cancelled(&self) -> bool;

    #[doc(alias = "g_file_monitor_set_rate_limit")]
    fn set_rate_limit(&self, limit_msecs: i32);

    #[doc(alias = "get_property_rate_limit")]
    fn rate_limit(&self) -> i32;

    fn connect_changed<F: Fn(&Self, &File, Option<&File>, FileMonitorEvent) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_cancelled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rate_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileMonitor>> FileMonitorExt for O {
    fn cancel(&self) -> bool {
        unsafe { from_glib(ffi::g_file_monitor_cancel(self.as_ref().to_glib_none().0)) }
    }

    fn emit_event<P: IsA<File>, Q: IsA<File>>(
        &self,
        child: &P,
        other_file: &Q,
        event_type: FileMonitorEvent,
    ) {
        unsafe {
            ffi::g_file_monitor_emit_event(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                other_file.as_ref().to_glib_none().0,
                event_type.to_glib(),
            );
        }
    }

    fn is_cancelled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_file_monitor_is_cancelled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_rate_limit(&self, limit_msecs: i32) {
        unsafe {
            ffi::g_file_monitor_set_rate_limit(self.as_ref().to_glib_none().0, limit_msecs);
        }
    }

    fn rate_limit(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"rate-limit\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `rate-limit` getter")
        }
    }

    fn connect_changed<F: Fn(&Self, &File, Option<&File>, FileMonitorEvent) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<
            P,
            F: Fn(&P, &File, Option<&File>, FileMonitorEvent) + 'static,
        >(
            this: *mut ffi::GFileMonitor,
            file: *mut ffi::GFile,
            other_file: *mut ffi::GFile,
            event_type: ffi::GFileMonitorEvent,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileMonitor>,
        {
            let f: &F = &*(f as *const F);
            f(
                &FileMonitor::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(file),
                Option::<File>::from_glib_borrow(other_file)
                    .as_ref()
                    .as_ref(),
                from_glib(event_type),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_cancelled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cancelled_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GFileMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileMonitor>,
        {
            let f: &F = &*(f as *const F);
            f(&FileMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::cancelled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cancelled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_rate_limit_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rate_limit_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GFileMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileMonitor>,
        {
            let f: &F = &*(f as *const F);
            f(&FileMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rate-limit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rate_limit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FileMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileMonitor")
    }
}
