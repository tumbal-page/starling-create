fn create_right_click(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    face: [i32; 3],
    canceled: &mut bool,
) {
    game.with_level(|_, level, _| {
        crate::game::create_port::right_click(level, pos, face, canceled);
    });
}
