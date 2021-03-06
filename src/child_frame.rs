use std::marker::PhantomData;

use crate::legacy::ImGuiWindowFlags;
use crate::sys;
use crate::{ImStr, Ui};

#[must_use]
pub struct ChildFrame<'ui, 'p> {
    name: &'p ImStr,
    size: [f32; 2],
    border: bool,
    flags: ImGuiWindowFlags,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui, 'p> ChildFrame<'ui, 'p> {
    pub fn new(_: &Ui<'ui>, name: &'p ImStr, size: [f32; 2]) -> ChildFrame<'ui, 'p> {
        ChildFrame {
            name,
            size: size.into(),
            border: false,
            flags: ImGuiWindowFlags::empty(),
            _phantom: PhantomData,
        }
    }
    #[inline]
    pub fn movable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoMove, !value);
        self
    }
    #[inline]
    pub fn show_scrollbar(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoScrollbar, !value);
        self
    }
    #[inline]
    pub fn show_scrollbar_with_mouse(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoScrollWithMouse, !value);
        self
    }
    #[inline]
    pub fn collapsible(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoCollapse, !value);
        self
    }
    #[inline]
    pub fn always_resizable(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::AlwaysAutoResize, value);
        self
    }
    #[inline]
    pub fn show_borders(mut self, value: bool) -> Self {
        self.border = value;
        self
    }
    #[inline]
    pub fn input_allow(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoInputs, !value);
        self
    }
    #[inline]
    pub fn show_menu(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::MenuBar, value);
        self
    }
    #[inline]
    pub fn scrollbar_horizontal(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::HorizontalScrollbar, value);
        self
    }
    #[inline]
    pub fn focus_on_appearing(mut self, value: bool) -> Self {
        self.flags.set(ImGuiWindowFlags::NoFocusOnAppearing, !value);
        self
    }
    #[inline]
    pub fn bring_to_front_on_focus(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::NoBringToFrontOnFocus, !value);
        self
    }
    #[inline]
    pub fn always_show_vertical_scroll_bar(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::AlwaysVerticalScrollbar, value);
        self
    }
    #[inline]
    pub fn always_show_horizontal_scroll_bar(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::AlwaysHorizontalScrollbar, value);
        self
    }
    #[inline]
    pub fn always_use_window_padding(mut self, value: bool) -> Self {
        self.flags
            .set(ImGuiWindowFlags::AlwaysUseWindowPadding, value);
        self
    }
    pub fn build<F: FnOnce()>(self, f: F) {
        let render_child_frame = unsafe {
            sys::igBeginChild(
                self.name.as_ptr(),
                self.size.into(),
                self.border,
                self.flags.bits(),
            )
        };
        if render_child_frame {
            f();
        }
        unsafe { sys::igEndChild() };
    }
}
