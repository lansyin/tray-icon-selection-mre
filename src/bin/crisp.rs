use tray_icon_patched::{Icon, TrayIconBuilder};
use winit::{event::Event, event_loop::EventLoop};

fn main() {
    let _tray_icon = TrayIconBuilder::new()
        .with_icon(Icon::from_path("./icon.ico", None).unwrap())
        .build()
        .unwrap();

    #[allow(deprecated)]
    EventLoop::new()
        .unwrap()
        .run(move |_event: Event<()>, _event_loop| {})
        .unwrap();
}
