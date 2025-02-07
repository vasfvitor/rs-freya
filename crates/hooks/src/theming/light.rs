use crate::cow_borrowed;
use crate::theming::*;

pub const LIGHT_THEME: Theme = Theme {
    name: "light",
    body: BodyTheme {
        background: cow_borrowed!("white"),
        color: cow_borrowed!("black"),
        padding: cow_borrowed!("none"),
    },
    slider: SliderTheme {
        background: cow_borrowed!("rgb(210, 210, 210)"),
        thumb_background: cow_borrowed!("rgb(210, 210, 210)"),
        thumb_inner_background: cow_borrowed!("rgb(103, 80, 164)"),
    },
    button: ButtonTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
        padding: cow_borrowed!("8 16"),
        margin: cow_borrowed!("4"),
        corner_radius: cow_borrowed!("8"),
        width: cow_borrowed!("auto"),
        height: cow_borrowed!("auto"),
    },
    input: InputTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
        width: cow_borrowed!("150"),
        margin: cow_borrowed!("4"),
    },
    switch: SwitchTheme {
        background: cow_borrowed!("rgb(121, 116, 126)"),
        thumb_background: cow_borrowed!("rgb(231, 224, 236)"),
        enabled_background: cow_borrowed!("rgb(103, 80, 164)"),
        enabled_thumb_background: cow_borrowed!("rgb(234, 221, 255)"),
    },
    scroll_bar: ScrollBarTheme {
        background: cow_borrowed!("rgb(225, 225, 225)"),
        thumb_background: cow_borrowed!("rgb(135, 135, 135)"),
        hover_thumb_background: cow_borrowed!("rgb(115, 115, 115)"),
        active_thumb_background: cow_borrowed!("rgb(95, 95, 95)"),
        offset_x: cow_borrowed!("0"),
        offset_y: cow_borrowed!("0"),
    },
    scroll_view: ScrollViewTheme {
        height: cow_borrowed!("100%"),
        width: cow_borrowed!("100%"),
        padding: cow_borrowed!("0"),
    },
    tooltip: TooltipTheme {
        background: cow_borrowed!("rgb(245, 245, 245)"),
        color: cow_borrowed!("rgb(25,25,25)"),
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
    },
    external_link: ExternalLinkTheme {
        highlight_color: cow_borrowed!("rgb(43,106,208)"),
    },
    dropdown: DropdownTheme {
        dropdown_background: cow_borrowed!("white"),
        background_button: cow_borrowed!("rgb(245, 245, 245)"),
        hover_background: cow_borrowed!("rgb(235, 235, 235)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
        arrow_fill: cow_borrowed!("rgb(40, 40, 40)"),
    },
    dropdown_item: DropdownItemTheme {
        background: cow_borrowed!("white"),
        select_background: cow_borrowed!("rgb(240, 240, 240)"),
        hover_background: cow_borrowed!("rgb(220, 220, 220)"),
        font_theme: FontTheme {
            color: cow_borrowed!("rgb(10, 10, 10)"),
        },
    },
    accordion: AccordionTheme {
        color: cow_borrowed!("black"),
        background: cow_borrowed!("rgb(245, 245, 245)"),
        border_fill: cow_borrowed!("rgb(210, 210, 210)"),
    },
    loader: LoaderTheme {
        primary_color: cow_borrowed!("rgb(50, 50, 50)"),
        secondary_color: cow_borrowed!("rgb(150, 150, 150)"),
    },
    progress_bar: ProgressBarTheme {
        color: cow_borrowed!("white"),
        background: cow_borrowed!("rgb(210, 210, 210)"),
        progress_background: cow_borrowed!("rgb(103, 80, 164)"),
        width: cow_borrowed!("100%"),
        height: cow_borrowed!("20"),
    },
    table: TableTheme {
        font_theme: FontTheme {
            color: cow_borrowed!("black"),
        },
        background: cow_borrowed!("white"),
        arrow_fill: cow_borrowed!("rgb(40, 40, 40)"),
        row_background: cow_borrowed!("transparent"),
        alternate_row_background: cow_borrowed!("rgb(240, 240, 240)"),
        divider_fill: cow_borrowed!("rgb(200, 200, 200)"),
        height: cow_borrowed!("auto"),
        corner_radius: cow_borrowed!("6"),
        shadow: cow_borrowed!("0 2 15 5 rgb(35, 35, 35, 70)"),
    },
    canvas: CanvasTheme {
        width: cow_borrowed!("300"),
        height: cow_borrowed!("150"),
        background: cow_borrowed!("white"),
    },
    graph: GraphTheme {
        width: cow_borrowed!("100%"),
        height: cow_borrowed!("100%"),
    },
    network_image: NetworkImageTheme {
        width: cow_borrowed!("100%"),
        height: cow_borrowed!("100%"),
    },
    arrow_icon: ArrowIconTheme {
        width: cow_borrowed!("10"),
        height: cow_borrowed!("10"),
        margin: cow_borrowed!("none"),
    },
};
