use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
    tasks: Vec<Task>,
}

impl State {
    fn increase_status(&mut self, idx: usize) {
        self.tasks.get_mut(idx).filter(|e| e.status < 3).map(|e| e.status = e.status + 1);
    }
    fn decrease_status(&mut self, idx: usize) {
        self.tasks.get_mut(idx).filter(|e| e.status > 1).map(|e| e.status = e.status - 1);
    }
}

struct Task {
    name: String,
    assignee: String,
    estimate: u32,
    status: u32,
}

enum Msg {
    IncreaseStatus(usize),
    DecreaseStatus(usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            state: State {
                tasks: vec! [
                    Task { name: "Task 1".to_string(), assignee: "🐱".to_string(), estimate: 3, status: 1 },
                    Task { name: "Task 2".to_string(), assignee: "🐶".to_string(), estimate: 2, status: 1 },
                    Task { name: "Task 3".to_string(), assignee: "🐱".to_string(), estimate: 1, status: 2 },
                    Task { name: "Task 4".to_string(), assignee: "🐹".to_string(), estimate: 3, status: 3 },
                ]
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncreaseStatus(idx) => self.state.increase_status(idx),
            Msg::DecreaseStatus(idx) => self.state.decrease_status(idx),
        }
        true
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
                            { self.view_column(1, "未対応", &self.state.tasks) }
                            { self.view_column(2, "処理中", &self.state.tasks) }
                            { self.view_column(3, "完了"  , &self.state.tasks) }
                        </div>
                    </div>
                </section>
            </>
        }
    }
}

impl Model {
    fn view_column(&self, status: u32, status_text: &str, tasks: &Vec<Task>) -> Html {
        html! {
            <div class=format!("column status-{}", status)>
                <div class="tags has-addons">
                    <span class="tag">{ status_text }</span> <span class="tag is-dark">{ tasks.iter().filter(|e| e.status == status).count() }</span>
                </div>
                { for tasks.iter().enumerate().filter(|e| e.1.status == status).map(|e| self.view_task(e)) }
            </div>
        }
    }

    fn view_task(&self, (idx, task): (usize, &Task)) -> Html {
        html! {
            <div class="card">
                <div class="card-content">
                    { &task.name }
                </div>
                <footer class="card-footer">
                    <div class="card-footer-item">{ &task.assignee }</div>
                    <div class="card-footer-item">{ format!("{} 人日", &task.estimate) }</div>
                </footer>
                <footer class="card-footer">
                    <a class="card-footer-item" onclick={self.link.callback(move |_| Msg::DecreaseStatus(idx))}>{ "◀︎︎" }</a>
                    <a class="card-footer-item" onclick={self.link.callback(move |_| Msg::IncreaseStatus(idx))}>{ "▶︎︎" }</a>
                </footer>
            </div>
        }
    }
}


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
