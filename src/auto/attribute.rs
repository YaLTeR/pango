// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 4740f5e)
// DO NOT EDIT

use AttrClass;
use ffi;
use glib::translate::*;
use glib_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Attribute(Boxed<ffi::PangoAttribute>);

    match fn {
        copy => |ptr| ffi::pango_attribute_copy(mut_override(ptr)),
        free => |ptr| ffi::pango_attribute_destroy(ptr),
    }
}

impl Attribute {
    fn equal(&self, attr2: &Attribute) -> bool {
        unsafe {
            from_glib(ffi::pango_attribute_equal(self.to_glib_none().0, attr2.to_glib_none().0))
        }
    }

    pub fn init(&mut self, klass: &AttrClass) {
        unsafe {
            ffi::pango_attribute_init(self.to_glib_none_mut().0, klass.to_glib_none().0);
        }
    }
}

impl PartialEq for Attribute {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Attribute {}
