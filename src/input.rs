use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

pub fn process_event(view: &mut crate::view::View, event: &winit::event::WindowEvent) -> bool {
    match event {
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(key),
                    ..
                },
            ..
        } => match key {
            VirtualKeyCode::R => {
                view.keys.rotation = !view.keys.rotation;
                true
            }
            _ => false,
        },
        _ => false,
    }
}
