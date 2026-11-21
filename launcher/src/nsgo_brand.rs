//! NSGO branding, logos, and buttons for Nshorizon.
use super::*;
use gpui::{div, px, rgb, Div, FontWeight, Stateful};

/// The NSGO flame / brand mark at `size` pixels square.
pub(super) fn logo(size: f32) -> Div {
    let r = size * 0.28;
    div()
        .relative()
        .flex_none()
        .size(px(size))
        .rounded(px(r))
        .bg(rgb(0x0d0d12))
        .border_1()
        .border_color(rgb(0x27272a))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .relative()
                .size(px(size * 0.65))
                .rounded(px(r * 0.7))
                .bg(gpui::linear_gradient(
                    180.0,
                    gpui::linear_color_stop(rgb(0xfb923c), 0.0),
                    gpui::linear_color_stop(rgb(0xea580c), 1.0),
                ))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .size(px(size * 0.32))
                        .rounded(px(r * 0.35))
                        .bg(rgb(0xfdba74)),
                ),
        )
}

/// Stylized "NSGO" primary sign-in button.
pub(super) fn sign_in_button(id: &'static str) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .justify_center()
        .gap(px(10.))
        .h(px(46.))
        .px(px(20.))
        .rounded(px(10.))
        .bg(gpui::linear_gradient(
            135.0,
            gpui::linear_color_stop(rgb(0xea580c), 0.0),
            gpui::linear_color_stop(rgb(0xc2410c), 1.0),
        ))
        .border_1()
        .border_color(rgb(0xfb923c))
        .cursor_pointer()
        .hover(|s| s.opacity(0.92))
        .font_family(horizon_theme::glass::FONT)
        .text_size(px(15.))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(gpui::white())
        .child(logo(18.))
        .child(tr("NSGO"))
}
