use rltk::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum GameOverResult { NoSelection, QuitToMenu }

pub fn game_over(ctx : &mut Rltk) -> GameOverResult {
    let mut draw_batch = DrawBatch::new();
    draw_batch.print_color_centered(
        15,
        "No i klops, Wieclaw zmarl na zawal!",
        ColorPair::new(RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK))
    );
    draw_batch.print_color_centered(
        17,
        "Ale nie martw sie, karetka juz go zabrala i wkrótce",
        ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK))
    );
    draw_batch.print_color_centered(
        18,
        "znów bedzie mógl udac sie na wyprawe po Zombek.",
        ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK))
    );

    draw_batch.print_color_centered(
        19,
        &format!("Wieclaw przetrwal {} tur.", crate::gamelog::get_event_count("Turn")),
        ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK))
    );
    draw_batch.print_color_centered(
        20,
        &format!("Otrzymane obrazenia: {}.", crate::gamelog::get_event_count("Damage Taken")),
        ColorPair::new(RGB::named(rltk::RED), RGB::named(rltk::BLACK))
    );
    draw_batch.print_color_centered(
        21,
        &format!("Zadane obrazenia: {}.", crate::gamelog::get_event_count("Damage Inflicted")),
        ColorPair::new(RGB::named(rltk::RED), RGB::named(rltk::BLACK))
    );

    draw_batch.print_color_centered(
        23,
        "Wcisnij dowolny klawisz by wyjsc do menu.",
        ColorPair::new(RGB::named(rltk::MAGENTA), RGB::named(rltk::BLACK))
    );

    draw_batch.submit(6000).expect("Unable to submit");

    match ctx.key {
        None => GameOverResult::NoSelection,
        Some(_) => GameOverResult::QuitToMenu
    }
}