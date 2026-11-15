fn motor_scroll(
    game: &mut crate::game::Game<'_>,
    screen: &crate::mod_api::NativeScreenView<'_>,
    scroll: [f64; 2],
    modifiers: u32,
    canceled: &mut bool,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_value_ui::scroll(level, screen, scroll, modifiers, canceled);
    });
}
