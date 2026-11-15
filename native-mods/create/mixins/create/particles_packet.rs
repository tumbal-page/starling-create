fn create_particles_packet(game: &mut crate::game::GameLevel<'_>, channel: &str, data: &[u8]) {
    crate::game::create_port_burner_sync::from_server(game, channel, data);
    crate::game::create_port_particles::packet(game, channel, data);
}
