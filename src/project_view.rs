use gpui::*;
use ui::prelude::*;
use ui_input::TextField;

pub struct ProjectView {
    name: String,
    todos: Vec<String>,
    input: View<TextField>,
}

fn get_text(element: &View<TextField>, cx: &mut WindowContext) -> String {
    element
        .read(cx)
        .editor()
        .read(cx)
        .text(cx)
        .trim()
        .to_string()
}

impl ProjectView {
    pub fn new(name: String, cx: &mut ViewContext<Self>) -> Self {
        Self {
            name,
            todos: Vec::new(),
            input: cx.new_view(|cx| TextField::new(cx, "new-todo", "new todo")),
        }
    }

    fn add_todo(&mut self, cx: &mut ViewContext<Self>) {
        let new_todo = get_text(&self.input, cx);
        if !new_todo.is_empty() {
            self.todos.push(new_todo);
            self.input.update(cx, |input, cx| {
                input
                    .editor()
                    .update(cx, |editor, cx| editor.set_text("", cx))
            });
        }
    }

    fn delete_todo(&mut self, index: usize) {
        if index < self.todos.len() {
            self.todos.remove(index);
        }
    }
}

impl Render for ProjectView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let todos = self.todos.clone();
        let name = self.name.clone();

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(Rems(1.0))
            .size_full()
            .bg(rgb(0x1f2335))
            .text_color(rgb(0xffffff))
            .children(vec![
                div().child(Label::new(name)),
                div().flex().gap(Rems(0.5)).children(vec![
                    div().child(self.input.clone().into_element()),
                    div().child(
                        Button::new("add", "add").on_click(cx.listener(|this, _, cx| {
                            this.add_todo(cx);
                        })),
                    ),
                ]),
                div()
                    .flex()
                    .flex_col()
                    .gap(Rems(0.5))
                    .children(todos.iter().enumerate().map(|(index, todo)| {
                        let todo = todo.clone();
                        div().flex().items_center().gap(Rems(0.5)).children(vec![
                            div().child(Label::new(todo)),
                            div().child(Button::new(("delete", index), "x").on_click(cx.listener(
                                move |this, _, _cx| {
                                    this.delete_todo(index);
                                },
                            ))),
                        ])
                    })),
            ])
    }
}
