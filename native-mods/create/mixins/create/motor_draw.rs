fn motor_draw(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    screen: &crate::mod_api::NativeScreenView<'_>,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_value_ui::draw(level, batch, screen);
    });
}
