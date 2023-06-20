use rltk::prelude::*;
use crate::{State, rex_assets::RexAssets};

#[derive(PartialEq, Copy, Clone)]
pub enum CreditsResult { NoSelection, QuitToMenu }

pub fn credits(gs : &mut State, ctx : &mut Rltk) -> CreditsResult {
    let mut draw_batch = DrawBatch::new();
    let assets = gs.ecs.fetch::<RexAssets>();
    ctx.render_xp_sprite(&assets.menu, 0, 0);
    draw_batch.draw_double_box(Rect::with_size(17, 13, 45, 12), ColorPair::new(RGB::named(rltk::WHEAT), RGB::named(rltk::BLACK)));
    draw_batch.print_color_centered(
        15,
        "AUTORZY",
        ColorPair::new(RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK))
    );
    draw_batch.print_color_centered(
        17,
        "Gzijebeziu Zebyr Zyjgolem - wszystko",
        ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK))
    );
    draw_batch.print_color_centered(
        18,
        "Wielki Wezyr Al Zahir - concept art",
        ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK))
    );
    draw_batch.print_color_centered(
        19,
        "Maciej - creative assistance",
        ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK))
    );

    draw_batch.print_color_centered(
        23,
        "Wcisnij dowolny klawisz, by wyjsc do menu.",
        ColorPair::new(RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK))
    );

    draw_batch.submit(6000).expect("Unable to submit");

    match ctx.key {
        None => CreditsResult::NoSelection,
        Some(_) => CreditsResult::QuitToMenu
    }
}