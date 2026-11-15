fn motor_open(
    game: &mut crate::game::Game<'_>,
    button: i32,
    hand: horizon_client::first_person::Hand,
    swing: &mut bool,
    canceled: &mut bool,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_value_ui::interaction(level, button, hand, swing, canceled);
    });
}
