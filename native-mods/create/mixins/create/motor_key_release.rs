fn motor_key_release(
    game: &mut crate::game::Game<'_>,
    screen: &crate::mod_api::NativeScreenView<'_>,
    key: &str,
    canceled: &mut bool,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_value_ui::key_release(level, screen, key, canceled);
    });
}
