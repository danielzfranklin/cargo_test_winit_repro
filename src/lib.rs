#[cfg(test)]
mod tests {
    #[test]
    fn with_winit() {
        use winit::{
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        };

        // NOTE: This is required because new_any_thread is only supported on
        // Windows and Linux
        #[cfg(target_os = "linux")]
        use winit::platform::unix::EventLoopExtUnix;
        #[cfg(target_os = "windows")]
        use winit::platform::windows::EventLoopExtWindows;

        let event_loop = EventLoop::<()>::new_any_thread();
        let _window = WindowBuilder::new().build(&event_loop).unwrap();

        event_loop.run(move |_event, _, control_flow| {
            *control_flow = ControlFlow::Exit;
        });
    }

    #[test]
    fn without_winit() {}
}
