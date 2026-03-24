//! Code editor component with syntax highlighting.
//!
//! Uses the overlay technique: a transparent `<textarea>` sits on top of a
//! `<pre>` that shows the highlighted code. The textarea handles input while
//! the pre provides the colors.

use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::highlight;

#[derive(Properties, PartialEq)]
pub struct EditorProps {
    pub value: AttrValue,
    pub on_change: Callback<String>,
}

#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    let pre_ref = use_node_ref();

    // Sync scroll position from textarea to the highlighted pre
    let on_scroll = {
        let pre_ref = pre_ref.clone();
        Callback::from(move |e: Event| {
            if let Some(textarea) = e.target().and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
                && let Some(pre) = pre_ref.cast::<web_sys::HtmlElement>()
            {
                pre.set_scroll_top(textarea.scroll_top());
                pre.set_scroll_left(textarea.scroll_left());
            }
        })
    };

    let on_input = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target()
                && let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>()
            {
                on_change.emit(textarea.value());
            }
        })
    };

    let spans = highlight::highlight(&props.value);
    let highlighted: Html = spans
        .into_iter()
        .map(|s| {
            html! {
                <span style={format!("color:{}", s.color)}>{s.text}</span>
            }
        })
        .collect::<Html>();

    let container_style = "\
        position: relative; \
        flex: 1; \
        min-height: 0; \
        border: 1px solid #313244; \
        border-radius: 6px; \
        overflow: hidden; \
        background: #181825;";

    // Shared text styling for both layers
    let text_style = "\
        font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', monospace; \
        font-size: 14px; \
        line-height: 1.5; \
        padding: 12px; \
        tab-size: 4; \
        white-space: pre-wrap; \
        word-wrap: break-word; \
        overflow-wrap: break-word;";

    let pre_style = format!(
        "{text_style} \
         position: absolute; \
         top: 0; left: 0; right: 0; bottom: 0; \
         margin: 0; \
         overflow: auto; \
         pointer-events: none; \
         color: #cdd6f4;"
    );

    let textarea_style = format!(
        "{text_style} \
         position: absolute; \
         top: 0; left: 0; \
         width: 100%; height: 100%; \
         background: transparent; \
         color: transparent; \
         caret-color: #f5e0dc; \
         border: none; \
         outline: none; \
         resize: none; \
         z-index: 1;"
    );

    html! {
        <div style={container_style}>
            <pre ref={pre_ref} style={pre_style}>
                <code>{highlighted}</code>
            </pre>
            <textarea
                value={props.value.clone()}
                oninput={on_input}
                onscroll={on_scroll}
                spellcheck="false"
                autocomplete="off"
                autocorrect="off"
                autocapitalize="off"
                style={textarea_style}
            />
        </div>
    }
}
