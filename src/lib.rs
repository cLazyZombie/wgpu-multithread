use std::rc::Rc;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub async fn init() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).unwrap();

    log::info!("init");
}

#[wasm_bindgen]
pub async fn run() {
    log::info!("run");

    let once = Rc::new(tokio::sync::OnceCell::<String>::new());
    let (tx, mut rx) = tokio::sync::watch::channel::<Option<String>>(None);

    wasm_bindgen_futures::spawn_local({
        let once = Rc::clone(&once);
        async move {
            gloo_timers::future::sleep(std::time::Duration::from_secs(2)).await;
            tx.send(Some("hello".to_string())).unwrap();

            once.get_or_init(|| async {
                log::info!("get_or_init");
                gloo_timers::future::sleep(std::time::Duration::from_secs(2)).await;
                "hello".to_string()
            })
            .await;
        }
    });

    rx.changed().await.unwrap();
    let borrowed = rx.borrow();
    let v = borrowed.as_ref().unwrap();
    log::info!("v: {}", v);

    // log::info!("before sleep 1");
    // // gloo_timers::future::sleep(std::time::Duration::from_secs(1)).await;
    // log::info!("after sleep 1");
    //
    // let v = once.get_or_init(|| async { "world".to_string() }).await;
    // log::info!("get_or_init: {}", v);

    loop {
        gloo_timers::future::sleep(std::time::Duration::from_secs(3)).await;
        log::info!("loop");
    }
}

// fn window() -> web_sys::Window {
//     web_sys::window().expect("no global `window` exists")
// }
