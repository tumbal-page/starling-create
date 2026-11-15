fn motor_world_reset(game: &mut crate::game::Game<'_>) {
    game.with_level(|_, level, _| {
        crate::game::create_port_burner_sync::clear(level);
        crate::game::create_port_nativeaudio::unload(level);
        crate::game::create_port_motor_value_world::reset();
        crate::game::create_port_kinetic_runtime::reset();
        crate::game::create_port_basin::reset(false);
        crate::game::create_port_basin_recipes::reset(false);
        crate::game::create_port_press_basin::reset(false);
        crate::game::create_port_mixer::reset(false);
        crate::game::create_port_press_client::reset();
        crate::game::create_port_burner_presentation::reset();
    });
}
