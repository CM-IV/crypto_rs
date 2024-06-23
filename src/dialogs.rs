use fltk::{
    app,
    button::Button,
    enums::FrameType,
    frame, group, output,
    prelude::{GroupExt, InputExt, WidgetExt, WindowExt},
    window::Window,
};
use fltk_theme::WidgetScheme;

pub struct MyDialog {
    pub out: output::Output,
}

impl MyDialog {
    pub fn new(val: &str, title: &str, label: &str) -> Self {
        let mut win = Window::default().with_size(750, 100).with_label(title);

        let widget_scheme = WidgetScheme::new(fltk_theme::SchemeType::Fluent);
        widget_scheme.apply();

        // win.set_color(Color::from_rgb(240, 240, 240));
        frame::Frame::default().with_label(label).with_pos(170, 20);
        let mut pack = group::Pack::default()
            .with_size(550, 30)
            .center_of_parent()
            .with_type(group::PackType::Horizontal);
        pack.set_spacing(20);
        let mut out = output::Output::default().with_size(400, 0);
        out.set_value(val);
        out.set_frame(FrameType::FlatBox);
        let mut copy = Button::default().with_size(80, 0).with_label("Copy");
        let mut ok = Button::default().with_size(80, 0).with_label("Ok");
        pack.end();
        win.end();
        win.make_modal(true);
        win.show();
        copy.set_callback({
            let out_val = out.value();
            move |_| {
                app::copy(out_val.as_str());
            }
        });
        ok.set_callback({
            let mut win = win.clone();
            move |_| {
                win.hide();
            }
        });
        while win.shown() {
            app::wait();
        }
        Self { out }
    }
}
