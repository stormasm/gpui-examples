use gpui::*;

struct HelloWorld {
    text: SharedString,
}

impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .child(
                div()
                    .flex()
                    .bg(rgb(0x2e7d32))
                    .size(Length::Definite(Pixels(100.0).into()))
                    .justify_center()
                    .items_center()
                    .shadow_lg()
                    .border()
                    .border_color(rgb(0x0000ff))
                    .text_xl()
                    .text_color(rgb(0xffffff))
                    .child(format!("A {}", &self.text)),
            )
            .child(
                div()
                    .flex()
                    .bg(rgb(0x2e7d32))
                    .size(Length::Definite(Pixels(100.0).into()))
                    .justify_center()
                    .items_center()
                    .shadow_lg()
                    .border()
                    .border_color(rgb(0x0000ff))
                    .text_xl()
                    .text_color(rgb(0xffffff))
                    .child(format!("B {}", &self.text)),
            )
            .child(
                div()
                    .flex()
                    .bg(rgb(0x2e7d32))
                    .size(Length::Definite(Pixels(100.0).into()))
                    .justify_center()
                    .items_center()
                    .shadow_lg()
                    .border()
                    .border_color(rgb(0x0000ff))
                    .text_xl()
                    .text_color(rgb(0xffffff))
                    .child(format!("C {}", &self.text)),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds {
                    origin: Default::default(),
                    size: size(px(1000.), px(500.)).into(),
                }),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| HelloWorld { text: "D".into() }),
        );
        cx.activate(true);
    });
}
