use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, ArrowIconTheme, ArrowIconThemeWith};

#[derive(Props, PartialEq)]
pub struct ArrowIcon {
    /// Theme override.
    #[props(optional)]
    pub theme: Option<ArrowIconThemeWith>,
    #[props(into)]
    pub rotate: String,
    #[props(into)]
    pub fill: String,
}

#[allow(non_snake_case)]
pub fn ArrowIcon(cx: Scope<ArrowIcon>) -> Element {
    let ArrowIcon {
        rotate,
        fill,
        theme,
    } = &cx.props;
    let ArrowIconTheme {
        height,
        width,
        margin,
    } = use_applied_theme!(cx, theme, arrow_icon);

    render!(svg {
        height: "{height}",
        width: "{width}",
        margin: "{margin}",
        rotate: "{rotate}deg",
        svg_content: r#"
            <svg viewBox="0 0 18 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" clip-rule="evenodd" d="M7.18177 9.58579L0 2.40401L1.81823 0.585785L9 7.76756L16.1818 0.585787L18 2.40402L10.8182 9.58579L10.8185 9.58601L9.00023 11.4042L9 11.404L8.99977 11.4042L7.18154 9.58602L7.18177 9.58579Z" fill="{fill}" stroke="{fill}" stroke-width="2"/>
            </svg>
        "#
    })
}
