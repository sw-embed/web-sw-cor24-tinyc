mod compiler;

use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

const DEFAULT_SOURCE: &str = r#"// COR24 C — UART hello + LED
void putc(int c) {
    *(char *)0xFF0100 = c;
}

void led_on() {
    *(char *)0xFF0000 = 0;
}

int main() {
    putc(72);   // H
    putc(105);  // i
    putc(33);   // !
    led_on();
    return 42;
}
"#;

#[function_component(App)]
fn app() -> Html {
    let source = use_state(|| DEFAULT_SOURCE.to_string());
    let assembly = use_state(String::new);
    let output = use_state(String::new);

    let on_input = {
        let source = source.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target()
                && let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>()
            {
                source.set(textarea.value());
            }
        })
    };

    let on_run = {
        let source = source.clone();
        let assembly = assembly.clone();
        let output = output.clone();
        Callback::from(move |_: MouseEvent| {
            let result = compiler::compile_and_run(&source);
            assembly.set(result.assembly);
            output.set(result.output);
        })
    };

    html! {
        <main style="display:flex; flex-direction:column; height:100vh; padding:16px; gap:12px;">
            <h1 style="font-size:1.4rem; color:#89b4fa;">
                {"web-tc24r"}
                <span style="font-size:0.8rem; color:#6c7086; margin-left:8px;">
                    {"COR24 compiler in your browser"}
                </span>
            </h1>

            <div style="display:flex; flex:1; gap:12px; min-height:0;">
                // C source editor
                <div style="flex:1; display:flex; flex-direction:column; gap:8px;">
                    <label style="font-size:0.85rem; color:#a6adc8;">{"C Source"}</label>
                    <textarea
                        value={(*source).clone()}
                        oninput={on_input}
                        spellcheck="false"
                        style="flex:1; background:#181825; color:#cdd6f4; border:1px solid #313244; \
                               border-radius:6px; padding:12px; font-family:monospace; font-size:14px; \
                               resize:none; outline:none;"
                    />
                </div>

                // Generated assembly
                <div style="flex:1; display:flex; flex-direction:column; gap:8px;">
                    <label style="font-size:0.85rem; color:#a6adc8;">{"Generated Assembly"}</label>
                    <pre style="flex:1; background:#181825; color:#f9e2af; border:1px solid #313244; \
                                border-radius:6px; padding:12px; font-family:monospace; font-size:14px; \
                                overflow:auto; white-space:pre-wrap;">
                        {&*assembly}
                    </pre>
                </div>

                // Execution output
                <div style="flex:1; display:flex; flex-direction:column; gap:8px;">
                    <label style="font-size:0.85rem; color:#a6adc8;">{"Output"}</label>
                    <pre style="flex:1; background:#181825; color:#a6e3a1; border:1px solid #313244; \
                                border-radius:6px; padding:12px; font-family:monospace; font-size:14px; \
                                overflow:auto; white-space:pre-wrap;">
                        {&*output}
                    </pre>
                </div>
            </div>

            <button onclick={on_run}
                style="align-self:flex-start; padding:8px 24px; background:#89b4fa; color:#1e1e2e; \
                       border:none; border-radius:6px; font-size:1rem; font-weight:600; cursor:pointer;">
                {"Compile & Run"}
            </button>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
