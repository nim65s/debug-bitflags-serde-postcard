use postcard::experimental::max_size::MaxSize;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq, Eq, MaxSize)]
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

impl MaxSize for Issue {
    const POSTCARD_MAX_SIZE: usize = 1;
}

/// wrap postcard functions
#[allow(clippy::missing_errors_doc)]
pub trait Wire<const MAX_SIZE: usize>: serde::Serialize + for<'a> serde::Deserialize<'a> {
    fn to_vec(self) -> postcard::Result<heapless::Vec<u8, MAX_SIZE>> {
        postcard::to_vec(&self)
    }

    fn take_from_bytes(value: &[u8]) -> postcard::Result<(Self, &[u8])> {
        postcard::take_from_bytes(value)
    }
}

impl Wire<{ Self::POSTCARD_MAX_SIZE }> for Issue {}
impl Wire<{ Self::POSTCARD_MAX_SIZE }> for Message {}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn wire_issue() {
        let issue_in = Issue::TIMEOUT | Issue::WRONG_CRC;
        std::println!("issue_in: {issue_in:?}");
        let serialized = issue_in.to_vec().expect("can't serialize");
        std::println!("serialized: {serialized:?}");
        let (issue_out, extra) = Issue::take_from_bytes(&serialized).expect("can't deserialize");
        assert_eq!(issue_in, issue_out);
        assert_eq!(extra.len(), 0);
    }

    #[test]
    fn wire_message() {
        let message_in = Message::Stop;
        std::println!("message_in: {message_in:?}");
        let serialized = message_in.to_vec().expect("can't serialize");
        std::println!("serialized: {serialized:?}");
        let (message_out, extra) = Message::take_from_bytes(&serialized).expect("can't deserialize");
        assert_eq!(message_in, message_out);
        assert_eq!(extra.len(), 0);
    }
}
