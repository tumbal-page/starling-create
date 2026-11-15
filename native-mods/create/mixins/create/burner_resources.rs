fn create_burner_resources_reloaded(game: &mut crate::game::Game<'_>) {
    game.with_level(|_, level, _| {
        crate::game::create_port_burner_presentation::resources_reloaded(level);
    });
}
