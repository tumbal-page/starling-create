fn motor_release(
    game: &mut crate::game::Game<'_>,
    screen: &crate::mod_api::NativeScreenView<'_>,
    button: i32,
    canceled: &mut bool,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_value_ui::mouse_release(level, screen, button, canceled);
    });
}
