use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

pub fn process_event(view: &mut crate::view::View, event: &winit::event::WindowEvent) -> bool {
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
                    true
                }
                VirtualKeyCode::W | VirtualKeyCode::Up => {
                    view.keys.forward = is_pressed;
                    true
                }
                VirtualKeyCode::A | VirtualKeyCode::Left => {
                    view.keys.left = is_pressed;
                    true
                }
                VirtualKeyCode::S | VirtualKeyCode::Down => {
                    view.keys.backward = is_pressed;
                    true
                }
                VirtualKeyCode::D | VirtualKeyCode::Right => {
                    view.keys.right = is_pressed;
                    true
                }
                _ => false,
            }
        }
        _ => false,
    }
}
