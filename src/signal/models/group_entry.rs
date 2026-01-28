use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub admins: Vec<String>,
    pub members: Vec<String>,
    pub blocked: bool,
    pub internal_id: String,
    pub invite_link: Option<String>,
    pub pending_invites: Vec<String>,
    pub pending_members: Vec<String>,
}
