// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::translate::*;
use glib::GString;
use pango_sys;
use std::mem;
use std::ptr;
use Analysis;
use AttrIterator;
use AttrList;
use Context;
use Direction;
use GlyphString;
use Item;
use Rectangle;
use Stretch;
use Style;
use Variant;
use Weight;

//#[cfg_attr(feature = "v1_44", deprecated)]
//pub fn break_(text: &str, analysis: &mut Analysis, attrs: /*Ignored*/&[&LogAttr]) {
//    unsafe { TODO: call pango_sys:pango_break() }
//}

//pub fn default_break(text: &str, analysis: Option<&mut Analysis>, attrs: /*Ignored*/&mut LogAttr, attrs_len: i32) {
//    unsafe { TODO: call pango_sys:pango_default_break() }
//}

pub fn extents_to_pixels(inclusive: Option<&Rectangle>, nearest: Option<&Rectangle>) {
    unsafe {
        pango_sys::pango_extents_to_pixels(
            mut_override(inclusive.to_glib_none().0),
            mut_override(nearest.to_glib_none().0),
        );
    }
}

pub fn find_base_dir(text: &str) -> Direction {
    let length = text.len() as i32;
    unsafe {
        from_glib(pango_sys::pango_find_base_dir(
            text.to_glib_none().0,
            length,
        ))
    }
}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn find_map(language: &mut Language, engine_type_id: u32, render_type_id: u32) -> /*Ignored*/Option<Map> {
//    unsafe { TODO: call pango_sys:pango_find_map() }
//}

pub fn find_paragraph_boundary(text: &str) -> (i32, i32) {
    let length = text.len() as i32;
    unsafe {
        let mut paragraph_delimiter_index = mem::MaybeUninit::uninit();
        let mut next_paragraph_start = mem::MaybeUninit::uninit();
        pango_sys::pango_find_paragraph_boundary(
            text.to_glib_none().0,
            length,
            paragraph_delimiter_index.as_mut_ptr(),
            next_paragraph_start.as_mut_ptr(),
        );
        let paragraph_delimiter_index = paragraph_delimiter_index.assume_init();
        let next_paragraph_start = next_paragraph_start.assume_init();
        (paragraph_delimiter_index, next_paragraph_start)
    }
}

//pub fn get_log_attrs(text: &str, level: i32, language: &mut Language, log_attrs: /*Ignored*/&[&LogAttr]) {
//    unsafe { TODO: call pango_sys:pango_get_log_attrs() }
//}

pub fn is_zero_width(ch: char) -> bool {
    unsafe { from_glib(pango_sys::pango_is_zero_width(ch.to_glib())) }
}

pub fn itemize(
    context: &Context,
    text: &str,
    start_index: i32,
    length: i32,
    attrs: &AttrList,
    cached_iter: Option<&AttrIterator>,
) -> Vec<Item> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(pango_sys::pango_itemize(
            context.to_glib_none().0,
            text.to_glib_none().0,
            start_index,
            length,
            attrs.to_glib_none().0,
            mut_override(cached_iter.to_glib_none().0),
        ))
    }
}

pub fn itemize_with_base_dir(
    context: &Context,
    base_dir: Direction,
    text: &str,
    start_index: i32,
    length: i32,
    attrs: &AttrList,
    cached_iter: Option<&AttrIterator>,
) -> Vec<Item> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(pango_sys::pango_itemize_with_base_dir(
            context.to_glib_none().0,
            base_dir.to_glib(),
            text.to_glib_none().0,
            start_index,
            length,
            attrs.to_glib_none().0,
            mut_override(cached_iter.to_glib_none().0),
        ))
    }
}

//pub fn markup_parser_finish(context: /*Ignored*/&glib::MarkupParseContext) -> Result<(AttrList, GString, char), glib::Error> {
//    unsafe { TODO: call pango_sys:pango_markup_parser_finish() }
//}

//pub fn markup_parser_new(accel_marker: char) -> /*Ignored*/Option<glib::MarkupParseContext> {
//    unsafe { TODO: call pango_sys:pango_markup_parser_new() }
//}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn module_register(module: /*Ignored*/&mut IncludedModule) {
//    unsafe { TODO: call pango_sys:pango_module_register() }
//}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn parse_enum(
    type_: glib::types::Type,
    str: Option<&str>,
    warn: bool,
) -> Option<(i32, GString)> {
    unsafe {
        let mut value = mem::MaybeUninit::uninit();
        let mut possible_values = ptr::null_mut();
        let ret = from_glib(pango_sys::pango_parse_enum(
            type_.to_glib(),
            str.to_glib_none().0,
            value.as_mut_ptr(),
            warn.to_glib(),
            &mut possible_values,
        ));
        let value = value.assume_init();
        if ret {
            Some((value, from_glib_full(possible_values)))
        } else {
            None
        }
    }
}

pub fn parse_markup(
    markup_text: &str,
    accel_marker: char,
) -> Result<(AttrList, GString, char), glib::Error> {
    let length = markup_text.len() as i32;
    unsafe {
        let mut attr_list = ptr::null_mut();
        let mut text = ptr::null_mut();
        let mut accel_char = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = pango_sys::pango_parse_markup(
            markup_text.to_glib_none().0,
            length,
            accel_marker.to_glib(),
            &mut attr_list,
            &mut text,
            accel_char.as_mut_ptr(),
            &mut error,
        );
        let accel_char = accel_char.assume_init();
        if error.is_null() {
            Ok((
                from_glib_full(attr_list),
                from_glib_full(text),
                std::convert::TryFrom::try_from(accel_char)
                    .expect("conversion from an invalid Unicode value attempted"),
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn parse_stretch(str: &str, warn: bool) -> Option<Stretch> {
    unsafe {
        let mut stretch = mem::MaybeUninit::uninit();
        let ret = from_glib(pango_sys::pango_parse_stretch(
            str.to_glib_none().0,
            stretch.as_mut_ptr(),
            warn.to_glib(),
        ));
        let stretch = stretch.assume_init();
        if ret {
            Some(from_glib(stretch))
        } else {
            None
        }
    }
}

pub fn parse_style(str: &str, warn: bool) -> Option<Style> {
    unsafe {
        let mut style = mem::MaybeUninit::uninit();
        let ret = from_glib(pango_sys::pango_parse_style(
            str.to_glib_none().0,
            style.as_mut_ptr(),
            warn.to_glib(),
        ));
        let style = style.assume_init();
        if ret {
            Some(from_glib(style))
        } else {
            None
        }
    }
}

pub fn parse_variant(str: &str, warn: bool) -> Option<Variant> {
    unsafe {
        let mut variant = mem::MaybeUninit::uninit();
        let ret = from_glib(pango_sys::pango_parse_variant(
            str.to_glib_none().0,
            variant.as_mut_ptr(),
            warn.to_glib(),
        ));
        let variant = variant.assume_init();
        if ret {
            Some(from_glib(variant))
        } else {
            None
        }
    }
}

pub fn parse_weight(str: &str, warn: bool) -> Option<Weight> {
    unsafe {
        let mut weight = mem::MaybeUninit::uninit();
        let ret = from_glib(pango_sys::pango_parse_weight(
            str.to_glib_none().0,
            weight.as_mut_ptr(),
            warn.to_glib(),
        ));
        let weight = weight.assume_init();
        if ret {
            Some(from_glib(weight))
        } else {
            None
        }
    }
}

pub fn quantize_line_geometry(thickness: &mut i32, position: &mut i32) {
    unsafe {
        pango_sys::pango_quantize_line_geometry(thickness, position);
    }
}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn read_line(stream: /*Unimplemented*/Option<Fundamental: Pointer>, str: /*Ignored*/&mut glib::String) -> i32 {
//    unsafe { TODO: call pango_sys:pango_read_line() }
//}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn scan_int(pos: /*Unimplemented*/GString) -> Option<i32> {
//    unsafe { TODO: call pango_sys:pango_scan_int() }
//}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn scan_string(pos: /*Unimplemented*/GString, out: /*Ignored*/&mut glib::String) -> bool {
//    unsafe { TODO: call pango_sys:pango_scan_string() }
//}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn scan_word(pos: /*Unimplemented*/GString, out: /*Ignored*/&mut glib::String) -> bool {
//    unsafe { TODO: call pango_sys:pango_scan_word() }
//}

pub fn shape(text: &str, analysis: &Analysis, glyphs: &mut GlyphString) {
    let length = text.len() as i32;
    unsafe {
        pango_sys::pango_shape(
            text.to_glib_none().0,
            length,
            analysis.to_glib_none().0,
            glyphs.to_glib_none_mut().0,
        );
    }
}

//#[cfg_attr(feature = "v1_38", deprecated)]
//pub fn skip_space(pos: /*Unimplemented*/GString) -> bool {
//    unsafe { TODO: call pango_sys:pango_skip_space() }
//}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn split_file_list(str: &str) -> Vec<GString> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(pango_sys::pango_split_file_list(str.to_glib_none().0))
    }
}

//#[cfg(any(feature = "v1_44", feature = "dox"))]
//#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
//pub fn tailor_break(text: &str, analysis: &mut Analysis, offset: i32, log_attrs: /*Ignored*/&[&LogAttr]) {
//    unsafe { TODO: call pango_sys:pango_tailor_break() }
//}

#[cfg_attr(feature = "v1_38", deprecated)]
pub fn trim_string(str: &str) -> Option<GString> {
    unsafe { from_glib_full(pango_sys::pango_trim_string(str.to_glib_none().0)) }
}

pub fn unichar_direction(ch: char) -> Direction {
    unsafe { from_glib(pango_sys::pango_unichar_direction(ch.to_glib())) }
}

pub fn units_from_double(d: f64) -> i32 {
    unsafe { pango_sys::pango_units_from_double(d) }
}

pub fn units_to_double(i: i32) -> f64 {
    unsafe { pango_sys::pango_units_to_double(i) }
}

pub fn version() -> i32 {
    unsafe { pango_sys::pango_version() }
}

pub fn version_check(
    required_major: i32,
    required_minor: i32,
    required_micro: i32,
) -> Option<GString> {
    unsafe {
        from_glib_none(pango_sys::pango_version_check(
            required_major,
            required_minor,
            required_micro,
        ))
    }
}

pub fn version_string() -> Option<GString> {
    unsafe { from_glib_none(pango_sys::pango_version_string()) }
}
