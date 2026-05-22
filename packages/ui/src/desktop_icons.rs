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
        // Lucide 'User'
        svg_path: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2 M12 3a4 4 0 1 0 0 8 4 4 0 1 0 0-8z",
    },
    IconDef {
        label: "Projects",
        command: "projects",
        initial_pos: (20, 120),
        // Lucide 'Folder'
        svg_path: "M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z",
    },

    IconDef {
        label: "Joke",
        command: "joke",
        initial_pos: (20, 220),
        // Lucide 'Smile'
        svg_path: "M12 22a10 10 0 1 0 0-20 10 10 0 0 0 0 20z M8 14s1.5 2 4 2 4-2 4-2 M9 9h.01 M15 9h.01",
    },
    IconDef {
        label: "Recycle Bin",
        command: "clear",
        initial_pos: (20, 320),
        // Lucide 'Trash2'
        svg_path: "M3 6h18 M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6 M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2 M10 11v6 M14 11v6",
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
            style: "transform: translate3d({pos.0}px, {pos.1}px, 0); position: absolute; z-index: {z_index};",
            class: "flex flex-col items-center w-16 p-1.5 cursor-pointer select-none group rounded-md hover:bg-white/10 transition-colors duration-200 {drag_class} pointer-events-auto",
            onpointerdown: handle_down,
            ondoubleclick: move |_| on_double_click.call(command.clone()),
            div {
                class: "flex flex-col items-center w-full icon-entrance {bounce_class}",
                style: "{entrance_delay}",
                div {
                    class: "w-10 h-10 flex items-center justify-center bg-blue-500/20 rounded-lg border border-blue-400/30 group-hover:bg-blue-500/30 group-hover:border-blue-400/50 backdrop-blur-sm transition-smooth shadow-lg pointer-events-none",
                    style: "transition: transform 200ms cubic-bezier(0.34, 1.56, 0.64, 1), background-color 200ms ease, border-color 200ms ease;",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        class: "w-6 h-6 text-white drop-shadow-[0_0_8px_rgba(255,255,255,0.5)]",
                        path { d: "{svg_path}" }
                    }
                }
                span {
                    class: "mt-1 text-[10px] font-medium text-white text-center drop-shadow-[0_1px_2px_rgba(0,0,0,0.8)] px-1 py-0.5 rounded leading-tight pointer-events-none",
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
        for icon in ICONS {
            map.insert(icon.command.to_string(), icon.initial_pos);
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

    rsx! {
        div {
            class: "absolute inset-0 z-0",
            onpointermove: handle_pointer_move,
            onpointerup: handle_pointer_up,
            onpointerleave: handle_pointer_up,
            div {
                class: "relative w-full h-full {container_pointer}",
                for (idx, icon) in ICONS.iter().enumerate() {
                    DesktopIcon {
                        key: "{icon.command}",
                        label: icon.label.to_string(),
                        command: icon.command.to_string(),
                        pos: *positions.read().get(icon.command).unwrap_or(&icon.initial_pos),
                        index: idx,
                        is_dragging: dragging.read().as_ref().map(|(c, _)| c == icon.command).unwrap_or(false),
                        is_bouncing: bouncing_icon.read().as_ref().map(|c| c == icon.command).unwrap_or(false),
                        svg_path: icon.svg_path.to_string(),
                        on_pointer_down: move |offset| {
                            dragging.set(Some((icon.command.to_string(), offset)));
                        },
                        on_double_click: move |cmd: String| {
                            bouncing_icon.set(Some(cmd.clone()));
                            // Clear bounce after animation completes
                            spawn(async move {
                                gloo_timers::future::sleep(std::time::Duration::from_millis(350)).await;
                                bouncing_icon.set(None);
                            });
                            props.on_icon_click.call(cmd);
                        }
                    }
                }
            }
        }
    }
}
