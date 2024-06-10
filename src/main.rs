use leptos::*;
use wasm_bindgen::*;

fn main() {
    let (read_txt, write_txt) = create_signal(String::from("E = mc^2"));

    // the code that runs the conversion of the textfield input (read_txt.get())
    // and returns the result as a string.
    let render_input = move || {
        let txt = read_txt.get();

        let opts = katex::Opts::builder()
                        .display_mode(true)
                        .output_type(katex::OutputType::Mathml)
                        .build();
        let Ok(opts) = opts else {
            return String::from("Failed setting up options.");
        };

        match katex::render_with_opts(&txt, opts) {
            Ok(html_in_display_mode) => {
                html_in_display_mode
            },
            Err(katex::Error::JsExecError(err)) => {
                logging::log!("{:?}", err);
                format!("<center>Error while parsing. Check your input.</center>")
            },
            Err(err) => {
                logging::log!("{:?}", err);
                String::from("Weird error. Please inspect the log!")
            }
        }
    };

    // dynamically adjust the vertical size of the text field
    // remove this when firefox has field-sizing
    let count_rows = move || {
        read_txt.get().lines().count() + 1
    };

    mount_to_body(move || {
        view! { 
            <h4 style="width: 95%; margin-left: auto; margin-right: auto;">Simple KaTeX demo using Leptos (Rust) only</h4>
            <div style="width: 95%; margin-left: auto; margin-right: auto;">
            This page has been created, without writing a single line of JavaScript, 
            but just Rust using Leptos. There is auto-generated JavaScript as glue code (wasm-bindgen).             If this seems interesting to you, I can highly recommend reading the 
            <a href="url">Leptos documentation</a>. The source code is also fairly easy. In release mode            less than 0.6 MiB data is transfered, two files: The JavaScript glue code and the wasm file.
            </div> 
            <br/>
            <br/>
            <br/>
            <div inner_html=render_input/>
            <br/>
            <br/>

            <textarea
              on:input=move |ev| {
                  write_txt(event_target_value(&ev))
              }
              prop:value=read_txt
              rows=count_rows
              style="width: 90%;
                     margin-left: auto;
                     margin-right: auto;
                     display: block;
                     padding: 12px 20px;
                     box-sizing: border-box;
                     border: 2px solid #ccc;
                     border-radius: 4px;
                     background-color: #f8f8f8;
                     font-size: 16px;
                     resize: none;"
            />

        }
    })
}