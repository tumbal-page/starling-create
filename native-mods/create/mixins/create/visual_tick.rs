fn create_visual_tick(game: &mut crate::game::Game<'_>) {
    game.with_level(|_, level, _| {
        crate::game::create_port_burner_sync::tick(level);
        crate::game::create_port_nativeaudio::begin_tick(level);
        crate::game::create_port_fan::client_tick(level);
        crate::game::create_port_presentation::tick(level);
        crate::game::create_port_burner_presentation::client_tick(level);
        crate::game::create_port_millstone::client_tick(level);
        crate::game::create_port_basin::client_tick(level);
        crate::game::create_port_mixer::client_tick(level);
        crate::game::create_port_nativeaudio::end_tick(level);
    });
}
