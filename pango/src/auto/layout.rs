// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use pango_sys;
use std::fmt;
use std::mem;
use Alignment;
use AttrList;
use Context;
#[cfg(any(feature = "v1_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
use Direction;
use EllipsizeMode;
use FontDescription;
use LayoutIter;
use LayoutLine;
use Rectangle;
use TabArray;
use WrapMode;

glib_wrapper! {
    pub struct Layout(Object<pango_sys::PangoLayout, pango_sys::PangoLayoutClass>);

    match fn {
        get_type => || pango_sys::pango_layout_get_type(),
    }
}

impl Layout {
    pub fn new(context: &Context) -> Layout {
        unsafe { from_glib_full(pango_sys::pango_layout_new(context.to_glib_none().0)) }
    }

    pub fn context_changed(&self) {
        unsafe {
            pango_sys::pango_layout_context_changed(self.to_glib_none().0);
        }
    }

    pub fn copy(&self) -> Option<Layout> {
        unsafe { from_glib_full(pango_sys::pango_layout_copy(self.to_glib_none().0)) }
    }

    pub fn get_alignment(&self) -> Alignment {
        unsafe { from_glib(pango_sys::pango_layout_get_alignment(self.to_glib_none().0)) }
    }

    pub fn get_attributes(&self) -> Option<AttrList> {
        unsafe {
            from_glib_none(pango_sys::pango_layout_get_attributes(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_auto_dir(&self) -> bool {
        unsafe { from_glib(pango_sys::pango_layout_get_auto_dir(self.to_glib_none().0)) }
    }

    pub fn get_baseline(&self) -> i32 {
        unsafe { pango_sys::pango_layout_get_baseline(self.to_glib_none().0) }
    }

    pub fn get_character_count(&self) -> i32 {
        unsafe { pango_sys::pango_layout_get_character_count(self.to_glib_none().0) }
    }

    pub fn get_context(&self) -> Option<Context> {
        unsafe { from_glib_none(pango_sys::pango_layout_get_context(self.to_glib_none().0)) }
    }

    pub fn get_cursor_pos(&self, index_: i32) -> (Rectangle, Rectangle) {
        unsafe {
            let mut strong_pos = Rectangle::uninitialized();
            let mut weak_pos = Rectangle::uninitialized();
            pango_sys::pango_layout_get_cursor_pos(
                self.to_glib_none().0,
                index_,
                strong_pos.to_glib_none_mut().0,
                weak_pos.to_glib_none_mut().0,
            );
            (strong_pos, weak_pos)
        }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    pub fn get_direction(&self, index: i32) -> Direction {
        unsafe {
            from_glib(pango_sys::pango_layout_get_direction(
                self.to_glib_none().0,
                index,
            ))
        }
    }

    pub fn get_ellipsize(&self) -> EllipsizeMode {
        unsafe { from_glib(pango_sys::pango_layout_get_ellipsize(self.to_glib_none().0)) }
    }

    pub fn get_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            pango_sys::pango_layout_get_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    pub fn get_font_description(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_none(pango_sys::pango_layout_get_font_description(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { pango_sys::pango_layout_get_height(self.to_glib_none().0) }
    }

    pub fn get_indent(&self) -> i32 {
        unsafe { pango_sys::pango_layout_get_indent(self.to_glib_none().0) }
    }

    pub fn get_iter(&self) -> Option<LayoutIter> {
        unsafe { from_glib_full(pango_sys::pango_layout_get_iter(self.to_glib_none().0)) }
    }

    pub fn get_justify(&self) -> bool {
        unsafe { from_glib(pango_sys::pango_layout_get_justify(self.to_glib_none().0)) }
    }

    pub fn get_line(&self, line: i32) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(pango_sys::pango_layout_get_line(
                self.to_glib_none().0,
                line,
            ))
        }
    }

    pub fn get_line_count(&self) -> i32 {
        unsafe { pango_sys::pango_layout_get_line_count(self.to_glib_none().0) }
    }

    pub fn get_line_readonly(&self, line: i32) -> Option<LayoutLine> {
        unsafe {
            from_glib_none(pango_sys::pango_layout_get_line_readonly(
                self.to_glib_none().0,
                line,
            ))
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    pub fn get_line_spacing(&self) -> f32 {
        unsafe { pango_sys::pango_layout_get_line_spacing(self.to_glib_none().0) }
    }

    pub fn get_lines(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(pango_sys::pango_layout_get_lines(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_lines_readonly(&self) -> Vec<LayoutLine> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(pango_sys::pango_layout_get_lines_readonly(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn get_log_attrs(&self, attrs: /*Ignored*/Vec<LogAttr>) -> i32 {
    //    unsafe { TODO: call pango_sys:pango_layout_get_log_attrs() }
    //}

    //pub fn get_log_attrs_readonly(&self) -> /*Ignored*/Vec<LogAttr> {
    //    unsafe { TODO: call pango_sys:pango_layout_get_log_attrs_readonly() }
    //}

    pub fn get_pixel_extents(&self) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            pango_sys::pango_layout_get_pixel_extents(
                self.to_glib_none().0,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    pub fn get_pixel_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            pango_sys::pango_layout_get_pixel_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    pub fn get_serial(&self) -> u32 {
        unsafe { pango_sys::pango_layout_get_serial(self.to_glib_none().0) }
    }

    pub fn get_single_paragraph_mode(&self) -> bool {
        unsafe {
            from_glib(pango_sys::pango_layout_get_single_paragraph_mode(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            pango_sys::pango_layout_get_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe { pango_sys::pango_layout_get_spacing(self.to_glib_none().0) }
    }

    pub fn get_tabs(&self) -> Option<TabArray> {
        unsafe { from_glib_full(pango_sys::pango_layout_get_tabs(self.to_glib_none().0)) }
    }

    pub fn get_text(&self) -> Option<GString> {
        unsafe { from_glib_none(pango_sys::pango_layout_get_text(self.to_glib_none().0)) }
    }

    pub fn get_unknown_glyphs_count(&self) -> i32 {
        unsafe { pango_sys::pango_layout_get_unknown_glyphs_count(self.to_glib_none().0) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { pango_sys::pango_layout_get_width(self.to_glib_none().0) }
    }

    pub fn get_wrap(&self) -> WrapMode {
        unsafe { from_glib(pango_sys::pango_layout_get_wrap(self.to_glib_none().0)) }
    }

    pub fn index_to_line_x(&self, index_: i32, trailing: bool) -> (i32, i32) {
        unsafe {
            let mut line = mem::MaybeUninit::uninit();
            let mut x_pos = mem::MaybeUninit::uninit();
            pango_sys::pango_layout_index_to_line_x(
                self.to_glib_none().0,
                index_,
                trailing.to_glib(),
                line.as_mut_ptr(),
                x_pos.as_mut_ptr(),
            );
            let line = line.assume_init();
            let x_pos = x_pos.assume_init();
            (line, x_pos)
        }
    }

    pub fn index_to_pos(&self, index_: i32) -> Rectangle {
        unsafe {
            let mut pos = Rectangle::uninitialized();
            pango_sys::pango_layout_index_to_pos(
                self.to_glib_none().0,
                index_,
                pos.to_glib_none_mut().0,
            );
            pos
        }
    }

    pub fn is_ellipsized(&self) -> bool {
        unsafe { from_glib(pango_sys::pango_layout_is_ellipsized(self.to_glib_none().0)) }
    }

    pub fn is_wrapped(&self) -> bool {
        unsafe { from_glib(pango_sys::pango_layout_is_wrapped(self.to_glib_none().0)) }
    }

    pub fn move_cursor_visually(
        &self,
        strong: bool,
        old_index: i32,
        old_trailing: i32,
        direction: i32,
    ) -> (i32, i32) {
        unsafe {
            let mut new_index = mem::MaybeUninit::uninit();
            let mut new_trailing = mem::MaybeUninit::uninit();
            pango_sys::pango_layout_move_cursor_visually(
                self.to_glib_none().0,
                strong.to_glib(),
                old_index,
                old_trailing,
                direction,
                new_index.as_mut_ptr(),
                new_trailing.as_mut_ptr(),
            );
            let new_index = new_index.assume_init();
            let new_trailing = new_trailing.assume_init();
            (new_index, new_trailing)
        }
    }

    pub fn set_alignment(&self, alignment: Alignment) {
        unsafe {
            pango_sys::pango_layout_set_alignment(self.to_glib_none().0, alignment.to_glib());
        }
    }

    pub fn set_attributes(&self, attrs: Option<&AttrList>) {
        unsafe {
            pango_sys::pango_layout_set_attributes(self.to_glib_none().0, attrs.to_glib_none().0);
        }
    }

    pub fn set_auto_dir(&self, auto_dir: bool) {
        unsafe {
            pango_sys::pango_layout_set_auto_dir(self.to_glib_none().0, auto_dir.to_glib());
        }
    }

    pub fn set_ellipsize(&self, ellipsize: EllipsizeMode) {
        unsafe {
            pango_sys::pango_layout_set_ellipsize(self.to_glib_none().0, ellipsize.to_glib());
        }
    }

    pub fn set_font_description(&self, desc: Option<&FontDescription>) {
        unsafe {
            pango_sys::pango_layout_set_font_description(
                self.to_glib_none().0,
                desc.to_glib_none().0,
            );
        }
    }

    pub fn set_height(&self, height: i32) {
        unsafe {
            pango_sys::pango_layout_set_height(self.to_glib_none().0, height);
        }
    }

    pub fn set_indent(&self, indent: i32) {
        unsafe {
            pango_sys::pango_layout_set_indent(self.to_glib_none().0, indent);
        }
    }

    pub fn set_justify(&self, justify: bool) {
        unsafe {
            pango_sys::pango_layout_set_justify(self.to_glib_none().0, justify.to_glib());
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    pub fn set_line_spacing(&self, factor: f32) {
        unsafe {
            pango_sys::pango_layout_set_line_spacing(self.to_glib_none().0, factor);
        }
    }

    pub fn set_markup(&self, markup: &str) {
        let length = markup.len() as i32;
        unsafe {
            pango_sys::pango_layout_set_markup(
                self.to_glib_none().0,
                markup.to_glib_none().0,
                length,
            );
        }
    }

    pub fn set_markup_with_accel(&self, markup: &str, accel_marker: char) -> char {
        let length = markup.len() as i32;
        unsafe {
            let mut accel_char = mem::MaybeUninit::uninit();
            pango_sys::pango_layout_set_markup_with_accel(
                self.to_glib_none().0,
                markup.to_glib_none().0,
                length,
                accel_marker.to_glib(),
                accel_char.as_mut_ptr(),
            );
            let accel_char = accel_char.assume_init();
            std::convert::TryFrom::try_from(accel_char)
                .expect("conversion from an invalid Unicode value attempted")
        }
    }

    pub fn set_single_paragraph_mode(&self, setting: bool) {
        unsafe {
            pango_sys::pango_layout_set_single_paragraph_mode(
                self.to_glib_none().0,
                setting.to_glib(),
            );
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            pango_sys::pango_layout_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    pub fn set_tabs(&self, tabs: Option<&TabArray>) {
        unsafe {
            pango_sys::pango_layout_set_tabs(
                self.to_glib_none().0,
                mut_override(tabs.to_glib_none().0),
            );
        }
    }

    pub fn set_text(&self, text: &str) {
        let length = text.len() as i32;
        unsafe {
            pango_sys::pango_layout_set_text(self.to_glib_none().0, text.to_glib_none().0, length);
        }
    }

    pub fn set_width(&self, width: i32) {
        unsafe {
            pango_sys::pango_layout_set_width(self.to_glib_none().0, width);
        }
    }

    pub fn set_wrap(&self, wrap: WrapMode) {
        unsafe {
            pango_sys::pango_layout_set_wrap(self.to_glib_none().0, wrap.to_glib());
        }
    }

    pub fn xy_to_index(&self, x: i32, y: i32) -> (bool, i32, i32) {
        unsafe {
            let mut index_ = mem::MaybeUninit::uninit();
            let mut trailing = mem::MaybeUninit::uninit();
            let ret = from_glib(pango_sys::pango_layout_xy_to_index(
                self.to_glib_none().0,
                x,
                y,
                index_.as_mut_ptr(),
                trailing.as_mut_ptr(),
            ));
            let index_ = index_.assume_init();
            let trailing = trailing.assume_init();
            (ret, index_, trailing)
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Layout")
    }
}
