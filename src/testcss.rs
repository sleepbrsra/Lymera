use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Notebook, Box, Button, CssProvider};

pub fn ru() {
    let application = Application::new(Some("com.example.app"), Default::default());

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        let notebook = Notebook::new();

        // Создание страниц
        let main_page = create_main_page();
        let friends_page = create_friends_page();

        notebook.append_page(&main_page, Some(&gtk::Label::new(Some("Основная"))));
        notebook.append_page(&friends_page, Some(&gtk::Label::new(Some("Друзья"))));

        window.add(&notebook);
        window.set_title("Lymera");
        window.set_default_size(400, 300);
        window.show_all();

        // Загрузка CSS
        let css_provider = CssProvider::new();
        css_provider.load_from_path("src/styles.css").expect("Failed to load CSS");
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Unable to get default screen"),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );
    });

    application.run();
}

// Функция для создания главной страницы
fn create_main_page() -> Box {
    let page = Box::new(gtk::Orientation::Vertical, 0);
    page.pack_start(&Button::with_label("Кнопка на главной странице"), true, true, 0);
    page
}

// Функция для создания страницы "Друзья"
fn create_friends_page() -> Box {
    let page = Box::new(gtk::Orientation::Vertical, 0);
    page.pack_start(&Button::with_label("Кнопка на странице друзей"), true, true, 0);
    page
}
