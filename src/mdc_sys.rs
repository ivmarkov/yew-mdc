use wasm_bindgen::prelude::*;
use web_sys::Element;

#[cfg(any(feature = "button", feature = "fab"))]
#[wasm_bindgen(module = "@material/ripple/index")]
extern "C" {
    /// MDC Ripple provides the JavaScript and CSS required to provide components
    /// (or any element at all) with a material “ink ripple” interaction effect.
    /// It is designed to be efficient, uninvasive, and usable without adding
    /// any extra DOM to your elements.
    ///
    /// MDC Ripple also works without JavaScript, where it gracefully degrades
    /// to a simpler CSS-Only implementation.
    pub type MDCRipple;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCRipple;

    #[wasm_bindgen(method, getter)]
    pub fn unbounded(this: &MDCRipple) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_unbounded(this: &MDCRipple, unbounded: bool);

    #[wasm_bindgen(method)]
    pub fn activate(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn deactivate(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn layout(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn handle_focus(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn handle_blur(this: &MDCRipple);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &MDCRipple);
}

#[cfg(feature = "text-field")]
#[wasm_bindgen(module = "@material/textfield/index")]
extern "C" {
    /// Text fields allow users to input, edit, and select text.
    pub type MDCTextField;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCTextField;

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &MDCTextField) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &MDCTextField, value: String);

    #[wasm_bindgen(method, getter)]
    pub fn disabled(this: &MDCTextField) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_disabled(this: &MDCTextField, disabled: bool);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &MDCTextField);
}

#[cfg(feature = "menu")]
#[wasm_bindgen(module = "@material/menu/index")]
extern "C" {
    /// A menu displays a list of choices on a temporary surface.
    /// They appear when users interact with a button, action, or other control.
    pub type MDCMenu;

    #[wasm_bindgen(constructor)]
    pub fn new(surface: Element) -> MDCMenu;

    #[wasm_bindgen(method, getter)]
    pub fn open(this: &MDCMenu) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_open(this: &MDCMenu, open: bool);

    #[wasm_bindgen(method, getter)]
    pub fn items(this: &MDCMenu) -> js_sys::Array;

    #[wasm_bindgen(method, getter)]
    pub fn quick_open(this: &MDCMenu) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_quick_open(this: &MDCMenu, quick_open: bool);

    #[wasm_bindgen(method, getter)]
    pub fn wrap_focus(this: &MDCMenu) -> bool;
    #[wasm_bindgen(method, setter)]
    pub fn set_wrap_focus(this: &MDCMenu, wrap_focus: bool);

    #[wasm_bindgen(method)]
    pub fn set_anchor_corner(this: &MDCMenu, corner: js_sys::Object);
    #[wasm_bindgen(method)]
    pub fn set_anchor_margin(this: &MDCMenu, distance: js_sys::Object);

    #[wasm_bindgen(method)]
    pub fn set_absolute_position(this: &MDCMenu, x: i32, y: i32);

    #[wasm_bindgen(method)]
    pub fn set_fixed_position(this: &MDCMenu, is_fixed: bool);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &MDCMenu);
}

// #[cfg(feature = "text-field")]
// #[wasm_bindgen(module = "@material/notched-outline/index")]
// extern "C" {
//     /// The notched outline is a border around all sides of either a Text Field
//     /// or Select component. This is used for the Outlined variant of either a
//     /// Text Field or Select.
//     pub type MDCNotchedOutline;

//     #[wasm_bindgen(constructor)]
//     pub fn new(surface: Element) -> MDCNotchedOutline;

//     #[wasm_bindgen(method)]
//     pub fn notch(this: &MDCNotchedOutline, notch_width: u32);

//     #[wasm_bindgen(method)]
//     pub fn close_notch(this: &MDCNotchedOutline);

//     #[wasm_bindgen(method)]
//     pub fn destroy(this: &MDCNotchedOutline);
// }

// Commented out for now - might want to revisit later in case we need more
// sophisticated text fields.

// #[cfg(feature = "text-field")]
// #[wasm_bindgen(module = "@material/line-ripple/index")]
// extern "C" {
//     /// The line ripple is used to highlight user-specified input above it.
//     /// When a line ripple is active, the line’s color and thickness changes.
//     pub type MDCLineRipple;

//     #[wasm_bindgen(constructor)]
//     pub fn new(surface: Element) -> MDCLineRipple;

//     #[wasm_bindgen(method)]
//     pub fn activate(this: &MDCLineRipple);

//     #[wasm_bindgen(method)]
//     pub fn deactivate(this: &MDCLineRipple);

//     #[wasm_bindgen(method)]
//     pub fn set_ripple_center(this: &MDCLineRipple, xCoordinate: js_sys::Number);

//     #[wasm_bindgen(method)]
//     pub fn destroy(this: &MDCLineRipple);
// }

// #[cfg(feature = "text-field")]
// #[wasm_bindgen(module = "@material/floating-label/index")]
// extern "C" {
//     /// Floating labels display the type of input a field requires.
//     /// Every Text Field and Select should have a label, except for full-width
//     /// text fields, which use the input’s `placeholder` attribute instead.
//     /// Labels are aligned with the input line and always visible.
//     /// They can be resting (when a field is inactive and empty) or floating.
//     /// The label is a text caption or description for the Text Field.
//     pub type MDCFloatingLabel;

//     #[wasm_bindgen(constructor)]
//     pub fn new(surface: Element) -> MDCFloatingLabel;

//     #[wasm_bindgen(method)]
//     pub fn shake(this: &MDCFloatingLabel, should_shake: bool);

//     #[wasm_bindgen(method)]
//     pub fn float(this: &MDCFloatingLabel, should_float: bool);

//     #[wasm_bindgen(method)]
//     pub fn get_width(this: &MDCFloatingLabel) -> js_sys::Number;

//     #[wasm_bindgen(method)]
//     pub fn destroy(this: &MDCFloatingLabel);
// }
