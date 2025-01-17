// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files)
// DO NOT EDIT

use crate::Auth;
#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "SoupAuthNegotiate")]
    pub struct AuthNegotiate(Object<ffi::SoupAuthNegotiate>) @extends Auth;

    match fn {
        type_ => || ffi::soup_auth_negotiate_get_type(),
    }
}

impl AuthNegotiate {
    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "soup_auth_negotiate_supported")]
    pub fn supported() -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::soup_auth_negotiate_supported())
        }
    }
}

impl fmt::Display for AuthNegotiate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AuthNegotiate")
    }
}
