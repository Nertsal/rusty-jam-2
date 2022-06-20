use super::*;

mod controller;
mod model;
mod render;

use controller::Controller;
use model::*;
use render::Render;

pub struct Game {
    render: Render,
    model: Model,
    controller: Controller,
}

pub enum PlayerAction {
    GrabShape(Id),
    ReleaseGrabbed,
    EndTurn,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            render: Render::new(geng, assets),
            model: Model::new(),
            controller: Controller::new(),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Color::BLACK), None);
        self.render.draw(&self.model, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        if let Some(action) = self
            .controller
            .handle_event(&self.model, &self.render, event)
        {
            self.model.handle_player_action(action);
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);
        self.model.update(delta_time);
    }
}
