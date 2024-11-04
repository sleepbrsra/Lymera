// ui.rs
use gtk::prelude::*;
use gtk::{Box, Label, ListBox, Orientation, Entry, Button};

// Функция для создания боковой панели
pub fn create_sidebar() -> Box {
    let sidebar = Box::new(Orientation::Vertical, 5);
    sidebar.set_width_request(200);
    sidebar.set_name("sidebar");  // Изменено

    let label = Label::new(Some("Channels"));
    label.set_name("sidebar-header");  // Изменено
    sidebar.append(&label);

    let channels = ListBox::new();
    let channel_names = vec!["# general", "# random", "# help"];
    for name in channel_names {
        let label = Label::new(Some(name));
        channels.append(&label);
    }
    sidebar.append(&channels);

    sidebar
}

// Функция для создания окна чата
pub fn create_chat_window() -> Box {
    let chat_window = Box::new(Orientation::Vertical, 5);
    chat_window.set_name("chat-window");  // Изменено

    let chat_header = Label::new(Some("# general"));
    chat_header.set_name("chat-header");  // Изменено
    chat_window.append(&chat_header);

    let messages = ListBox::new();
    let example_messages = vec!["User1: Hi everyone!", "User2: Hello!"];
    for msg in example_messages {
        let label = Label::new(Some(msg));
        messages.append(&label);
    }
    chat_window.append(&messages);

    let input_box = Box::new(Orientation::Horizontal, 5);
    let input = Entry::new();
    input.set_placeholder_text(Some("Message #general"));
    input_box.append(&input);

    let send_button = Button::with_label("Send");
    input_box.append(&send_button);

    chat_window.append(&input_box);

    chat_window
}

// Функция для создания списка пользователей
pub fn create_user_list() -> Box {
    let user_list = Box::new(Orientation::Vertical, 5);
    user_list.set_width_request(200);
    user_list.set_name("user-list");  // Изменено

    let label = Label::new(Some("Online Users"));
    label.set_name("user-list-header");  // Изменено
    user_list.append(&label);

    let users = ListBox::new();
    let user_names = vec!["User1", "User2", "User3"];
    for name in user_names {
        let label = Label::new(Some(name));
        users.append(&label);
    }
    user_list.append(&users);

    user_list
}
