fn motor_level(game: &mut crate::game::Game<'_>) {
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_control::level_tick(level);
        crate::game::create_port_kinetic_runtime::refresh_transmissions(level);
        crate::game::create_port_belt::maintenance(level);
        crate::game::create_port_basin::maintenance(level);
        crate::game::create_port_mixer::maintenance(level);
        crate::game::create_port_press_basin::maintenance(level);
        crate::game::create_port_instruments::flush(level);
        crate::game::create_port_millstone::refresh_recipes(level);
        crate::game::create_port_fan::maintenance(level);
        crate::game::create_port_drill::maintenance(level);
    });
}
// CI re-trigger: motor_level with_level fix
