use leptos::*;

const MAXWELL : &str = "\\begin{aligned}
\\frac{\\partial\\mathcal{D}}{\\partial t} \\quad & = \\quad \\nabla\\times\\mathcal{H},   & \\quad \\text{(Loi de Faraday)} \\\\[5pt]
\\frac{\\partial\\mathcal{B}}{\\partial t} \\quad & = \\quad -\\nabla\\times\\mathcal{E},  & \\quad \\text{(Loi d'Ampère)}   \\\\[5pt]
\\nabla\\cdot\\mathcal{B}                 \\quad & = \\quad 0,                         & \\quad \\text{(Loi de Gauss)}   \\\\[5pt]
\\nabla\\cdot\\mathcal{D}                 \\quad & = \\quad 0.                         & \\quad \\text{(Loi de Colomb)}
\\end{aligned}";

fn main() {
    let (read_txt, write_txt) = create_signal(String::from(MAXWELL));

    // the code that runs the conversion of the textfield input (read_txt.get())
    // and renders it to MathML.
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
                format!("<center class=\"bg-red-50 border border-red-500 text-red-900 placeholder-red-700 text-sm rounded-lg focus:ring-red-500 dark:bg-gray-700 focus:border-red-500 block w-full p-2.5 dark:text-red-500 dark:placeholder-red-500 dark:border-red-500\">Error while parsing. Check your input.</center>")
            },
            Err(err) => {
                logging::log!("{:?}", err);
                String::from("Weird error. Please inspect the log!")
            }
        }
    };

    // dynamically adjust the vertical size of the textarea field, by counting the rows manually.
    // remove this when firefox has field-sizing implemented.
    let count_rows = move || {
        read_txt.get().lines().count() + 1
    };

    let einstein = move |_| {
        write_txt(String::from("E = mc^2"));
    };

    mount_to_body(move || {
        view! { 
            <h1 class="text-3xl font-bold">Simple KaTeX demo using Leptos (Rust) only</h1>
            <div class="grid gap-6 mb-5 md:grid-cols-1">
            <p>
            This page has been created, without writing a single line of JavaScript, 
            but just Rust using Leptos. There is auto-generated JavaScript as glue code
            (wasm-bindgen). If this seems interesting to you, I can highly
            recommend reading the <a href="https://book.leptos.dev/">Leptos documentation</a>.
            The <a href="https://github.com/ellcs/leptos-katex/blob/master/src/main.rs">source code of this page</a> is also fairly easy. In release mode            less than 0.6 MiB
            data is transfered. This includes just two files: JavaScript glue code and the wasm
            file. He a look in your developer tools.
            </p> 
            <br/>
            <br/>
            <br/>
            <div inner_html=render_input/>
            <br/>
            <br/>
            <br/>
            <textarea
              class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
              on:input=move |ev| {
                  write_txt(event_target_value(&ev))
              }
              prop:value=read_txt
              rows=count_rows
            />
            <div class="md:grid-cols-2 flex space-x-4 gap-3">
              <button 
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                on:click=einstein>One stone.
              </button>
              <button 
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                on:click=einstein>One stone.
              </button>
            </div>
            </div>
        }
    })
}
