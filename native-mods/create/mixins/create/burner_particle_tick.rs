// Level.tickBlockEntities: before the particle engine admits this tick's new effects.
fn create_burner_particle_tick(game: &mut crate::game::Game<'_>) {
    game.with_level(|_, level, _| {
        crate::game::create_port_press_client::tick(level);
        crate::game::create_port_burner_presentation::client_particles(level);
    });
}
