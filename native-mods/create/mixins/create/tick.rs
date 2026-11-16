fn create_on_tick(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    entity: &mut crate::containers::ModBlockEntity,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port::tick(level, pos, entity);
    });
}
