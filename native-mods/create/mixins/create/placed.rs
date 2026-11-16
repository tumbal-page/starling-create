fn create_placed(game: &mut crate::game::Game<'_>, stack: &crate::items::ItemStack, pos: [i32; 3]) {
    game.with_level(|_, level, _| {
        crate::game::create_port::placed(level, stack, pos);
    });
    if game.native_on("create")
        && game.with_level(|_, level, _| { crate::game::create_port::node(level, pos).is_some() })
    {
        game.with_level(|_, level, _| {
            crate::game::create_port_particles::queue_rotation(level, pos);
        });
    }
}
