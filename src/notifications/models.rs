use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationAutomation {
    pub id: String,
    pub name: String,
    pub chat_ids: Vec<String>,
    pub automation_type: AutomationType,
    pub notification_sound: Option<String>,
    pub focus_chat: bool,
    pub loop_config: Option<LoopConfig>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AutomationType {
    #[serde(rename = "loop")]
    Loop,
    #[serde(rename = "immediate")]
    Immediate,
}

impl std::fmt::Display for AutomationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutomationType::Loop => write!(f, "Loop"),
            AutomationType::Immediate => write!(f, "Immediate"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopConfig {
    pub until: LoopUntil,
    pub time: Option<u64>,
    #[serde(default = "default_check_interval")]
    pub check_interval: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum LoopUntil {
    #[serde(rename = "message_seen")]
    MessageSeen,
    #[serde(rename = "answer")]
    Answer,
    #[serde(rename = "for_a_time")]
    ForATime,
}

impl std::fmt::Display for LoopUntil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoopUntil::MessageSeen => write!(f, "Message Seen"),
            LoopUntil::Answer => write!(f, "Answer"),
            LoopUntil::ForATime => write!(f, "For A Time"),
        }
    }
}

fn default_check_interval() -> u64 {
    3000
}

impl NotificationAutomation {
    pub fn new(id: String, name: String, chat_ids: Vec<String>) -> Self {
        Self {
            id,
            name,
            chat_ids,
            automation_type: AutomationType::Immediate,
            notification_sound: None,
            focus_chat: false,
            loop_config: None,
            enabled: true,
        }
    }
}
