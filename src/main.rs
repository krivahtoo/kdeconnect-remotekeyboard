use client::{
    remotekeyboard::{Keys, RemoteKeyboard, SpecialKey},
    Connection, Daemon, Device,
};

use {egui_miniquad as egui_mq, miniquad as mq};

mod client;
mod queue;

struct Stage {
    egui_mq: egui_mq::EguiMq,
    pixels_per_point: f32,
    input: String,
    status: bool,
    conn: Connection,
    devices: Vec<(String, String)>,
    selected_device: usize,
    keys: queue::Queue<Keys>,
}

impl Stage {
    fn new(ctx: &mut mq::Context) -> Self {
        let conn = Connection::new().expect("unable to start dbus connection");
        Self {
            egui_mq: egui_mq::EguiMq::new(ctx),
            pixels_per_point: 1.23,
            input: "".into(),
            status: false,
            conn,
            devices: Vec::new(),
            selected_device: 0,
            keys: queue::Queue::new(),
        }
    }
}

impl mq::EventHandler for Stage {
    fn update(&mut self, _ctx: &mut mq::Context) {
        let daemon = Daemon::new(&self.conn);
        self.devices.clear();
        if let Ok(devices) = daemon.get_devices(true, true) {
            for device in devices.iter() {
                self.devices.push((
                    device.get_name().unwrap_or_default(),
                    device.get_id().into(),
                ));
            }
        };
        if !self.devices.is_empty() {
            let device = Device::new(&self.conn, self.devices[self.selected_device].1.clone());
            let rkeyboard: RemoteKeyboard = device.get_keyboard();
            self.status = rkeyboard.get_state().unwrap_or_default();
            if self.status {
                if let Some(k) = self.keys.peek() {
                    if rkeyboard.send_key(k.clone()).is_ok() {
                        self.keys.dequeue();
                    }
                }
            }
        }
    }

    fn draw(&mut self, mq_ctx: &mut mq::Context) {
        mq_ctx.clear(Some((1., 1., 1., 1.)), None, None);
        mq_ctx.begin_default_pass(mq::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        mq_ctx.end_render_pass();

        // Run the UI code:
        self.egui_mq.run(mq_ctx, |_mq_ctx, egui_ctx| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    egui::widgets::global_dark_light_mode_switch(ui);
                });

                egui::ComboBox::from_label(if self.status {
                    "Available"
                } else {
                    "Unavailable"
                })
                .show_index(
                    ui,
                    &mut self.selected_device,
                    self.devices.len(),
                    |i| {
                        if !self.devices.is_empty() {
                            self.devices[i].0.clone()
                        } else {
                            "No active Device".into()
                        }
                    },
                );

                ui.vertical(|ui| {
                    if self.status {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.input)
                                    .hint_text("Type to relay via kde...")
                                    .interactive(false),
                            );
                        });
                    }
                });
            });

            // Don't change scale while dragging the slider
            if !egui_ctx.is_using_pointer() {
                egui_ctx.set_pixels_per_point(self.pixels_per_point);
            }
        });

        // Draw things behind egui here

        self.egui_mq.draw(mq_ctx);

        // Draw things in front of egui here

        mq_ctx.commit_frame();
    }

    fn mouse_motion_event(&mut self, _: &mut mq::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _: &mut mq::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        if self.status && character.is_ascii() {
            self.keys.enqueue(Keys::Char(character));
        }
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        if self.status {
            match keycode {
                mq::KeyCode::Backspace => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Backspace));
                }
                mq::KeyCode::Escape => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Escape));
                },
                mq::KeyCode::Enter => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Enter));
                },
                mq::KeyCode::Tab => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Tab));
                }
                mq::KeyCode::Delete => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Delete));
                }
                mq::KeyCode::Right => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Right));
                }
                mq::KeyCode::Left => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Left));
                }
                mq::KeyCode::Down => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Down));
                }
                mq::KeyCode::Up => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Up));
                }
                mq::KeyCode::KpEnter => {
                    self.keys.enqueue(Keys::Special(SpecialKey::Enter));
                }
                _ => (),
            }
        }
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let conf = mq::conf::Conf {
        high_dpi: true,
        window_width: 300,
        window_height: 130,
        window_resizable: false,
        ..Default::default()
    };
    mq::start(conf, |ctx| Box::new(Stage::new(ctx)));
}
