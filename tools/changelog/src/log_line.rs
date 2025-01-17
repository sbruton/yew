#[derive(Debug)]
pub struct LogLine {
    pub message: String,
    pub user: String,
    pub issue_id: String,
    pub is_breaking_change: bool,
}
