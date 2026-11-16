fn create_particles_flush(game: &mut crate::game::Game<'_>, dimension: &str) {
    game.with_level(|_, level, _| {
        crate::game::create_port_particles::flush(level, dimension);
    });
}
