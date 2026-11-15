fn motor_hud(game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    game.with_level(|_, level, _| {
        crate::game::create_port_motor_value_ui::hud(level, batch);
        crate::game::create_port_presentation::hud(level, batch);
    });
}
