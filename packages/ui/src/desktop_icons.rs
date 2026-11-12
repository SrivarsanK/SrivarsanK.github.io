/*
Microsoft Fluent System Icons
https://github.com/microsoft/fluentui-system-icons

MIT License

Copyright (c) 2020 Microsoft Corporation

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Props, Clone, PartialEq)]
pub struct DesktopIconsProps {
    on_icon_click: EventHandler<String>,
}

#[derive(Clone, PartialEq)]
struct IconDef {
    label: &'static str,
    command: &'static str,
    initial_pos: (i32, i32),
    svg_path: &'static str,
}

const ICONS: &[IconDef] = &[
    IconDef {
        label: "About Me",
        command: "about",
        initial_pos: (20, 20),
        svg_path: "M17.7545 14.0002C18.9966 14.0002 20.0034 15.007 20.0034 16.2491V16.8245C20.0034 17.7188 19.6838 18.5836 19.1023 19.263C17.5329 21.0965 15.1457 22.0013 12.0004 22.0013C8.8545 22.0013 6.46849 21.0962 4.90219 19.2619C4.32242 18.583 4.00391 17.7195 4.00391 16.8267V16.2491C4.00391 15.007 5.01076 14.0002 6.25278 14.0002H17.7545ZM17.7545 15.5002H6.25278C5.83919 15.5002 5.50391 15.8355 5.50391 16.2491V16.8267C5.50391 17.3624 5.69502 17.8805 6.04287 18.2878C7.29618 19.7555 9.26206 20.5013 12.0004 20.5013C14.7387 20.5013 16.7063 19.7555 17.9627 18.2876C18.3117 17.8799 18.5034 17.361 18.5034 16.8245V16.2491C18.5034 15.8355 18.1681 15.5002 17.7545 15.5002ZM12.0004 2.00488C14.7618 2.00488 17.0004 4.24346 17.0004 7.00488C17.0004 9.76631 14.7618 12.0049 12.0004 12.0049C9.23894 12.0049 7.00036 9.76631 7.00036 7.00488C7.00036 4.24346 9.23894 2.00488 12.0004 2.00488ZM12.0004 3.50488C10.0674 3.50488 8.50036 5.07189 8.50036 7.00488C8.50036 8.93788 10.0674 10.5049 12.0004 10.5049C13.9334 10.5049 15.5004 8.93788 15.5004 7.00488C15.5004 5.07189 13.9334 3.50488 12.0004 3.50488Z",
    },
    IconDef {
        label: "Projects",
        command: "projects",
        initial_pos: (20, 120),
        svg_path: "M3.5 6.25V8H8.12868C8.32759 8 8.51836 7.92098 8.65901 7.78033L10.1893 6.25L8.65901 4.71967C8.51836 4.57902 8.32759 4.5 8.12868 4.5H5.25C4.2835 4.5 3.5 5.2835 3.5 6.25ZM2 6.25C2 4.45507 3.45507 3 5.25 3H8.12868C8.72542 3 9.29771 3.23705 9.71967 3.65901L11.5607 5.5H18.75C20.5449 5.5 22 6.95507 22 8.75V17.75C22 19.5449 20.5449 21 18.75 21H5.25C3.45507 21 2 19.5449 2 17.75V6.25ZM3.5 9.5V17.75C3.5 18.7165 4.2835 19.5 5.25 19.5H18.75C19.7165 19.5 20.5 18.7165 20.5 17.75V8.75C20.5 7.7835 19.7165 7 18.75 7H11.5607L9.71967 8.84099C9.29771 9.26295 8.72542 9.5 8.12868 9.5H3.5Z",
    },

    IconDef {
        label: "Skills",
        command: "skills",
        initial_pos: (100, 20),
        svg_path: "M15.0461 1.49224C15.1885 1.10338 15.6191 0.903659 16.008 1.04595C16.3967 1.18841 16.5965 1.619 16.4543 2.00786L8.95427 22.5079C8.81191 22.8967 8.38124 23.0963 7.99236 22.9542C7.60349 22.8118 7.40378 22.3812 7.54607 21.9922L15.0461 1.49224ZM6.2072 6.23247C6.4929 5.93284 6.96791 5.92152 7.26775 6.20708C7.5673 6.49276 7.57861 6.9678 7.29314 7.26763L2.78533 12.0001L7.29314 16.7325C7.57857 17.0323 7.56733 17.5074 7.26775 17.793C6.96793 18.0786 6.4929 18.0672 6.2072 17.7676L1.2072 12.5176C0.93134 12.228 0.93134 11.7721 1.2072 11.4825L6.2072 6.23247ZM16.7326 6.20708C17.0325 5.92158 17.5075 5.93279 17.7931 6.23247L22.7931 11.4825C23.0688 11.7721 23.0689 12.228 22.7931 12.5176L17.7931 17.7676C17.5075 18.0673 17.0325 18.0785 16.7326 17.793C16.4329 17.5073 16.4216 17.0323 16.7072 16.7325L21.215 12.0001L16.7072 7.26763C16.4216 6.96776 16.4329 6.49277 16.7326 6.20708Z",
    },
    IconDef {
        label: "Joke",
        command: "joke",
        initial_pos: (20, 220),
        svg_path: "M6.74927 12C6.53852 12 6.33749 12.0887 6.19539 12.2443C6.05329 12.4 5.98323 12.6082 6.00237 12.8181C6.28259 15.8916 8.55224 18.5 12 18.5C15.4478 18.5 17.7174 15.8916 17.9977 12.8181C18.0168 12.6082 17.9468 12.4 17.8047 12.2443C17.6626 12.0887 17.4616 12 17.2508 12H6.74927ZM12 17C9.74286 17 8.12852 15.5205 7.63237 13.5H16.3677C15.8715 15.5205 14.2571 17 12 17ZM15.2501 8.75C14.8416 8.75 14.5398 9.03719 14.492 9.35982C14.4314 9.76957 14.05 10.0526 13.6403 9.99192C13.2305 9.93126 12.9475 9.54993 13.0082 9.14018C13.1696 8.0495 14.1313 7.25 15.2501 7.25C16.3689 7.25 17.3306 8.0495 17.492 9.14018C17.5527 9.54993 17.2697 9.93126 16.8599 9.99192C16.4502 10.0526 16.0688 9.76957 16.0082 9.35982C15.9604 9.03719 15.6586 8.75 15.2501 8.75ZM7.99202 9.35982C8.03977 9.03719 8.34157 8.75 8.7501 8.75C9.15863 8.75 9.46043 9.03719 9.50818 9.35982C9.56884 9.76957 9.95017 10.0526 10.3599 9.99192C10.7697 9.93126 11.0527 9.54993 10.992 9.14018C10.8306 8.0495 9.86892 7.25 8.7501 7.25C7.63128 7.25 6.66963 8.0495 6.50818 9.14018C6.44753 9.54993 6.73053 9.93126 7.14028 9.99192C7.55003 10.0526 7.93136 9.76957 7.99202 9.35982ZM12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2ZM3.5 12C3.5 7.30558 7.30558 3.5 12 3.5C16.6944 3.5 20.5 7.30558 20.5 12C20.5 16.6944 16.6944 20.5 12 20.5C7.30558 20.5 3.5 16.6944 3.5 12Z",
    },
    IconDef {
        label: "Recycle Bin",
        command: "clear",
        initial_pos: (20, 320),
        svg_path: "M10 5H14C14 3.89543 13.1046 3 12 3C10.8954 3 10 3.89543 10 5ZM8.5 5C8.5 3.067 10.067 1.5 12 1.5C13.933 1.5 15.5 3.067 15.5 5H21.25C21.6642 5 22 5.33579 22 5.75C22 6.16421 21.6642 6.5 21.25 6.5H19.9309L18.7589 18.6112C18.5729 20.5334 16.9575 22 15.0263 22H8.97369C7.04254 22 5.42715 20.5334 5.24113 18.6112L4.06908 6.5H2.75C2.33579 6.5 2 6.16421 2 5.75C2 5.33579 2.33579 5 2.75 5H8.5ZM10.5 9.75C10.5 9.33579 10.1642 9 9.75 9C9.33579 9 9 9.33579 9 9.75V17.25C9 17.6642 9.33579 18 9.75 18C10.1642 18 10.5 17.6642 10.5 17.25V9.75ZM14.25 9C14.6642 9 15 9.33579 15 9.75V17.25C15 17.6642 14.6642 18 14.25 18C13.8358 18 13.5 17.6642 13.5 17.25V9.75C13.5 9.33579 13.8358 9 14.25 9ZM6.73416 18.4667C6.84577 19.62 7.815 20.5 8.97369 20.5H15.0263C16.185 20.5 17.1542 19.62 17.2658 18.4667L18.4239 6.5H5.57608L6.73416 18.4667Z",
    },
    IconDef {
        label: "Contact Me",
        command: "contact",
        initial_pos: (20, 420),
        svg_path: "M5.25 4H18.75C20.483 4 21.8992 5.35645 21.9949 7.06558L22 7.25V16.75C22 18.483 20.6435 19.8992 18.9344 19.9949L18.75 20H5.25C3.51697 20 2.10075 18.6435 2.00514 16.9344L2 16.75V7.25C2 5.51697 3.35645 4.10075 5.06558 4.00514L5.25 4H18.75H5.25ZM20.5 9.373L12.3493 13.6637C12.1619 13.7623 11.9431 13.7764 11.7468 13.706L11.6507 13.6637L3.5 9.374V16.75C3.5 17.6682 4.20711 18.4212 5.10647 18.4942L5.25 18.5H18.75C19.6682 18.5 20.4212 17.7929 20.4942 16.8935L20.5 16.75V9.373ZM18.75 5.5H5.25C4.33183 5.5 3.57881 6.20711 3.5058 7.10647L3.5 7.25V7.679L12 12.1525L20.5 7.678V7.25C20.5 6.33183 19.7929 5.57881 18.8935 5.5058L18.75 5.5Z",
    },
];

#[component]
fn DesktopIcon(
    label: String,
    command: String,
    pos: (i32, i32),
    index: usize,
    is_dragging: bool,
    is_bouncing: bool,
    svg_path: String,
    on_pointer_down: EventHandler<(f64, f64)>,
    on_double_click: EventHandler<String>,
) -> Element {
    let z_index = if is_dragging { 20 } else { 10 };
    let drag_class = if is_dragging { "opacity-70" } else { "" };
    
    // Staggered entrance animation
    let entrance_delay = format!("animation-delay: {}ms;", index * 100 + 200);
    
    // Bounce class on double-click
    let bounce_class = if is_bouncing { "icon-bounce" } else { "" };
    
    let handle_down = move |e: Event<PointerData>| {
        let offset = (
            e.client_coordinates().x - pos.0 as f64,
            e.client_coordinates().y - pos.1 as f64,
        );
        on_pointer_down.call(offset);
    };

    rsx! {
        div {
            style: "transform: translate3d({pos.0}px, {pos.1}px, 0); position: absolute; z-index: {z_index}; width: 80px; padding: 6px 2px; box-sizing: border-box;",
            class: "flex flex-col items-center w-16 p-1.5 cursor-pointer select-none group rounded-md hover:bg-white/10 transition-colors duration-200 {drag_class} pointer-events-auto",
            onpointerdown: handle_down,
            ondoubleclick: move |_| on_double_click.call(command.clone()),
            div {
                class: "flex flex-col items-center w-full icon-entrance {bounce_class}",
                style: "{entrance_delay}",
                div {
                    style: "width: 48px; height: 48px; display: flex; align-items: center; justify-content: center; flex-shrink: 0; pointer-events: none;",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        width: "32", height: "32",
                        fill: "currentColor",
                        "aria-hidden": "true",
                        style: "display: block; width: 32px; height: 32px; color: #f5f7fa; filter: drop-shadow(0 1px 2px #0009);",
                        path { d: "{svg_path}" }
                    }
                }
                span {
                    class: "mt-1 text-xs font-medium text-white text-center drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)] px-1 py-0.5 rounded leading-tight pointer-events-none",
                    style: "display: block; width: 80px; margin-top: 5px; padding: 0; color: white; font-family: Segoe UI, sans-serif; font-size: 12px; font-weight: 400; line-height: 16px; text-align: center; white-space: nowrap; text-shadow: 0 1px 3px #000, 0 0 4px #0008;",
                    "{label}"
                }
            }
        }
    }
}

#[component]
pub fn DesktopIcons(props: DesktopIconsProps) -> Element {
    let mut positions = use_signal(|| {
        let mut map = HashMap::new();
        let w = web_sys::window()
            .and_then(|win| win.inner_width().ok())
            .and_then(|w| w.as_f64())
            .unwrap_or(1280.0);
        let h = web_sys::window()
            .and_then(|win| win.inner_height().ok())
            .and_then(|h| h.as_f64())
            .unwrap_or(720.0);
            
        let is_mobile = w < 600.0;
        let mut current_x = 20;
        let mut current_y = 20;
        let icon_spacing_y = 100;
        let icon_spacing_x = 96;

        for icon in ICONS {
            if is_mobile {
                if current_x + icon_spacing_x > (w as i32 - 20) && current_x > 20 {
                    current_y += icon_spacing_y;
                    current_x = 20;
                }
                map.insert(icon.command.to_string(), (current_x, current_y));
                current_x += icon_spacing_x;
            } else {
                if current_y + icon_spacing_y > (h as i32 - 40) && current_y > 20 {
                    current_x += icon_spacing_x;
                    current_y = 20;
                }
                map.insert(icon.command.to_string(), (current_x, current_y));
                current_y += icon_spacing_y;
            }
        }
        map
    });

    let mut dragging = use_signal(|| None::<(String, (f64, f64))>);
    let mut bouncing_icon = use_signal(|| None::<String>);

    let handle_pointer_move = move |e: Event<PointerData>| {
        if let Some((cmd, offset)) = dragging.read().as_ref() {
            let mut pos_map = positions.write();
            if let Some(pos) = pos_map.get_mut(cmd) {
                pos.0 = (e.client_coordinates().x - offset.0) as i32;
                pos.1 = (e.client_coordinates().y - offset.1) as i32;
            }
        }
    };

    let handle_pointer_up = move |_| {
        dragging.set(None);
    };

    let is_dragging = dragging.read().is_some();
    let container_pointer = if is_dragging { "pointer-events-auto" } else { "pointer-events-none" };
    let current_positions = positions.read().clone();
    let current_dragging = dragging.read().clone();
    let current_bouncing = bouncing_icon.read().clone();

    rsx! {
        div {
            class: "absolute inset-0 z-0 hidden-mobile",
            onpointermove: handle_pointer_move,
            onpointerup: handle_pointer_up,
            div {
                class: "relative w-full h-full {container_pointer}",
                for (idx, icon) in ICONS.iter().enumerate() {
                    DesktopIcon {
                        key: "{icon.command}",
                        label: icon.label.to_string(),
                        command: icon.command.to_string(),
                        pos: *current_positions.get(icon.command).unwrap_or(&icon.initial_pos),
                        index: idx,
                        is_dragging: current_dragging.as_ref().map(|(c, _)| c == icon.command).unwrap_or(false),
                        is_bouncing: current_bouncing.as_ref().map(|c| c == icon.command).unwrap_or(false),
                        svg_path: icon.svg_path.to_string(),
                        on_pointer_down: move |offset| {
                            dragging.set(Some((icon.command.to_string(), offset)));
                        },
                        on_double_click: move |cmd: String| {
                            bouncing_icon.set(Some(cmd.clone()));
                            // Call icon click immediately
                            props.on_icon_click.call(cmd);
                            // Clear bounce after animation completes in a detached task
                            let mut bounce_sig = bouncing_icon;
                            spawn(async move {
                                gloo_timers::future::sleep(std::time::Duration::from_millis(350)).await;
                                bounce_sig.set(None);
                            });
                        }
                    }
                }
            }
        }
    }
}
