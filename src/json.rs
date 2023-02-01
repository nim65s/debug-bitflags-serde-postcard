extern crate alloc;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Start,
    Stop,
    Issue(Issue),
}

bitflags::bitflags! {
    #[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Issue: u8 {
        const WRONG_CRC = 0b0000_0001;
        const TIMEOUT = 0b0000_0010;
    }
}

/// wrap serde_json functions
#[allow(clippy::missing_errors_doc)]
pub trait Wire: serde::Serialize + for<'a> serde::Deserialize<'a> {
    fn to_string(self) -> serde_json::Result<alloc::string::String> {
        serde_json::to_string(&self)
    }

    fn from_str(value: &str) -> serde_json::Result<Self> {
        serde_json::from_str(value)
    }
}

impl Wire for Issue {}
impl Wire for Message {}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn wire_issue() {
        let issue_in = Issue::TIMEOUT | Issue::WRONG_CRC;
        std::println!("issue_in: {issue_in:?}");
        let serialied = issue_in.to_string().expect("can't serialize");
        std::println!("serialied: {serialied:?}");
        let issue_out = Issue::from_str(&serialied).expect("can't deserialize");
        assert_eq!(issue_in, issue_out);
    }

    #[test]
    fn wire_message() {
        let message_in = Message::Stop;
        std::println!("message_in: {message_in:?}");
        let serialied = message_in.to_string().expect("can't serialize");
        std::println!("serialied: {serialied:?}");
        let message_out = Message::from_str(&serialied).expect("can't deserialize");
        assert_eq!(message_in, message_out);
    }
}
