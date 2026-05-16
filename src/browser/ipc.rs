use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IpcMessage {
    Navigate { url: String },
    Back,
    Forward,
    Reload,
    NewTab { url: Option<String> },
}

#[derive(Debug)]
pub enum BrowserCommand {
    Navigate(String),
    Back,
    Forward,
    Reload,
    NewTab(Option<String>),
    PageLoaded(String),
    SetLoading(bool),
}

impl IpcMessage {
    pub fn into_command(self, resolver: impl Fn(&str) -> String) -> BrowserCommand {
        match self {
            IpcMessage::Navigate { url } => BrowserCommand::Navigate(resolver(&url)),
            IpcMessage::Back           => BrowserCommand::Back,
            IpcMessage::Forward        => BrowserCommand::Forward,
            IpcMessage::Reload         => BrowserCommand::Reload,
            IpcMessage::NewTab { url } => BrowserCommand::NewTab(url),
        }
    }
}
