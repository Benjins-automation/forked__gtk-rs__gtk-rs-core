// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::BusType;
use crate::Cancellable;
use crate::DBusCallFlags;
use crate::DBusConnection;
use crate::DBusInterface;
use crate::DBusInterfaceInfo;
use crate::DBusProxyFlags;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
use crate::UnixFDList;
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
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct DBusProxy(Object<ffi::GDBusProxy, ffi::GDBusProxyClass>) @implements DBusInterface;

    match fn {
        type_ => || ffi::g_dbus_proxy_get_type(),
    }
}

impl DBusProxy {
    #[doc(alias = "g_dbus_proxy_new_for_bus_sync")]
    pub fn for_bus_sync<P: IsA<Cancellable>>(
        bus_type: BusType,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: &str,
        object_path: &str,
        interface_name: &str,
        cancellable: Option<&P>,
    ) -> Result<DBusProxy, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_new_for_bus_sync(
                bus_type.to_glib(),
                flags.to_glib(),
                info.to_glib_none().0,
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_proxy_new_sync")]
    pub fn new_sync<P: IsA<Cancellable>>(
        connection: &DBusConnection,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        cancellable: Option<&P>,
    ) -> Result<DBusProxy, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_new_sync(
                connection.to_glib_none().0,
                flags.to_glib(),
                info.to_glib_none().0,
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_proxy_new")]
    pub fn new<P: IsA<Cancellable>, Q: FnOnce(Result<DBusProxy, glib::Error>) + Send + 'static>(
        connection: &DBusConnection,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn new_trampoline<
            Q: FnOnce(Result<DBusProxy, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_new_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_trampoline::<Q>;
        unsafe {
            ffi::g_dbus_proxy_new(
                connection.to_glib_none().0,
                flags.to_glib(),
                info.to_glib_none().0,
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_future(
        connection: &DBusConnection,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: Option<&str>,
        object_path: &str,
        interface_name: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<DBusProxy, glib::Error>> + 'static>> {
        let connection = connection.clone();
        let info = info.map(ToOwned::to_owned);
        let name = name.map(ToOwned::to_owned);
        let object_path = String::from(object_path);
        let interface_name = String::from(interface_name);
        Box_::pin(crate::GioFuture::new(&(), move |_obj, send| {
            let cancellable = Cancellable::new();
            Self::new(
                &connection,
                flags,
                info.as_ref().map(::std::borrow::Borrow::borrow),
                name.as_ref().map(::std::borrow::Borrow::borrow),
                &object_path,
                &interface_name,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    #[doc(alias = "g_dbus_proxy_new_for_bus")]
    pub fn new_for_bus<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<DBusProxy, glib::Error>) + Send + 'static,
    >(
        bus_type: BusType,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: &str,
        object_path: &str,
        interface_name: &str,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn new_for_bus_trampoline<
            Q: FnOnce(Result<DBusProxy, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_new_for_bus_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_for_bus_trampoline::<Q>;
        unsafe {
            ffi::g_dbus_proxy_new_for_bus(
                bus_type.to_glib(),
                flags.to_glib(),
                info.to_glib_none().0,
                name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_for_bus_future(
        bus_type: BusType,
        flags: DBusProxyFlags,
        info: Option<&DBusInterfaceInfo>,
        name: &str,
        object_path: &str,
        interface_name: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<DBusProxy, glib::Error>> + 'static>> {
        let info = info.map(ToOwned::to_owned);
        let name = String::from(name);
        let object_path = String::from(object_path);
        let interface_name = String::from(interface_name);
        Box_::pin(crate::GioFuture::new(&(), move |_obj, send| {
            let cancellable = Cancellable::new();
            Self::new_for_bus(
                bus_type,
                flags,
                info.as_ref().map(::std::borrow::Borrow::borrow),
                &name,
                &object_path,
                &interface_name,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }
}

impl fmt::Display for DBusProxy {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&DBusProxyExt::name(self))
    }
}

unsafe impl Send for DBusProxy {}
unsafe impl Sync for DBusProxy {}

pub const NONE_DBUS_PROXY: Option<&DBusProxy> = None;

pub trait DBusProxyExt: 'static {
    #[doc(alias = "g_dbus_proxy_call")]
    fn call<P: IsA<Cancellable>, Q: FnOnce(Result<glib::Variant, glib::Error>) + Send + 'static>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn call_future(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Variant, glib::Error>> + 'static>>;

    #[doc(alias = "g_dbus_proxy_call_sync")]
    fn call_sync<P: IsA<Cancellable>>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&P>,
    ) -> Result<glib::Variant, glib::Error>;

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_dbus_proxy_call_with_unix_fd_list")]
    fn call_with_unix_fd_list<
        P: IsA<UnixFDList>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(glib::Variant, UnixFDList), glib::Error>) + Send + 'static,
    >(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    );

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    fn call_with_unix_fd_list_future<P: IsA<UnixFDList> + Clone + 'static>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&P>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(glib::Variant, UnixFDList), glib::Error>>
                + 'static,
        >,
    >;

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_dbus_proxy_call_with_unix_fd_list_sync")]
    fn call_with_unix_fd_list_sync<P: IsA<UnixFDList>, Q: IsA<Cancellable>>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&P>,
        cancellable: Option<&Q>,
    ) -> Result<(glib::Variant, UnixFDList), glib::Error>;

    #[doc(alias = "g_dbus_proxy_get_cached_property")]
    fn cached_property(&self, property_name: &str) -> Option<glib::Variant>;

    #[doc(alias = "g_dbus_proxy_get_cached_property_names")]
    fn cached_property_names(&self) -> Vec<glib::GString>;

    #[doc(alias = "g_dbus_proxy_get_connection")]
    fn connection(&self) -> DBusConnection;

    #[doc(alias = "g_dbus_proxy_get_default_timeout")]
    fn default_timeout(&self) -> i32;

    #[doc(alias = "g_dbus_proxy_get_flags")]
    fn flags(&self) -> DBusProxyFlags;

    #[doc(alias = "g_dbus_proxy_get_interface_info")]
    fn interface_info(&self) -> Option<DBusInterfaceInfo>;

    #[doc(alias = "g_dbus_proxy_get_interface_name")]
    fn interface_name(&self) -> glib::GString;

    #[doc(alias = "g_dbus_proxy_get_name")]
    fn name(&self) -> glib::GString;

    #[doc(alias = "g_dbus_proxy_get_name_owner")]
    fn name_owner(&self) -> Option<glib::GString>;

    #[doc(alias = "g_dbus_proxy_get_object_path")]
    fn object_path(&self) -> glib::GString;

    #[doc(alias = "g_dbus_proxy_set_cached_property")]
    fn set_cached_property(&self, property_name: &str, value: Option<&glib::Variant>);

    #[doc(alias = "g_dbus_proxy_set_default_timeout")]
    fn set_default_timeout(&self, timeout_msec: i32);

    #[doc(alias = "g_dbus_proxy_set_interface_info")]
    fn set_interface_info(&self, info: Option<&DBusInterfaceInfo>);

    #[doc(alias = "get_property_g_connection")]
    fn g_connection(&self) -> Option<DBusConnection>;

    #[doc(alias = "get_property_g_default_timeout")]
    fn g_default_timeout(&self) -> i32;

    #[doc(alias = "set_property_g_default_timeout")]
    fn set_g_default_timeout(&self, g_default_timeout: i32);

    #[doc(alias = "get_property_g_flags")]
    fn g_flags(&self) -> DBusProxyFlags;

    #[doc(alias = "get_property_g_interface_info")]
    fn g_interface_info(&self) -> Option<DBusInterfaceInfo>;

    #[doc(alias = "set_property_g_interface_info")]
    fn set_g_interface_info(&self, g_interface_info: Option<&DBusInterfaceInfo>);

    #[doc(alias = "get_property_g_interface_name")]
    fn g_interface_name(&self) -> Option<glib::GString>;

    #[doc(alias = "get_property_g_name")]
    fn g_name(&self) -> Option<glib::GString>;

    #[doc(alias = "get_property_g_name_owner")]
    fn g_name_owner(&self) -> Option<glib::GString>;

    #[doc(alias = "get_property_g_object_path")]
    fn g_object_path(&self) -> Option<glib::GString>;

    fn connect_property_g_default_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_g_interface_info_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_g_name_owner_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DBusProxy>> DBusProxyExt for O {
    fn call<P: IsA<Cancellable>, Q: FnOnce(Result<glib::Variant, glib::Error>) + Send + 'static>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn call_trampoline<
            Q: FnOnce(Result<glib::Variant, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_call_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = call_trampoline::<Q>;
        unsafe {
            ffi::g_dbus_proxy_call(
                self.as_ref().to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                flags.to_glib(),
                timeout_msec,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn call_future(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Variant, glib::Error>> + 'static>>
    {
        let method_name = String::from(method_name);
        let parameters = parameters.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.call(
                &method_name,
                parameters.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                timeout_msec,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    fn call_sync<P: IsA<Cancellable>>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&P>,
    ) -> Result<glib::Variant, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_call_sync(
                self.as_ref().to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                flags.to_glib(),
                timeout_msec,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    fn call_with_unix_fd_list<
        P: IsA<UnixFDList>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(glib::Variant, UnixFDList), glib::Error>) + Send + 'static,
    >(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn call_with_unix_fd_list_trampoline<
            R: FnOnce(Result<(glib::Variant, UnixFDList), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut out_fd_list = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_call_with_unix_fd_list_finish(
                _source_object as *mut _,
                &mut out_fd_list,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(out_fd_list)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = call_with_unix_fd_list_trampoline::<R>;
        unsafe {
            ffi::g_dbus_proxy_call_with_unix_fd_list(
                self.as_ref().to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                flags.to_glib(),
                timeout_msec,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    fn call_with_unix_fd_list_future<P: IsA<UnixFDList> + Clone + 'static>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&P>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(glib::Variant, UnixFDList), glib::Error>>
                + 'static,
        >,
    > {
        let method_name = String::from(method_name);
        let parameters = parameters.map(ToOwned::to_owned);
        let fd_list = fd_list.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.call_with_unix_fd_list(
                &method_name,
                parameters.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                timeout_msec,
                fd_list.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    fn call_with_unix_fd_list_sync<P: IsA<UnixFDList>, Q: IsA<Cancellable>>(
        &self,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&P>,
        cancellable: Option<&Q>,
    ) -> Result<(glib::Variant, UnixFDList), glib::Error> {
        unsafe {
            let mut out_fd_list = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_proxy_call_with_unix_fd_list_sync(
                self.as_ref().to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                flags.to_glib(),
                timeout_msec,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
                &mut out_fd_list,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(out_fd_list)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn cached_property(&self, property_name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_dbus_proxy_get_cached_property(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    fn cached_property_names(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_dbus_proxy_get_cached_property_names(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connection(&self) -> DBusConnection {
        unsafe {
            from_glib_none(ffi::g_dbus_proxy_get_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn default_timeout(&self) -> i32 {
        unsafe { ffi::g_dbus_proxy_get_default_timeout(self.as_ref().to_glib_none().0) }
    }

    fn flags(&self) -> DBusProxyFlags {
        unsafe { from_glib(ffi::g_dbus_proxy_get_flags(self.as_ref().to_glib_none().0)) }
    }

    fn interface_info(&self) -> Option<DBusInterfaceInfo> {
        unsafe {
            from_glib_none(ffi::g_dbus_proxy_get_interface_info(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn interface_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_dbus_proxy_get_interface_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::g_dbus_proxy_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn name_owner(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_dbus_proxy_get_name_owner(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn object_path(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_dbus_proxy_get_object_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_cached_property(&self, property_name: &str, value: Option<&glib::Variant>) {
        unsafe {
            ffi::g_dbus_proxy_set_cached_property(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_default_timeout(&self, timeout_msec: i32) {
        unsafe {
            ffi::g_dbus_proxy_set_default_timeout(self.as_ref().to_glib_none().0, timeout_msec);
        }
    }

    fn set_interface_info(&self, info: Option<&DBusInterfaceInfo>) {
        unsafe {
            ffi::g_dbus_proxy_set_interface_info(
                self.as_ref().to_glib_none().0,
                info.to_glib_none().0,
            );
        }
    }

    fn g_connection(&self) -> Option<DBusConnection> {
        unsafe {
            let mut value = glib::Value::from_type(<DBusConnection as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-connection\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `g-connection` getter")
        }
    }

    fn g_default_timeout(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-default-timeout\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `g-default-timeout` getter")
        }
    }

    fn set_g_default_timeout(&self, g_default_timeout: i32) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-default-timeout\0".as_ptr() as *const _,
                g_default_timeout.to_value().to_glib_none().0,
            );
        }
    }

    fn g_flags(&self) -> DBusProxyFlags {
        unsafe {
            let mut value = glib::Value::from_type(<DBusProxyFlags as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-flags\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `g-flags` getter")
        }
    }

    fn g_interface_info(&self) -> Option<DBusInterfaceInfo> {
        unsafe {
            let mut value =
                glib::Value::from_type(<DBusInterfaceInfo as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-interface-info\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `g-interface-info` getter")
        }
    }

    fn set_g_interface_info(&self, g_interface_info: Option<&DBusInterfaceInfo>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-interface-info\0".as_ptr() as *const _,
                g_interface_info.to_value().to_glib_none().0,
            );
        }
    }

    fn g_interface_name(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-interface-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `g-interface-name` getter")
        }
    }

    fn g_name(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `g-name` getter")
        }
    }

    fn g_name_owner(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-name-owner\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `g-name-owner` getter")
        }
    }

    fn g_object_path(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"g-object-path\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `g-object-path` getter")
        }
    }

    fn connect_property_g_default_timeout_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_g_default_timeout_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusProxy,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DBusProxy>,
        {
            let f: &F = &*(f as *const F);
            f(&DBusProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::g-default-timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_g_default_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_g_interface_info_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_g_interface_info_trampoline<
            P,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusProxy,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DBusProxy>,
        {
            let f: &F = &*(f as *const F);
            f(&DBusProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::g-interface-info\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_g_interface_info_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_g_name_owner_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_g_name_owner_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut ffi::GDBusProxy,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<DBusProxy>,
        {
            let f: &F = &*(f as *const F);
            f(&DBusProxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::g-name-owner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_g_name_owner_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
