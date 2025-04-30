pub mod user_api;

/*const SONG_LOOKUP: &[fn()] = &[
    super_mario,
    starwars_imperial,
    tetris,
    aerodynamic,
    nyancat,
    alle_meine_entchen,
    intro,
    doom
];*/

#[derive(Copy, Clone)]
pub enum SongID {
    super_mario = 0,
    starwars_imperial = 1,
    tetris = 2,
    aerodynamic = 3,
    nyancat = 4,
    alle_meine_entchen = 5,
    intro = 6,
    doom = 7,
}
