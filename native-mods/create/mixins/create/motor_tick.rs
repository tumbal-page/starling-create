fn motor_tick(game: &mut crate::game::Game<'_>) {
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_value_world::tick(level);
        crate::game::create_port_motor_value_ui::tick(level);
    });
}
