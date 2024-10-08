use crate::project_view::ProjectView;

use gpui::*;
use project::Project;
use theme::*;

mod project_view;

fn main() {
    App::new().run(|cx| {
        settings::init(cx);

        editor::init(cx);

        theme::init(LoadThemes::JustBase, cx);

        language::init(cx);

        Project::init_settings(cx);

        let _ = cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(300.), px(300.)),
                    cx,
                ))),
                ..Default::default()
            },
            |cx| cx.new_view(|cx| ProjectView::new("rust gpui test".to_string(), cx)),
        );
    });
}
