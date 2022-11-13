use fltk::{
    app,
    button::Button,
    dialog::alert,
    group::{Pack, PackType},
    input::Input,
    menu::Choice,
    prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};

const WIN_W: i32 = 450;
const WIN_H: i32 = 650;
const BORDER: i32 = 10;
const BUT_H: i32 = (WIN_W) / 3;
const BUT_W: i32 = (WIN_W - BORDER * 2) / 3;

fn main() {
    let app = app::App::default();

    let mut wind = Window::default()
        .with_label("MACRO9")
        .with_size(WIN_W, WIN_H)
        .center_screen();

    let vpack = Pack::new(
        BORDER,
        BORDER,
        WIN_W - (BORDER * 2),
        WIN_W * 2 - (BORDER * 2),
        "",
    );

    let mut hpack = Pack::new(0, 0, WIN_W - (BORDER * 2), BUT_H, "");
    let mut but_1 = Button::new(0, 0, BUT_W, BUT_H, "1");
    but_1.set_label_size(32);
    let mut but_2 = Button::new(0, 0, BUT_W, BUT_H, "2");
    but_2.set_label_size(32);
    let mut but_3 = Button::new(0, 0, BUT_W, BUT_H, "3");
    but_3.set_label_size(32);
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, WIN_W - (BORDER * 2), BUT_H, "");
    let mut but_4 = Button::new(0, 0, BUT_W, BUT_H, "4");
    but_4.set_label_size(32);
    let mut but_5 = Button::new(0, 0, BUT_W, BUT_H, "5");
    but_5.set_label_size(32);
    let mut but_6 = Button::new(0, 0, BUT_W, BUT_H, "6");
    but_6.set_label_size(32);
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, WIN_W - (BORDER * 2), BUT_H, "");
    let mut but_7 = Button::new(0, 0, BUT_W, BUT_H, "7");
    but_7.set_label_size(32);
    let mut but_8 = Button::new(0, 0, BUT_W, BUT_H, "8");
    but_8.set_label_size(32);
    let mut but_9 = Button::new(0, 0, BUT_W, BUT_H, "9");
    but_9.set_label_size(32);
    hpack.end();
    hpack.set_type(PackType::Horizontal);
    vpack.end();

    let modifiers_y_pos = WIN_W + BORDER * 3;
    let mut alt_ch = Choice::new(BORDER + 20, modifiers_y_pos, 70, 30, "ALT");
    alt_ch.add_choice("Left");
    alt_ch.add_choice("Right");
    alt_ch.add_choice("Both");
    let mut ctrl_ch = Choice::new(BORDER + 130, modifiers_y_pos, 70, 30, "CTRL");
    ctrl_ch.add_choice("Left");
    ctrl_ch.add_choice("Right");
    ctrl_ch.add_choice("Both");
    let mut shift_ch = Choice::new(BORDER + 245, modifiers_y_pos, 70, 30, "SHIFT");
    shift_ch.add_choice("Left");
    shift_ch.add_choice("Right");
    shift_ch.add_choice("Both");
    let mut meta_ch = Choice::new(BORDER + 360, modifiers_y_pos, 70, 30, "META");
    meta_ch.add_choice("Left");
    meta_ch.add_choice("Right");
    meta_ch.add_choice("Both");

    let keys_y_pos = WIN_W + BORDER * 3 + 50;

    let mut key1 = Input::new(BORDER + 48, keys_y_pos, 48, 30, "Keys:");
    let mut key2 = Input::new(BORDER + 10 + 48 * 2, keys_y_pos, 48, 30, "");
    let mut key3 = Input::new(BORDER + 10 * 2 + 48 * 3, keys_y_pos, 48, 30, "");
    let mut key4 = Input::new(BORDER + 10 * 3 + 48 * 4, keys_y_pos, 48, 30, "");
    let mut key5 = Input::new(BORDER + 10 * 4 + 48 * 5, keys_y_pos, 48, 30, "");
    let mut key6 = Input::new(BORDER + 10 * 5 + 48 * 6, keys_y_pos, 48, 30, "");

    let mut but_7 = Button::new(BORDER + 48, keys_y_pos + 50, 120, 50, "Send to device");

    wind.end();
    wind.show();
    //alert(0, 0, "ERROR");
    app.run().unwrap();
}
