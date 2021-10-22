// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files)
// DO NOT EDIT

use crate::Session;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "SoupSessionFeature")]
    pub struct SessionFeature(Interface<ffi::SoupSessionFeature, ffi::SoupSessionFeatureInterface>);

    match fn {
        type_ => || ffi::soup_session_feature_get_type(),
    }
}

pub const NONE_SESSION_FEATURE: Option<&SessionFeature> = None;

pub trait SessionFeatureExt: 'static {
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
    #[doc(alias = "soup_session_feature_add_feature")]
    fn add_feature(&self, type_: glib::types::Type) -> bool;

    #[doc(alias = "soup_session_feature_attach")]
    fn attach(&self, session: &impl IsA<Session>);

    #[doc(alias = "soup_session_feature_detach")]
    fn detach(&self, session: &impl IsA<Session>);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
    #[doc(alias = "soup_session_feature_has_feature")]
    fn has_feature(&self, type_: glib::types::Type) -> bool;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
    #[doc(alias = "soup_session_feature_remove_feature")]
    fn remove_feature(&self, type_: glib::types::Type) -> bool;
}

impl<O: IsA<SessionFeature>> SessionFeatureExt for O {
    #[cfg(any(feature = "v2_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
    fn add_feature(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::soup_session_feature_add_feature(self.as_ref().to_glib_none().0, type_.into_glib()))
        }
    }

    fn attach(&self, session: &impl IsA<Session>) {
        unsafe {
            ffi::soup_session_feature_attach(self.as_ref().to_glib_none().0, session.as_ref().to_glib_none().0);
        }
    }

    fn detach(&self, session: &impl IsA<Session>) {
        unsafe {
            ffi::soup_session_feature_detach(self.as_ref().to_glib_none().0, session.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
    fn has_feature(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::soup_session_feature_has_feature(self.as_ref().to_glib_none().0, type_.into_glib()))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_34")))]
    fn remove_feature(&self, type_: glib::types::Type) -> bool {
        unsafe {
            from_glib(ffi::soup_session_feature_remove_feature(self.as_ref().to_glib_none().0, type_.into_glib()))
        }
    }
}

impl fmt::Display for SessionFeature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SessionFeature")
    }
}