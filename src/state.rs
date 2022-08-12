#[derive(PartialEq)]

pub enum State {
    Playing,
    Tutorial,
    BetweenPhases,
    Start,
    Started,
    Paused,
    Dead,
}
