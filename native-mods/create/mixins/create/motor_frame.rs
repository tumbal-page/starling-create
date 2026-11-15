fn motor_frame(game: &mut crate::game::Game<'_>) {
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_value_ui::frame(level);
    });
}
