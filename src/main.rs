use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use gloo_worker::oneshot::oneshot;
use gloo_worker::{Registrable, Spawnable};
use wasm_bindgen::prelude::*;

fn main() {
    dioxus::launch(App);
}

// use gloo worker's oneshot macro to define a worker struct
#[oneshot]
async fn Squared(input: u32) -> u32 {
    input.pow(2)
}

#[component]
fn App() -> Element {
    let rsc = use_resource(|| async move {
        // consuming the worker, registration is done below by manually set a wasm-bindgen function
        let mut squared_bridge = Squared::spawner().spawn_with_loader("/assets/worker.js");
        let rst = squared_bridge.run(2).await;
        info!("Squared result: {:?}", rst);
        rst
    });

    rsx! {
        h1 { "Hello, Dioxus!" }
        h2 {
            "Worker result: "
            if let Some(x) = *rsc.read_unchecked() {
                "{x}"
            } else {
                "worker is working..."
            }
        }
    }
}

// register the worker, compile to wasm and use worker.js to call it
#[wasm_bindgen]
pub async fn register_gloo_worker() {
    Squared::registrar().register();
    info!("Gloo worker registered");
}
