use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0
    });
    
    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };

    html! {
        <div style="padding: 15px;">
            <div style="height: 75%; width: 50%; float: left;" class="col-xs-6" >
                <button {onclick} class="btn btn-primary" style="height: 100%; width: 100%; font-size: 100px">{ "+1" }</button>
            </div> 
            <div style="font-size: 200px; width: 50%; float: right" class="col-xs-6">
                <p style="text-align: center; padding-top: 5%; ">{ state.value }</p>
            </div>
        </div>
    }
}

fn main () {
    yew::start_app::<App>();
}