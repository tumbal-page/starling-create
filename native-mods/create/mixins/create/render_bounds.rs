fn create_render_bounds(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    block: &str,
    entity: &crate::containers::ModBlockEntity,
    bounds: &mut [f32; 6],
) {
    game.with_level(|_, level, _| {
        crate::game::create_port_belt::render_bounds(level, pos, entity, bounds);
    });
    crate::game::create_port_mixer::bounds(entity, bounds);
    if block == "create:large_water_wheel" {
        *bounds = [-2.0, -2.0, -2.0, 3.0, 3.0, 3.0];
    }
}
