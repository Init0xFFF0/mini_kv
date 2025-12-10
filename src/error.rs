use thiserror::Error;
pub type KvResult<T = ()> = std::result::Result<T, KvsError>;
#[derive(Debug, Error)]
pub enum KvsError {
    #[error("IO错误{0}")]
    Io(#[from] std::io::Error),
    #[error("Serde{0}")]
    Serde(#[from] serde_json::Error),
    // #[error("数据库键未找到")]
    // KvNotFound,

    // #[error("命令不存在")]
    // InvalidCommand,

    // #[error("参数多了{0},{1}")]
    // ValueTooLarge(String, usize),

    // #[error("键 {key},值{value}")]
    // OperationFailed { key: String, value: &'static str },
}
