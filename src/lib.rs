use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
}

enum Msg {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section class="section">
                    <div class="container">
                        <h1 class="title">
                            { "Hello, world!"}
                        </h1>
                        <p class="subtitle">
                            { "カンバンを作ってみる" }
                        </p>
                    </div>
                </section>

                <section id="board" class="section">
                    <div class="container">
                        <div class="columns">
                            { view_column(1, "未対応") }
                            { view_column(2, "処理中") }
                            { view_column(3, "完了") }
                        </div>
                    </div>
                </section>
            </>
        }
    }
}

fn view_column(status: u32, status_text: &str) -> Html {
    html! {
        <div class=format!("column status-{}", status)>
            <div class="tags has-addons">
                <span class="tag">{ status_text }</span> <span class="tag is-dark">{ 0 }</span>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
