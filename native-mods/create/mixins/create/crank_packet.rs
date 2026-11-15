// API 359 has no client block-entity tag receipt hook. This narrow raw adapter
// is pinned to Game::remote_block_entity_data's type/block/tag validation.
fn create_crank_packet(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    kind: i32,
    tag: &horizon_world::nbt::Tag,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port_press_client::packet(level, pos, kind, tag);
        crate::game::create_port_inventory_packet::packet(level, pos, kind, tag);
        crate::game::create_port_burner::packet(level, pos, kind, tag);
        crate::game::create_port_nativeaudio::crank_packet(level, pos, kind, tag);
    });
}
