use fltk::{
    app,
    button::Button,
    enums::FrameType,
    frame, group, output,
    prelude::{GroupExt, InputExt, WidgetExt, WindowExt},
    window::Window,
};
use fltk_theme::{WidgetTheme, ThemeType, WidgetScheme};

pub struct MyDialog {
    pub out: output::Output,
}

impl MyDialog {
    pub fn new(val: &str, title: &str, label: &str) -> Self {
        let mut win = Window::default().with_size(600, 100).with_label(title);

        let widget_theme = WidgetTheme::new(ThemeType::Dark);
        widget_theme.apply();

        let widget_scheme = WidgetScheme::new(fltk_theme::SchemeType::SvgBased);
        widget_scheme.apply();

        // win.set_color(Color::from_rgb(240, 240, 240));
        frame::Frame::default().with_label(label).with_pos(170, 20);
        let mut pack = group::Pack::default()
            .with_size(400, 30)
            .center_of_parent()
            .with_type(group::PackType::Horizontal);
        pack.set_spacing(20);
        let mut out = output::Output::default().with_size(350, 0);
        out.set_value(&val);
        out.set_frame(FrameType::FlatBox);
        let mut ok = Button::default().with_size(80, 0).with_label("Ok");
        pack.end();
        win.end();
        win.make_modal(true);
        win.show();
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
