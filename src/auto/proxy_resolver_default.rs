// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
use crate::SessionFeature;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

#[cfg(any(feature = "v2_24", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_24")))]
glib::wrapper! {
    #[doc(alias = "SoupProxyResolverDefault")]
    pub struct ProxyResolverDefault(Object<ffi::SoupProxyResolverDefault, ffi::SoupProxyResolverDefaultClass>) @implements SessionFeature;

    match fn {
        type_ => || ffi::soup_proxy_resolver_default_get_type(),
    }
}

#[cfg(not(any(feature = "v2_24", feature = "dox")))]
glib::wrapper! {
    #[doc(alias = "SoupProxyResolverDefault")]
    pub struct ProxyResolverDefault(Object<ffi::SoupProxyResolverDefault, ffi::SoupProxyResolverDefaultClass>);

    match fn {
        type_ => || ffi::soup_proxy_resolver_default_get_type(),
    }
}

pub const NONE_PROXY_RESOLVER_DEFAULT: Option<&ProxyResolverDefault> = None;

pub trait ProxyResolverDefaultExt: 'static {
    #[doc(alias = "gproxy-resolver")]
    fn set_gproxy_resolver<P: IsA<gio::ProxyResolver>>(&self, gproxy_resolver: Option<&P>);

    #[doc(alias = "gproxy-resolver")]
    fn connect_gproxy_resolver_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ProxyResolverDefault>> ProxyResolverDefaultExt for O {
    fn set_gproxy_resolver<P: IsA<gio::ProxyResolver>>(&self, gproxy_resolver: Option<&P>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"gproxy-resolver\0".as_ptr() as *const _, gproxy_resolver.to_value().to_glib_none().0);
        }
    }

    fn connect_gproxy_resolver_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gproxy_resolver_trampoline<P: IsA<ProxyResolverDefault>, F: Fn(&P) + 'static>(this: *mut ffi::SoupProxyResolverDefault, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(ProxyResolverDefault::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::gproxy-resolver\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_gproxy_resolver_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for ProxyResolverDefault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ProxyResolverDefault")
    }
}
