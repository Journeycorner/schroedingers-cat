use azul::{
    prelude::*,
    widgets::{button::Button, label::Label},
};

struct DataModel {
    counter: usize,
}

impl Layout for DataModel {
    // Model renders View
    fn layout(&self, _info: WindowInfo<Self>) -> Dom<Self> {
        let label_cat = Label::new(match self.counter {
            0 => String::from("Box is empty"),
            1 => String::from("There is one kitten in the box"),
            n => format!("There are {} kittens in the box", n),
        })
        .dom();
        let button = Button::with_label(match self.counter {
            0 => String::from("Put a kitten into box"),
            _ => String::from("Put another kitten into the box"),
        })
        .dom()
        .with_callback(On::MouseUp, Callback(update_counter));
        let reset_button = Button::with_label(match self.counter {
            0 => String::from("Box is empty"),
            1 => String::from("Let the kitten out!"),
            _ => String::from("Let the kittens out!"),
        })
        .dom()
        .with_callback(On::MouseUp, Callback(reset_counter));
        Dom::new(NodeType::Div)
            .with_child(label_cat)
            .with_child(button)
            .with_child(reset_button)
    }
}

// View updates Model
fn update_counter(
    app_state: &mut AppState<DataModel>,
    _event: WindowEvent<DataModel>,
) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    UpdateScreen::Redraw
}

// View updates Model
fn reset_counter(
    app_state: &mut AppState<DataModel>,
    _event: WindowEvent<DataModel>,
) -> UpdateScreen {
    app_state.data.modify(|state| state.counter = 0);
    UpdateScreen::Redraw
}

fn main() {
    App::new(DataModel { counter: 0 }, AppConfig::default())
        .run(Window::new(WindowCreateOptions::default(), Css::native()).unwrap())
        .unwrap();
}
