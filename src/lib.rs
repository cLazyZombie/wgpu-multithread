use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::event_loop::EventLoop;

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowBuilderExtWebSys;

#[wasm_bindgen(start)]
pub async fn init() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).unwrap();

    log::info!("init");
}

#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
    log::info!("run");

    let Some(window) = web_sys::window() else {
        return Err(JsValue::from_str("no window"));
    };

    // init wgpu
    let event_loop = EventLoop::new();

    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
    let size = (canvas.client_width() as u32, canvas.client_height() as u32);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        ..Default::default()
    });

    let builder = winit::window::WindowBuilder::new();
    let win = builder
        .with_title("wgpu multithread")
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();
    let surface = unsafe { instance.create_surface(&win).unwrap() };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, _queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    // wgpu::Limits::downlevel_webgl2_defaults()
                    wgpu::Limits::default()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        )
        .await
        .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);

    #[cfg(not(feature = "multithread"))]
    {
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![surface_caps.formats[0]],
        };
        surface.configure(&device, &config);
    }

    #[cfg(feature = "multithread")]
    wasm_thread::spawn(move || {
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![surface_caps.formats[0]],
        };
        surface.configure(&device, &config);
    });
    // let config = wgpu::SurfaceConfiguration {
    //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    //     format: surface_caps.formats[0],
    //     width: size.0,
    //     height: size.1,
    //     present_mode: wgpu::PresentMode::Fifo,
    //     alpha_mode: wgpu::CompositeAlphaMode::Auto,
    //     view_formats: vec![surface_caps.formats[0]],
    // };
    // surface.configure(&device, &config);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if web_sys::window().is_none() {
            return;
        }

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    if web_sys::window().is_none() {
        return;
    }

    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
