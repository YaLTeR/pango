// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 4740f5e)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct AttrList(Shared<ffi::PangoAttrList>);

    match fn {
        ref => |ptr| ffi::pango_attr_list_ref(ptr),
        unref => |ptr| ffi::pango_attr_list_unref(ptr),
        get_type => || ffi::pango_attr_list_get_type(),
    }
}

impl AttrList {
    pub fn new() -> AttrList {
        unsafe {
            from_glib_full(ffi::pango_attr_list_new())
        }
    }

    pub fn copy(&self) -> Option<AttrList> {
        unsafe {
            from_glib_full(ffi::pango_attr_list_copy(self.to_glib_none().0))
        }
    }

    //pub fn filter<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/AttrFilterFunc, data: P) -> Option<AttrList> {
    //    unsafe { TODO: call ffi::pango_attr_list_filter() }
    //}

    //pub fn get_iterator(&self) -> /*Ignored*/Option<AttrIterator> {
    //    unsafe { TODO: call ffi::pango_attr_list_get_iterator() }
    //}

    pub fn splice(&self, other: &AttrList, pos: i32, len: i32) {
        unsafe {
            ffi::pango_attr_list_splice(self.to_glib_none().0, other.to_glib_none().0, pos, len);
        }
    }
}

impl Default for AttrList {
    fn default() -> Self {
        Self::new()
    }
}
