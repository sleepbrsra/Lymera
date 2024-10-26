extern crate gtk;

use gtk::prelude::*;
use gtk::{Label, Window, Button, Box, Image};

pub fn run() {
    // Инициализируем GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Создаем главное окно
    let window = Window::new(gtk::WindowType::Toplevel);
    window.set_title("Hello, GTK!");
    window.set_default_size(300, 100);

    // Создаем вертикальный контейнер для виджетов
    let vbox = Box::new(gtk::Orientation::Vertical, 5);

    // Загружаем изображение
    let image = Image::from_file("/lymera/src/pics.jpg"); // Замените на путь к вашему изображению

    // Создаем метку
    let label = Label::new(Some("Привет, GTK!")); // Текст метки

    // Создаем кнопку
    let button = Button::with_label("Выход");

    // Обработчик для кнопки выхода
    button.connect_clicked(|_| {
        gtk::main_quit();
    });

    // Добавляем виджеты в контейнер
    vbox.pack_start(&image, true, true, 0); // Добавляем изображение
    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&button, true, true, 0);

    // Добавляем контейнер в окно
    window.add(&vbox);

    // Обработчик для закрытия окна
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Показываем все виджеты
    window.show_all();

    // Запускаем главный цикл
    gtk::main();
}
