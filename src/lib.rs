extern crate console_error_panic_hook;

use std::f32::consts::FRAC_PI_8;

use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlCanvasElement};
use wgpu::util::DeviceExt;
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::EventLoop,
    platform::web::WindowBuilderExtWebSys,
    window::WindowBuilder,
};
use crate::gfx::{Vertex, ApplicationState};

mod gfx;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn get_canvas() -> HtmlCanvasElement {
    let document = window().document().unwrap();
    let canvas = document.get_element_by_id("webgpu-canvas").unwrap();
    canvas.dyn_into::<HtmlCanvasElement>().expect("Couldn't find canvas element")
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, 0.5, 0.0], color: [0.0, 0.0, 1.0] },
    Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 1.0, 1.0] },
];

const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,
];

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let event_loop = EventLoop::new();

    let canvas = get_canvas();
    // console::log_1(&JsValue::from_str(format!("Found canvas, width: {}, height: {}", canvas.client_width(), canvas.client_height()).as_str()));
    let size = PhysicalSize::new(canvas.client_width(), canvas.client_height());
    let window = WindowBuilder::new()
        .with_canvas(Some(canvas))
        .with_inner_size(size)
        .build(&event_loop)
        .unwrap();

    // console::log_1(&JsValue::from_str(format!("Window size, width: {}, height: {}", size.width, size.height).as_str()));

    let state = ApplicationState::new(&window).await;

    let vertex_buffer = state.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );

    let index_buffer = state.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    );

    let mut mouse_rotating = false;
    event_loop.run(move |event, _, _| match event {
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion { delta: (x_diff, y_diff), },
            ..
        } => {
            if mouse_rotating {
                let angle_change = FRAC_PI_8 / 128.;
                console::log_1(&JsValue::from_str(format!("Rotating camera horizontally by {:?} rad", -angle_change * x_diff as f32).as_str()));
                console::log_1(&JsValue::from_str(format!("Rotating camera vertically by {:?} rad", angle_change * y_diff as f32).as_str()));
                // scene.rotate_camera_horizontally(-angle_change * x_diff as f32, &mut vulkan);
                // scene.rotate_camera_vertically(angle_change * y_diff as f32, &mut vulkan);
            }
        }
        Event::WindowEvent {
            event,
            ..
        } => {
            match event {
                WindowEvent::MouseInput {
                    button: MouseButton::Left,
                    state,
                    ..
                } => {
                    mouse_rotating = state == ElementState::Pressed;
                }
                _ => {}
            }
        }
        Event::RedrawRequested(_) => {
            let output = state.surface.get_current_texture().unwrap();
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.015_7,
                                g: 0.,
                                b: 0.360_7,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    })],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&state.render_pipeline);
                render_pass.set_bind_group(0, &state.camera_bind_group, &[]);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
            }

            // submit will accept anything that implements IntoIter
            state.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }
        _ => {}
    });
    // Ok(())
}
