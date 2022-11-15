use fltk::{
    app,
    button::Button,
    dialog::alert,
    enums::Color,
    group::{Pack, PackType},
    input::Input,
    menu::Choice,
    prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};

mod keys;

use keys::*;

const WIN_W: i32 = 450;
const WIN_H: i32 = 650;
const BORDER: i32 = 10;
const BUT_H: i32 = (WIN_W) / 3;
const BUT_W: i32 = (WIN_W - BORDER * 2) / 3;

#[derive(Clone, Copy, Debug)]
enum Message {
    Pressed(u8),
    SendToDevice,
    Save,
}

#[derive(Debug)]
struct State {
    pub keys: Option<KeypadConfig>,
    pub modified: bool,
    pub current_button: Option<usize>,
}

fn main() {
    let app = app::App::default();

    let mut wind = Window::default()
        .with_label("MACRO9 configurator")
        .with_size(WIN_W, WIN_H)
        .center_screen();

    let (s, r) = app::channel::<Message>();

    let mut state = State {
        keys: None,
        modified: false,
        current_button: None,
    };

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

    let mut send_to_device = Button::new(BORDER, keys_y_pos + 50, 115, 50, "Send to device");
    let mut save_button = Button::new(BORDER * 2 + 115, keys_y_pos + 50, 65, 50, "Save");

    let mut serial_port = Input::new(BORDER + 280, keys_y_pos + 50, 150, 30, "Serial port:");

    #[cfg(target_os = "linux")]
    {
        let m9_serial_port = get_macro9_serial();

        if let Ok(port) = m9_serial_port {
            serial_port.set_value(&port);
        }
    }

    but_1.emit(s, Message::Pressed(1));
    but_2.emit(s, Message::Pressed(2));
    but_3.emit(s, Message::Pressed(3));
    but_4.emit(s, Message::Pressed(4));
    but_5.emit(s, Message::Pressed(5));
    but_6.emit(s, Message::Pressed(6));
    but_7.emit(s, Message::Pressed(7));
    but_8.emit(s, Message::Pressed(8));
    but_9.emit(s, Message::Pressed(9));

    send_to_device.emit(s, Message::SendToDevice);
    save_button.emit(s, Message::Save);

    let mut buttons = [
        but_1, but_2, but_3, but_4, but_5, but_6, but_7, but_8, but_9,
    ];
    alt_ch.set_value(0);
    alt_ch.set_value(-1);
    wind.end();
    wind.show();
    //alert(0, 0, "ERROR");
    //app.run().unwrap();
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Pressed(n) => {
                    let n = n as usize;
                    // change current button
                    state.current_button = Some(n);
                    // read config into fields
                    if let Some(ref keypad) = state.keys {
                        let key = match &keypad.keys[n - 1] {
                            Modified::Yes(k) => k,
                            Modified::No(k) => k,
                        };

                        // set modifiers
                        let modifiers = key.modifier;
                        let alt: i8 =
                            ((modifiers & 0x04) >> 2) as i8 + ((modifiers & 0x40) >> 5) as i8 - 1;
                        alt_ch.set_value(alt as i32);
                        let ctrl: i8 =
                            (modifiers & 0x01) as i8 + ((modifiers & 0x10) >> 3) as i8 - 1;
                        ctrl_ch.set_value(ctrl as i32);
                        let shift: i8 =
                            ((modifiers & 0x02) >> 1) as i8 + ((modifiers & 0x20) >> 4) as i8 - 1;
                        shift_ch.set_value(shift as i32);
                        let meta: i8 =
                            ((modifiers & 0x08) >> 3) as i8 + ((modifiers & 0x80) >> 6) as i8 - 1;
                        meta_ch.set_value(meta as i32);

                        // set keys
                        let clear_if_zero = |keycode: u8| -> String {
                            if keycode == 0 {
                                "".to_string()
                            } else {
                                keycode.to_string()
                            }
                        };
                        key1.set_value(&clear_if_zero(key.keycodes[0]));
                        key2.set_value(&clear_if_zero(key.keycodes[1]));
                        key3.set_value(&clear_if_zero(key.keycodes[2]));
                        key4.set_value(&clear_if_zero(key.keycodes[3]));
                        key5.set_value(&clear_if_zero(key.keycodes[4]));
                        key6.set_value(&clear_if_zero(key.keycodes[5]));
                    }
                }
                Message::SendToDevice => {
                    // check if anything is modified
                    if state.modified {}
                    // serialize and send packet
                }
                Message::Save => {
                    // check and read fields into config

                    // change key Modified state

                    // change color
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn get_macro9_serial() -> std::io::Result<String> {
    const VENDOR_ID: &str = "16c0";
    const PRODUCT_ID: &str = "27db";

    let error = || std::io::Error::new(std::io::ErrorKind::Other, "Could not find serial port");

    let context = libudev::Context::new()?;
    let mut enumerator = libudev::Enumerator::new(&context)?;
    enumerator.match_subsystem("tty")?;
    enumerator.match_property("ID_VENDOR_ID", VENDOR_ID)?;
    enumerator.match_property("ID_MODEL_ID", PRODUCT_ID)?;
    let mut devices = enumerator.scan_devices()?;

    let port = devices
        .next()
        .ok_or_else(error)?
        .devnode()
        .ok_or_else(error)?
        .display()
        .to_string();

    Ok(port)
}
