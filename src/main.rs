extern crate allegro;
extern crate allegro_font;
extern crate allegro_ttf;

use allegro::*;
use std::collections::HashSet;

mod geometry;
#[cfg(test)]
mod geometry_test;
mod game_state;
mod ui;

use ui::*;
use game_state::*;

allegro_main! {
    let ui_config = UIConfig::default();
    let core = Core::init().unwrap();

    let ui = UI::new(&core, &ui_config);
    let timer = Timer::new(&core, 1.0 / 100.0).unwrap();
    let queue = EventQueue::new(&core).unwrap();
    core.install_keyboard().unwrap();

    queue.register_event_source(ui.display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(core.get_keyboard_event_source().unwrap());

    let mut redraw = true;
    timer.start();

    let mut game = GameState::make_initial();
    let mut keys_state_set: HashSet<KeyCode> = HashSet::new();

    'exit: loop {
        if redraw && queue.is_empty() {
            ui.render(&game);
            redraw = false;
        }

        let event = queue.wait_for_event();
        match event {
            KeyDown { keycode, .. } => {
                keys_state_set.insert(keycode);
                ();
            },
            KeyUp { keycode, .. } => {
                keys_state_set.remove(&keycode);
                ()
            },
            _ => (),
        };

        match event {
            DisplayClose{..} => break 'exit,
            TimerTick{ timestamp, .. } => {
                game.tick(timestamp, KeyboardState::from_key_set(&keys_state_set));
                redraw = true;
            },
            KeyDown { keycode: KeyCode::Escape, .. } =>
                break 'exit,
            KeyDown { keycode: KeyCode::P, .. } =>
                println!("{:#?}", game),
            _ => (),
        }
    }
}
