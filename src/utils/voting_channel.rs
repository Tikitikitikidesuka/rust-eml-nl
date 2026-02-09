use thiserror::Error;

use crate::utils::StringValueData;

/// Voting method used in the election.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VotingChannelType {
    /// A physical polling station
    Polling,
    /// Votes by mail
    Postal,
}

impl VotingChannelType {
    /// Create a VotingMethod from a `&str`, if possible.
    pub fn from_str_value(s: &str) -> Option<Self> {
        match s {
            "polling" => Some(VotingChannelType::Polling),
            "postal" => Some(VotingChannelType::Postal),
            _ => None,
        }
    }

    /// Get the `&str` representation of this VotingMethod.
    pub fn to_str_value(&self) -> &'static str {
        match self {
            VotingChannelType::Polling => "polling",
            VotingChannelType::Postal => "postal",
        }
    }
}

/// Error returned when an unknown voting channel string is encountered.
#[derive(Debug, Clone, Error)]
#[error("Unknown voting channel: {0}")]
pub struct UnknownVotingChannelError(String);

impl StringValueData for VotingChannelType {
    type Error = UnknownVotingChannelError;

    fn parse_from_str(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Self::from_str_value(s).ok_or(UnknownVotingChannelError(s.to_string()))
    }

    fn to_raw_value(&self) -> String {
        self.to_str_value().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_voting_channel_types() {
        let valid_channels = ["polling", "postal"];
        for channel in valid_channels {
            assert!(
                VotingChannelType::from_str_value(channel).is_some(),
                "VotingChannelType should accept valid channel: {}",
                channel
            );
        }
    }

    #[test]
    fn test_invalid_voting_channel_types() {
        let invalid_channels = ["", "test", "abc"];
        for channel in invalid_channels {
            assert!(
                VotingChannelType::from_str_value(channel).is_none(),
                "VotingChannelType should reject invalid channel: {}",
                channel
            );
        }
    }
}
