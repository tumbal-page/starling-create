fn create_particles_flush(game: &mut crate::game::GameLevel<'_>, dimension: &str) {
    crate::game::create_port_particles::flush(game, dimension);
}
