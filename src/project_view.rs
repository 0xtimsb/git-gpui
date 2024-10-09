use gpui::*;
use std::path::PathBuf;
use ui::prelude::*;

pub struct ProjectView {
    name: String,
    selected_directory: Option<PathBuf>,
}

impl ProjectView {
    pub fn new(name: String) -> Self {
        Self {
            name,
            selected_directory: None,
        }
    }

    fn open_project(&mut self, cx: &mut ViewContext<Self>) {
        // Use the native file dialog to select a directory
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.selected_directory = Some(path);
            cx.notify();
        }
    }
}

impl Render for ProjectView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(Rems(1.0))
            .size_full()
            .bg(rgb(0x1f2335))
            .text_color(rgb(0xffffff))
            .children(vec![
                div().child(Label::new(self.name.clone())),
                div().child(Button::new("op", "Open Project").on_click(cx.listener(
                    |this, _event, cx| {
                        this.open_project(cx);
                    },
                ))),
                self.selected_directory
                    .as_ref()
                    .map(|path| {
                        div().child(Label::new(format!(
                            "Selected directory: {}",
                            path.display()
                        )))
                    })
                    .unwrap_or_else(|| div().child(Label::new("No directory selected"))),
            ])
    }
}
