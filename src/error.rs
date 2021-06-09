#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SubclassError {
    NotMainThread,
    InstallFailed,
}
