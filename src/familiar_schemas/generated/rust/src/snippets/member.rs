//! Member - Represents a single user within a tenant, including their role and join date.
//! 
//! Generated from: Member.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Represents a single user within a tenant, including their role and join date.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Member {
    /// A canonical definition for a Universally Unique Identifier (UUID).
    pub user_id: String,
    /// Defines the roles a user can have within a tenant.
    pub role: String,
    /// A canonical definition for an ISO 8601 timestamp with timezone.
    pub joined_at: String,
}

impl Member {
    /// Create a new Member with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Member {
    fn default() -> Self {
        Self {
            user_id: String::new(),
            role: String::new(),
            joined_at: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_member_creation() {
        let instance = Member::new();
        assert_eq!(instance, Member::default());
    }

    #[test]
    fn test_member_serialization() {
        let instance = Member::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Member = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}