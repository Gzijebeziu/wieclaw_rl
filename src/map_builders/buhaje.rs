use super::{BuilderChain, XStart, YStart, AreaStartingPosition, CullUnreachable, VoronoiSpawning,
    AreaEndingPosition, XEnd, YEnd, BspInteriorBuilder};

pub fn buhaj_city(new_depth: i32, width: i32, height: i32) -> BuilderChain {
    println!("Buhaj builder");
    let mut chain = BuilderChain::new(new_depth, width, height, "Buhajówka");
    chain.start_with(BspInteriorBuilder::new());
    chain.with(AreaStartingPosition::new(XStart::CENTER, YStart::CENTER));
    chain.with(CullUnreachable::new());
    chain.with(AreaStartingPosition::new(XStart::RIGHT, YStart::CENTER));
    chain.with(AreaEndingPosition::new(XEnd::LEFT, YEnd::CENTER));
    chain.with(VoronoiSpawning::new());
    chain
}