use geng::prelude::*;
use geng::{Camera2d, Draw2d};

mod assets;
mod game;

use assets::*;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Untitled Circle Game".to_owned(),
        ..Default::default()
    });
    let assets = <Assets as geng::LoadAsset>::load(&geng, &static_path());

    geng::run(
        &geng,
        geng::LoadingScreen::new(&geng, geng::EmptyLoadingScreen, assets, {
            let geng = geng.clone();
            move |assets| {
                let assets = assets.unwrap();
                let assets = Rc::new(assets);
                game::Game::new(&geng, &assets)
            }
        }),
    )
}
