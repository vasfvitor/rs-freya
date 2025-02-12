#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render!(App {
        title: "My App",
        navbar: render!(Navbar {
            title: "Top Navbar"
        }),
        body: render!(
            ScrollView {
                theme: theme_with!(ScrollViewTheme {
                    height: "calc(100% - 75 - 75)".into(),
                }),
                show_scrollbar: true,
                Card {
                    title: "Card 0",
                    content: "Content 0",
                }
                ScrollView {
                    theme: theme_with!(ScrollViewTheme {
                        height: "200".into(),
                        padding: "0 20".into(),
                    }),
                    Card {
                        title: "Card 1",
                        content: "Content 1",
                    }
                    Card {
                        title: "Card 2",
                        content: "Content 2",
                    }
                    Card {
                        title: "Card 3",
                        content: "Content 3",
                    }
                    Card {
                        title: "Card 4",
                        content: "Content 4",
                    }
                }
                Card {
                    title: "Card 5",
                    content: "Content 5",
                }
                Card {
                    title: "Card 6",
                    content: "Content 6",
                }
                Card {
                    title: "Card 7",
                    content: "Content 7",
                }
                Card {
                    title: "Card 7",
                    content: "Content 7",
                }
            }
            Navbar {
                title: "Bottom Bar"
            }
        )
    })
}

#[derive(Props)]
struct NavbarProps<'a> {
    title: &'a str,
}

#[allow(non_snake_case)]
fn Navbar<'a>(cx: Scope<'a, NavbarProps<'a>>) -> Element {
    render!(
        rect {
            overflow: "clip",
            height: "75",
            width: "100%",
            background: "rgb(20, 20, 20)",
            padding: "15",
            main_align: "center",
            cross_align: "center",
            label {
                "{&cx.props.title}"
            }
        }
    )
}

#[allow(dead_code)]
#[derive(Props)]
struct AppProps<'a> {
    title: &'a str,
    body: Element<'a>,
    navbar: Element<'a>,
}

#[allow(non_snake_case)]
fn App<'a>(cx: Scope<'a, AppProps<'a>>) -> Element {
    render!(
        rect {
            color: "white",
            width: "100%",
            height: "100%",
            rect {
                width: "100%",
                height: "100%",
                &cx.props.navbar,
                &cx.props.body
            }
        }
    )
}

#[derive(Props)]
struct CardProps<'a> {
    title: &'a str,
    content: &'a str,
}

#[allow(non_snake_case)]
fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element {
    render!(
        rect {
            width: "100%",
            height: "200",
            padding: "10",
            background: "rgb(35, 35, 35)",
            rect {
                width: "100%",
                height: "50%",
                padding: "5",
                corner_radius: "10",
                main_align: "center",
                label {
                    height: "auto",
                    "{&cx.props.title}"
                }
                label {
                    height: "auto",
                    "{&cx.props.content}"
                }
            }
            Area {

            }
        }
    )
}

#[allow(non_snake_case)]
fn Area(cx: Scope) -> Element {
    let cursor_pos_over = use_state(cx, || (0f64, 0f64));
    let cursor_pos_click = use_state(cx, || (0f64, 0f64));

    let cursor_moved = |e: MouseEvent| {
        cursor_pos_over.with_mut(|cursor_pos| {
            let pos = e.get_screen_coordinates();
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        });
    };

    let cursor_clicked = |e: MouseEvent| {
        cursor_pos_click.with_mut(|cursor_pos| {
            let pos = e.get_screen_coordinates();
            cursor_pos.0 = pos.x;
            cursor_pos.1 = pos.y;
        })
    };

    render!(
        rect {
            height: "50%",
            width: "100%",
            background: "rgb(228, 143, 69)",
            padding: "10",
            corner_radius: "12",
            main_align: "center",
            cross_align: "center",
            onmouseover: cursor_moved,
            onclick: cursor_clicked,
            label {
                "Mouse is at [x: {cursor_pos_over.0}, y: {cursor_pos_over.1}] ",
            },
            label {
                "Mouse clicked at [x: {cursor_pos_click.0}, y: {cursor_pos_click.1}]"
            }
        }
    )
}
