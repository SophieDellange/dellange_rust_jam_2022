#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum State {
    Startup,
    Loading,
    Title,
    Play,
}
