// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::translate::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FrameTimings(Shared<gdk_sys::GdkFrameTimings>);

    match fn {
        ref => |ptr| gdk_sys::gdk_frame_timings_ref(ptr),
        unref => |ptr| gdk_sys::gdk_frame_timings_unref(ptr),
        get_type => || gdk_sys::gdk_frame_timings_get_type(),
    }
}

impl FrameTimings {
    pub fn get_complete(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_frame_timings_get_complete(self.to_glib_none().0))
        }
    }

    pub fn get_frame_counter(&self) -> i64 {
        unsafe {
            gdk_sys::gdk_frame_timings_get_frame_counter(self.to_glib_none().0)
        }
    }

    pub fn get_frame_time(&self) -> i64 {
        unsafe {
            gdk_sys::gdk_frame_timings_get_frame_time(self.to_glib_none().0)
        }
    }
}
