use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, DrawingArea, Orientation, glib};
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.uxugin.matrixfun")
        .build();
    app.connect_activate(build_ui);
    app.run()
}
fn build_ui(app: &Application) {
    let main_box = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();
    let drawing_area = DrawingArea::builder()
        .width_request(500)
        .height_request(500)
        .build();
    main_box.append(&drawing_area);
    drawing_area.set_draw_func(|_drawing_area, context, _width, _height| {
        context.line_to(100.0, 100.0);
        context.line_to(300.0, 200.0);
        context.stroke().unwrap();
    });
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&main_box)
        .build();
    window.present();
}
