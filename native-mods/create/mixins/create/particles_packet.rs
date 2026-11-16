fn create_particles_packet(game: &mut crate::game::Game<'_>, channel: &str, data: &[u8]) {
    game.with_level(|_, level, _| {
        crate::game::create_port_burner_sync::from_server(level, channel, data);
    });
    game.with_level(|_, level, _| {
        crate::game::create_port_particles::packet(level, channel, data);
    });
}
