use thiserror::Error;

#[derive(Debug, Error)]
pub enum SrcRepoError {
    #[error("failed to get config")]
    Config,

    #[error("failed to read git directory")]
    Git(#[from] git2::Error),

    #[error("failed to get remote \"origin\"")]
    GitUrl,

    #[error("failed to parse git url")]
    GitUrlParseError(#[from] git_url_parse::GitUrlParseError),

    #[error("failed to determine home directory")]
    HomeDir,

    #[error("invalid characters in repo path")]
    RepoPath,
}
