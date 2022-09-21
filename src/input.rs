use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

use crate::view::RenderView;

pub fn process_event(view: &mut RenderView, event: &WindowEvent) -> bool {
    match event {
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state,
                    virtual_keycode: Some(key),
                    ..
                },
            ..
        } => {
            let is_pressed = *state == ElementState::Pressed;
            match key {
                VirtualKeyCode::R => {
                    if is_pressed {
                        view.keys.rotation = !view.keys.rotation;
                    }
                }
                _ => return view.camera_controller.process_keyboard(*key, *state),
            }
        }

        WindowEvent::MouseWheel { delta, .. } => {
            view.camera_controller.process_scroll(delta);
        }

        WindowEvent::MouseInput {
            button: MouseButton::Left,
            state,
            ..
        } => {
            view.mouse_pressed = *state == ElementState::Pressed;
        }

        _ => return false,
    }

    true
}
