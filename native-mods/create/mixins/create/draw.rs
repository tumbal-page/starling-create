fn create_on_draw(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    block: &str,
    state: horizon_world::registry::BlockStateId,
    entity: &crate::containers::ModBlockEntity,
    draw: &mut crate::game::native_block_entity_draw::BlockEntityDraw,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port::draw(level, pos, block, state, entity, draw);
    });
}
