use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Clone)]
enum LineType {
    Input,
    Output,
    Error,
    Info,
    Success,
    Component(Element),
}

#[derive(Clone)]
struct TerminalLine {
    id: usize,
    line_type: LineType,
    content: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct TerminalProps {
    pub external_command: Signal<Option<String>>,
}

#[derive(Deserialize)]
struct WaifuResponse {
    url: String,
}

#[derive(Deserialize)]
struct JokeResponse {
    #[serde(rename = "type")]
    joke_type: String,
    joke: Option<String>,
    setup: Option<String>,
    delivery: Option<String>,
}

const WELCOME_MESSAGE: &str = r#"Hello, World! I'm Ovi ren
I'm a writer, i write scripts.

Type 'help' to see available commands."#;

#[component]
pub fn Terminal(props: TerminalProps) -> Element {
    let mut external_cmd = props.external_command;
    let mut lines = use_signal(|| vec![
        TerminalLine {
            id: 0,
            line_type: LineType::Output,
            content: WELCOME_MESSAGE.to_string(),
        }
    ]);
    let mut next_id = use_signal(|| 1);
    
    let mut current_input = use_signal(|| String::new());
    let mut command_history = use_signal(|| Vec::<String>::new());
    let mut history_index = use_signal(|| -1i32);
    
    let mut is_minimized = use_signal(|| false);
    let mut is_maximized = use_signal(|| false);
    
    let mut pos = use_signal(|| (100.0, 100.0));
    let mut size = use_signal(|| (800.0, 500.0));
    
    let mut dragging_header = use_signal(|| None::<(f64, f64)>);
    let mut resizing = use_signal(|| None::<(f64, f64, f64, f64)>); // start_x, start_y, start_w, start_h

    // Execute command function
    let mut execute_command = move |cmd_str: String| {
        let trimmed = cmd_str.trim().to_lowercase();
        
        let id = next_id();
        next_id += 1;
        
        lines.write().push(TerminalLine {
            id,
            line_type: LineType::Input,
            content: cmd_str.clone(),
        });

        if trimmed.is_empty() {
            return;
        }

        command_history.write().push(cmd_str.clone());
        history_index.set(-1);

        if trimmed == "cls" || trimmed == "clear" {
            lines.write().clear();
            return;
        }

        let out_id = next_id();
        next_id += 1;

        match trimmed.as_str() {
            "help" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "Available commands:\n  help      - Show this help message\n  about     - Learn about me\n  skills    - View my technical skills\n  projects  - Browse my projects\n  contact   - Get my contact information\n  whoami    - Display current user\n  date      - Show current date and time\n  clear     - Clear the terminal\n  waifu     - Show a random waifu image\n  joke      - Tell a random joke".to_string(),
                });
            }
            "about" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Component(rsx! {
                        div {
                            class: "my-4 space-y-4",
                            div {
                                class: "border border-border rounded-lg overflow-hidden w-64 bg-secondary/50 shadow-lg",
                                img {
                                    src: "https://iamovi.github.io/assets/sword.jpg",
                                    alt: "Profile",
                                    class: "w-full h-auto object-cover"
                                }
                            }
                            div {
                                class: "font-mono leading-relaxed bg-secondary/30 p-4 border border-border rounded-lg",
                                div { class: "text-terminal-cyan font-bold mb-2", "┌ ABOUT ME ┐" }
                                p { class: "mb-4 text-foreground/90", "Hi! I'm a hobby programmer. I love building things and exploring new technologies in my free time." }
                                p { class: "text-foreground/90", "When i was in high school i started programming as my hobby, that's how it all started." }
                            }
                        }
                    }),
                    content: "".to_string(),
                });
            }
            "skills" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "┌─────────────────────────────────────────┐\n│  TECHNICAL SKILLS                       │\n├─────────────────────────────────────────┤\n│  • Rust, TypeScript, Python             │\n│  • Dioxus, React, Next.js               │\n│  • TailwindCSS, Git, Docker             │\n└─────────────────────────────────────────┘".to_string(),
                });
            }
            "projects" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "┌─────────────────────────────────────────┐\n│  FEATURED PROJECTS                      │\n├─────────────────────────────────────────┤\n│  01. syswaifu                           │\n│  02. button-will-react                  │\n│  Visit https://iamovi.github.io         │\n└─────────────────────────────────────────┘".to_string(),
                });
            }
            "contact" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "┌─────────────────────────────────────────┐\n│  CONTACT                                │\n├─────────────────────────────────────────┤\n│  📧 Email      fornet.ovi@email.com     │\n│  🐙 GitHub     github.com/iamovi        │\n│  🌐 Website    iamovi.github.io         │\n└─────────────────────────────────────────┘".to_string(),
                });
            }
            "whoami" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: "ren@portfolio".to_string(),
                });
            }
            "date" => {
                let now = chrono::Local::now();
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Output,
                    content: now.format("%Y-%m-%d %H:%M:%S").to_string(),
                });
            }
            "waifu" => {
                // Initial loading message
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Info,
                    content: "Fetching waifu...".to_string(),
                });

                spawn(async move {
                    if let Ok(resp) = reqwest::get("https://api.waifu.pics/sfw/waifu").await {
                        if let Ok(data) = resp.json::<WaifuResponse>().await {
                            let mut l = lines.write();
                            if let Some(pos) = l.iter().position(|x| x.id == out_id) {
                                l[pos] = TerminalLine {
                                    id: out_id,
                                    line_type: LineType::Component(rsx! {
                                        img {
                                            src: "{data.url}",
                                            class: "my-4 rounded-lg shadow-lg border border-border max-w-sm"
                                        }
                                    }),
                                    content: "".to_string(),
                                };
                            }
                            return;
                        }
                    }
                    let mut l = lines.write();
                    if let Some(pos) = l.iter().position(|x| x.id == out_id) {
                        l[pos] = TerminalLine {
                            id: out_id,
                            line_type: LineType::Error,
                            content: "Failed to fetch waifu.".to_string(),
                        };
                    }
                });
            }
            "joke" => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Info,
                    content: "Fetching a joke for you...".to_string(),
                });

                spawn(async move {
                    if let Ok(resp) = reqwest::get("https://v2.jokeapi.dev/joke/Any").await {
                        if let Ok(data) = resp.json::<JokeResponse>().await {
                            let mut l = lines.write();
                            if let Some(pos) = l.iter().position(|x| x.id == out_id) {
                                let content_elem = if data.joke_type == "single" {
                                    rsx! { p { class: "text-foreground/90", "{data.joke.clone().unwrap_or_default()}" } }
                                } else {
                                    rsx! {
                                        div { class: "space-y-4",
                                            p { class: "text-foreground/90 italic", "\"{data.setup.clone().unwrap_or_default()}\"" }
                                            p { class: "text-terminal-cyan font-bold", "{data.delivery.clone().unwrap_or_default()}" }
                                        }
                                    }
                                };
                                l[pos] = TerminalLine {
                                    id: out_id,
                                    line_type: LineType::Component(rsx! {
                                        div { class: "my-4 p-4 border border-border rounded-lg bg-secondary/30 max-w-lg font-mono leading-relaxed",
                                            div { class: "text-terminal-yellow font-bold mb-2", "┌ JOKE ┐" }
                                            {content_elem}
                                        }
                                    }),
                                    content: "".to_string(),
                                };
                            }
                            return;
                        }
                    }
                    let mut l = lines.write();
                    if let Some(pos) = l.iter().position(|x| x.id == out_id) {
                        l[pos] = TerminalLine {
                            id: out_id,
                            line_type: LineType::Error,
                            content: "Failed to fetch joke.".to_string(),
                        };
                    }
                });
            }
            _ => {
                lines.write().push(TerminalLine {
                    id: out_id,
                    line_type: LineType::Error,
                    content: format!("Command not found: {}. Type 'help' for available commands.", trimmed),
                });
            }
        }
    };

    // Watch external_command
    use_effect(move || {
        let cmd = external_cmd.read().clone();
        if let Some(cmd_str) = cmd {
            if !cmd_str.is_empty() {
                execute_command(cmd_str);
                // Clear command once executed
                if !is_minimized() {
                    external_cmd.set(None);
                } else {
                    is_minimized.set(false);
                    external_cmd.set(None);
                }
            }
        }
    });

    let handle_key_down = move |e: Event<KeyboardData>| {
        match e.key() {
            Key::Enter => {
                let input = current_input.read().clone();
                execute_command(input);
                current_input.set(String::new());
            }
            Key::ArrowUp => {
                let h = command_history.read();
                if !h.is_empty() {
                    let mut idx = history_index();
                    if idx < (h.len() as i32) - 1 {
                        idx += 1;
                    }
                    history_index.set(idx);
                    let cmd_idx = h.len() - 1 - (idx as usize);
                    current_input.set(h[cmd_idx].clone());
                }
            }
            Key::ArrowDown => {
                let h = command_history.read();
                let mut idx = history_index();
                if idx > 0 {
                    idx -= 1;
                    history_index.set(idx);
                    let cmd_idx = h.len() - 1 - (idx as usize);
                    current_input.set(h[cmd_idx].clone());
                } else {
                    history_index.set(-1);
                    current_input.set(String::new());
                }
            }
            _ => {}
        }
    };

    let handle_pointer_move = move |e: Event<PointerData>| {
        if let Some(offset) = dragging_header() {
            pos.set((
                e.client_coordinates().x - offset.0,
                e.client_coordinates().y - offset.1,
            ));
        } else if let Some(start) = resizing() {
            let width = f64::max(400.0, start.2 + (e.client_coordinates().x - start.0));
            let height = f64::max(300.0, start.3 + (e.client_coordinates().y - start.1));
            size.set((width, height));
        }
    };

    let handle_pointer_up = move |_| {
        dragging_header.set(None);
        resizing.set(None);
    };

    if is_minimized() {
        return rsx! {
            div {
                class: "fixed bottom-[40px] right-4 bg-secondary border border-border rounded-t-lg px-4 py-2 cursor-pointer shadow-lg animate-fade-in flex items-center gap-2 z-[60]",
                onclick: move |_| is_minimized.set(false),
                div { class: "w-2 h-2 rounded-full bg-terminal-green animate-pulse" }
                span { class: "text-xs font-mono", "Terminal (Guest@Portfolio)" }
            }
        };
    }

    let transform = if is_maximized() { "none".to_string() } else { format!("translate3d({}px, {}px, 0)", pos().0, pos().1) };
    let width_style = if is_maximized() { "100%".to_string() } else { format!("{}px", size().0) };
    let height_style = if is_maximized() { "100%".to_string() } else { format!("{}px", size().1) };
    let pos_style = if is_maximized() { "fixed" } else { "absolute" };
    let z_index = if is_maximized() { 50 } else { 40 };
    let drag_class = if dragging_header().is_some() || resizing().is_some() { "select-none" } else { "transition-all duration-200" };

    rsx! {
        div {
            style: "transform: {transform}; width: {width_style}; height: {height_style}; position: {pos_style}; top: 0; left: 0; z-index: {z_index};",
            class: "bg-card border border-border rounded-lg overflow-hidden shadow-2xl flex flex-col pointer-events-auto {drag_class}",
            onpointermove: handle_pointer_move,
            onpointerup: handle_pointer_up,
            onpointerleave: handle_pointer_up,
            
            // Header
            div {
                class: "flex items-center justify-between px-4 py-2 bg-secondary border-b border-border cursor-move select-none",
                onpointerdown: move |e| {
                    if !is_maximized() {
                        dragging_header.set(Some((
                            e.client_coordinates().x - pos().0,
                            e.client_coordinates().y - pos().1
                        )));
                    }
                },
                div { class: "flex items-center gap-2", span { class: "text-xs text-muted-foreground font-mono", "Guest@Portfolio:~" } }
                div { class: "flex items-center terminal-header-buttons",
                    button { class: "p-2 hover:bg-white/10 transition-colors", onclick: move |e| { e.stop_propagation(); is_minimized.set(true); },
                        svg { xmlns: "http://www.w3.org/2000/svg", width: "14", height: "14", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M5 12h14" } }
                    }
                    button { class: "p-2 hover:bg-white/10 transition-colors", onclick: move |e| { e.stop_propagation(); is_maximized.set(!is_maximized()); },
                        if is_maximized() {
                            svg { xmlns: "http://www.w3.org/2000/svg", width: "14", height: "14", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", path { d: "M8 8H20V20H8z" }, path { d: "M4 16V4H16" } }
                        } else {
                            svg { xmlns: "http://www.w3.org/2000/svg", width: "14", height: "14", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round", rect { x: "3", y: "3", width: "18", height: "18", rx: "2", ry: "2" } }
                        }
                    }
                }
            }

            // Body
            div {
                class: "flex-1 overflow-y-auto p-4 font-mono text-sm leading-relaxed terminal-scrollbar min-h-0",
                for line in lines() {
                    div { key: "{line.id}",
                        class: match &line.line_type {
                            LineType::Error => "text-terminal-red whitespace-pre-wrap",
                            LineType::Success => "text-terminal-green whitespace-pre-wrap",
                            LineType::Info => "text-terminal-cyan whitespace-pre-wrap",
                            _ => "text-foreground whitespace-pre-wrap"
                        },
                        match &line.line_type {
                            LineType::Input => {
                                rsx! { div { class: "flex", span { class: "text-terminal-green mr-2", "ren@portfolio:~$ " } span { "{line.content}" } } }
                            }
                            LineType::Component(elem) => {
                                rsx! { div { class: "animate-fade-in", {elem.clone()} } }
                            }
                            _ => {
                                rsx! { div { class: "animate-fade-in", "{line.content}" } }
                            }
                        }
                    }
                }

                // Input Line
                div {
                    class: "flex items-center mt-2",
                    span { class: "text-terminal-green mr-2", "ren@portfolio:~$ " }
                    input {
                        r#type: "text",
                        value: "{current_input()}",
                        oninput: move |e| current_input.set(e.value()),
                        onkeydown: handle_key_down,
                        class: "flex-1 bg-transparent outline-none text-foreground",
                        autofocus: true,
                        spellcheck: false,
                        autocomplete: "off",
                    }
                }
            }

            // Footer
            div {
                class: "px-4 py-2 bg-secondary border-t border-border text-xs text-muted-foreground flex justify-between select-none relative",
                span { "Type 'help' for commands" }
                div { class: "flex items-center gap-4",
                    span { "↑↓ History • Tab Autocomplete" }
                    if !is_maximized() {
                        div {
                            class: "cursor-nwse-resize p-1 hover:text-foreground transition-colors absolute right-0 bottom-0 z-50",
                            onpointerdown: move |e| {
                                resizing.set(Some((e.client_coordinates().x, e.client_coordinates().y, size().0, size().1)));
                            },
                            div { class: "w-3 h-3 border-r-2 border-b-2 border-muted-foreground/50 m-1" }
                        }
                    }
                }
            }
        }
    }
}
