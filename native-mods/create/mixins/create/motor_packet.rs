fn motor_packet(game: &mut crate::game::Game<'_>, player: u128, channel: &str, data: &[u8]) {
    game.with_level(|_, level, _| {
        crate::game::create_port_burner_sync::from_client(level, player, channel, data);
    });
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_control::receive(level, player, channel, data);
    });
}
