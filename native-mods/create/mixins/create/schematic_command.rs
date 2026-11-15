fn create_schematic_command(
    game: &mut crate::game::Game<'_>,
    player: &crate::mod_api::NativeServerPlayer,
    command: &str,
    ran: &mut bool,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port_schematic_runtime::command(level, player, command, ran);
    });
}
