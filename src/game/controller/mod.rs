use super::*;

use model::*;

impl Game {
    pub fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key } => match key {
                geng::Key::Enter => {
                    self.model.tick();
                }
                _ => {}
            },
            _ => {}
        }
    }
}
