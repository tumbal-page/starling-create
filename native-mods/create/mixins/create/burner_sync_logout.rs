fn create_burner_sync_logout(game: &mut crate::game::Game<'_>, player: u128) {
    game.with_level(|_, level, _| {
        crate::game::create_port_burner_sync::logged_out(level, player);
    });
}
