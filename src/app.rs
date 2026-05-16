use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};

use crate::browser::{ipc::BrowserCommand, window::BrowserWindow};

#[derive(Debug)]
pub enum UserEvent {
    Command(BrowserCommand),
}

pub fn run() {
    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Astra")
        .with_inner_size(LogicalSize::new(1280.0_f64, 800.0_f64))
        .build(&event_loop)
        .expect("failed to create window");

    let mut browser = BrowserWindow::new(&window, proxy)
        .expect("failed to create browser window");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                browser.resize(size, window.scale_factor());
            }
            Event::UserEvent(UserEvent::Command(cmd)) => {
                browser.handle_command(cmd);
            }
            _ => {}
        }
    });
}
