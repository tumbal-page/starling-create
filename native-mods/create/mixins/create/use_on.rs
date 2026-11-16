fn create_use_on(
    game: &mut crate::game::Game<'_>,
    stack: &crate::items::ItemStack,
    pos: [i32; 3],
    face: [i32; 3],
    claimed: &mut bool,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port::use_on(level, stack, pos, face);
        if crate::game::create_port_placement::alias(level, stack, pos, face) {
            *claimed = true;
        }
    });
}
