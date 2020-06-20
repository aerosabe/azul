    #![allow(dead_code, unused_imports)]
    //! Definition of azuls internal `Option<*>` wrappers
    use crate::dll::*;
    use std::ffi::c_void;


    /// `OptionWaylandTheme` struct
    pub use crate::dll::AzOptionWaylandTheme as OptionWaylandTheme;

    impl std::fmt::Debug for OptionWaylandTheme { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_wayland_theme_fmt_debug)(self)) } }
    impl Clone for OptionWaylandTheme { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_wayland_theme_deep_copy)(self) } }
    impl Drop for OptionWaylandTheme { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_wayland_theme_delete)(self); } }


    /// `OptionTaskBarIcon` struct
    pub use crate::dll::AzOptionTaskBarIcon as OptionTaskBarIcon;

    impl std::fmt::Debug for OptionTaskBarIcon { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_task_bar_icon_fmt_debug)(self)) } }
    impl Clone for OptionTaskBarIcon { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_task_bar_icon_deep_copy)(self) } }
    impl Drop for OptionTaskBarIcon { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_task_bar_icon_delete)(self); } }


    /// `OptionHwndHandle` struct
    pub use crate::dll::AzOptionHwndHandle as OptionHwndHandle;

    impl std::fmt::Debug for OptionHwndHandle { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_hwnd_handle_fmt_debug)(self)) } }
    impl Clone for OptionHwndHandle { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_hwnd_handle_deep_copy)(self) } }
    impl Drop for OptionHwndHandle { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_hwnd_handle_delete)(self); } }


    /// `OptionLogicalPosition` struct
    pub use crate::dll::AzOptionLogicalPosition as OptionLogicalPosition;

    impl std::fmt::Debug for OptionLogicalPosition { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_logical_position_fmt_debug)(self)) } }
    impl Clone for OptionLogicalPosition { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_logical_position_deep_copy)(self) } }
    impl Drop for OptionLogicalPosition { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_logical_position_delete)(self); } }


    /// `OptionHotReloadOptions` struct
    pub use crate::dll::AzOptionHotReloadOptions as OptionHotReloadOptions;

    impl std::fmt::Debug for OptionHotReloadOptions { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_hot_reload_options_fmt_debug)(self)) } }
    impl Clone for OptionHotReloadOptions { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_hot_reload_options_deep_copy)(self) } }
    impl Drop for OptionHotReloadOptions { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_hot_reload_options_delete)(self); } }


    /// `OptionPhysicalPositionI32` struct
    pub use crate::dll::AzOptionPhysicalPositionI32 as OptionPhysicalPositionI32;

    impl std::fmt::Debug for OptionPhysicalPositionI32 { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_physical_position_i32_fmt_debug)(self)) } }
    impl Clone for OptionPhysicalPositionI32 { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_physical_position_i32_deep_copy)(self) } }
    impl Drop for OptionPhysicalPositionI32 { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_physical_position_i32_delete)(self); } }


    /// `OptionWindowIcon` struct
    pub use crate::dll::AzOptionWindowIcon as OptionWindowIcon;

    impl std::fmt::Debug for OptionWindowIcon { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_window_icon_fmt_debug)(self)) } }
    impl Clone for OptionWindowIcon { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_window_icon_deep_copy)(self) } }
    impl Drop for OptionWindowIcon { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_window_icon_delete)(self); } }


    /// `OptionString` struct
    pub use crate::dll::AzOptionString as OptionString;

    impl std::fmt::Debug for OptionString { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_string_fmt_debug)(self)) } }
    impl Clone for OptionString { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_string_deep_copy)(self) } }
    impl Drop for OptionString { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_string_delete)(self); } }


    /// `OptionX11Visual` struct
    pub use crate::dll::AzOptionX11Visual as OptionX11Visual;

    impl std::fmt::Debug for OptionX11Visual { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_x11_visual_fmt_debug)(self)) } }
    impl Clone for OptionX11Visual { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_x11_visual_deep_copy)(self) } }
    impl Drop for OptionX11Visual { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_x11_visual_delete)(self); } }


    /// `OptionI32` struct
    pub use crate::dll::AzOptionI32 as OptionI32;

    impl std::fmt::Debug for OptionI32 { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_i32_fmt_debug)(self)) } }
    impl Clone for OptionI32 { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_i32_deep_copy)(self) } }
    impl Drop for OptionI32 { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_i32_delete)(self); } }


    /// `OptionF32` struct
    pub use crate::dll::AzOptionF32 as OptionF32;

    impl std::fmt::Debug for OptionF32 { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_f32_fmt_debug)(self)) } }
    impl Clone for OptionF32 { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_f32_deep_copy)(self) } }
    impl Drop for OptionF32 { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_f32_delete)(self); } }


    /// `OptionMouseCursorType` struct
    pub use crate::dll::AzOptionMouseCursorType as OptionMouseCursorType;

    impl std::fmt::Debug for OptionMouseCursorType { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_mouse_cursor_type_fmt_debug)(self)) } }
    impl Clone for OptionMouseCursorType { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_mouse_cursor_type_deep_copy)(self) } }
    impl Drop for OptionMouseCursorType { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_mouse_cursor_type_delete)(self); } }


    /// `OptionLogicalSize` struct
    pub use crate::dll::AzOptionLogicalSize as OptionLogicalSize;

    impl std::fmt::Debug for OptionLogicalSize { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_logical_size_fmt_debug)(self)) } }
    impl Clone for OptionLogicalSize { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_logical_size_deep_copy)(self) } }
    impl Drop for OptionLogicalSize { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_logical_size_delete)(self); } }


    /// `OptionChar` struct
    pub use crate::dll::AzOptionChar as OptionChar;

    impl std::fmt::Debug for OptionChar { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_char_fmt_debug)(self)) } }
    impl Clone for OptionChar { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_char_deep_copy)(self) } }
    impl Drop for OptionChar { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_char_delete)(self); } }


    /// `OptionVirtualKeyCode` struct
    pub use crate::dll::AzOptionVirtualKeyCode as OptionVirtualKeyCode;

    impl std::fmt::Debug for OptionVirtualKeyCode { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_virtual_key_code_fmt_debug)(self)) } }
    impl Clone for OptionVirtualKeyCode { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_virtual_key_code_deep_copy)(self) } }
    impl Drop for OptionVirtualKeyCode { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_virtual_key_code_delete)(self); } }


    /// `OptionPercentageValue` struct
    pub use crate::dll::AzOptionPercentageValue as OptionPercentageValue;

    impl std::fmt::Debug for OptionPercentageValue { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_percentage_value_fmt_debug)(self)) } }
    impl Clone for OptionPercentageValue { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_percentage_value_deep_copy)(self) } }
    impl Drop for OptionPercentageValue { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_percentage_value_delete)(self); } }


    /// `OptionDom` struct
    pub use crate::dll::AzOptionDom as OptionDom;

    impl std::fmt::Debug for OptionDom { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_dom_fmt_debug)(self)) } }
    impl Clone for OptionDom { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_dom_deep_copy)(self) } }
    impl Drop for OptionDom { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_dom_delete)(self); } }


    /// `OptionTexture` struct
    pub use crate::dll::AzOptionTexture as OptionTexture;

    impl std::fmt::Debug for OptionTexture { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_texture_fmt_debug)(self)) } }
    impl Drop for OptionTexture { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_texture_delete)(self); } }


    /// `OptionTabIndex` struct
    pub use crate::dll::AzOptionTabIndex as OptionTabIndex;

    impl std::fmt::Debug for OptionTabIndex { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_tab_index_fmt_debug)(self)) } }
    impl Clone for OptionTabIndex { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_tab_index_deep_copy)(self) } }
    impl Drop for OptionTabIndex { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_tab_index_delete)(self); } }


    /// `OptionDuration` struct
    pub use crate::dll::AzOptionDuration as OptionDuration;

    impl std::fmt::Debug for OptionDuration { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_duration_fmt_debug)(self)) } }
    impl Clone for OptionDuration { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_duration_deep_copy)(self) } }
    impl Drop for OptionDuration { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_duration_delete)(self); } }


    /// `OptionInstantPtr` struct
    pub use crate::dll::AzOptionInstantPtr as OptionInstantPtr;

    impl std::fmt::Debug for OptionInstantPtr { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_instant_ptr_fmt_debug)(self)) } }
    impl Clone for OptionInstantPtr { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_instant_ptr_deep_copy)(self) } }
    impl Drop for OptionInstantPtr { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_instant_ptr_delete)(self); } }


    /// `OptionUsize` struct
    pub use crate::dll::AzOptionUsize as OptionUsize;

    impl std::fmt::Debug for OptionUsize { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_usize_fmt_debug)(self)) } }
    impl Clone for OptionUsize { fn clone(&self) -> Self { (crate::dll::get_azul_dll().az_option_usize_deep_copy)(self) } }
    impl Drop for OptionUsize { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_usize_delete)(self); } }


    /// `OptionU8VecRef` struct
    pub use crate::dll::AzOptionU8VecRef as OptionU8VecRef;

    impl std::fmt::Debug for OptionU8VecRef { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", (crate::dll::get_azul_dll().az_option_u8_vec_ref_fmt_debug)(self)) } }
    impl Drop for OptionU8VecRef { fn drop(&mut self) { (crate::dll::get_azul_dll().az_option_u8_vec_ref_delete)(self); } }