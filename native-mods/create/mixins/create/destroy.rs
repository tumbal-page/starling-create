fn create_destroy(game: &mut crate::game::Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    game.with_level(|_, level, _| {
        if crate::game::create_port::inventory_blocked(level, pos) {
            *canceled = true;
            return;
        }
        if level.native_block(pos) == "create:basin" {
            crate::game::create_port_basin::removed(level, pos);
        }
        crate::game::create_port_belt::destroy(level, pos);
        crate::game::create_port_placement::destroy(level, pos, canceled);
    });
}
