use yew::prelude::*;

enum Msg {
    Inc,
}

struct CounterComponent {
    count: i32,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Inc => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <button onclick={ link.callback(|_| Msg::Inc) }>{ "Increment" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
