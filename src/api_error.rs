//! Structured error type returned by Tauri command wrappers for the master
//! CRUD paths (shop, manufacturer, product, ...).
//!
//! Serialised into JSON before it reaches the frontend, so `catch (err)` in
//! JS receives an object of shape `{ code, message, entity? }` instead of an
//! opaque string. `code` is a snake_case constant the frontend classifier
//! (`res/js/master-crud.js`) matches on to pick the i18n key; `message` is
//! the human-friendly English fallback; `entity` is set for the master-name
//! errors so the classifier can drop it into the localised template.
//!
//! `From<String>` is deliberately NOT implemented: every validation
//! `Result<(), String>` must be converted explicitly with
//! `.map_err(ApiError::validation)?`, so a bare stringly error can never
//! be silently misclassified as validation when it originated from a DB
//! failure.

use serde::Serialize;
use std::fmt;

/// Wire-level API error. All Tauri command wrappers in master CRUD paths
/// return `Result<T, ApiError>`; Tauri serialises the Err arm via serde
/// straight into the JS `catch` binding.
#[derive(Debug, Serialize, Clone)]
pub struct ApiError {
    /// Snake-case classification key. Matched by the JS `mapMasterErrorCode`
    /// helper. Never internationalise this — it is a stable machine code.
    pub code: String,

    /// Human-friendly English fallback. The frontend uses this only when the
    /// code is unknown or when the mapped i18n key is not registered yet.
    pub message: String,

    /// Master-entity slug (`"shop"`, `"manufacturer"`, `"product"`, ...)
    /// for the master-name errors. `None` for validation and database
    /// errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
}

impl ApiError {
    // ---- Code constants ------------------------------------------------
    // Kept as `&'static str` so tests can assert against them by reference
    // and the JS side can keep the strings in one shared place.

    pub const CODE_DUPLICATE_NAME: &'static str = "duplicate_name";
    pub const CODE_DUPLICATE_CODE: &'static str = "duplicate_code";
    pub const CODE_NOT_FOUND: &'static str = "not_found";
    pub const CODE_MANUFACTURER_NOT_FOUND: &'static str = "manufacturer_not_found";
    pub const CODE_ADMIN_PROTECTED: &'static str = "admin_protected";
    pub const CODE_IN_USE: &'static str = "in_use";
    pub const CODE_VALIDATION: &'static str = "validation";
    pub const CODE_DATABASE: &'static str = "database";
    // PR14 (Fable-5 #21) — auth-specific codes so the login / setup
    // screens can map to their own i18n messages instead of dumping the
    // raw English `Err` string next to a Japanese label.
    pub const CODE_AUTH_INVALID_CREDENTIALS: &'static str = "auth_invalid_credentials";
    pub const CODE_AUTH_SETUP_COMPLETED: &'static str = "auth_setup_completed";
    // Fable-5 #1/#5 — password change rejected because the caller-supplied
    // "current password" did not verify against the stored hash. Distinct
    // from `auth_invalid_credentials` (login) so the user-management
    // screen can point the user at the correct input.
    pub const CODE_OLD_PASSWORD_INCORRECT: &'static str = "old_password_incorrect";

    // ---- Constructors --------------------------------------------------

    /// Duplicate master-name row already exists for this user.
    pub fn duplicate_name(entity: &str) -> Self {
        Self {
            code: Self::CODE_DUPLICATE_NAME.to_string(),
            message: format!("{} name already exists", entity),
            entity: Some(entity.to_lowercase()),
        }
    }

    /// Duplicate master-code row already exists for this user. Distinct
    /// from `duplicate_name` because Account (and any future
    /// code-identified master) checks the CODE column, not the NAME
    /// column, so the inline error should blame the code field, not the
    /// name field. The i18n key stays `${prefix}.duplicate_error` for
    /// per-screen wording; only the target field differs.
    pub fn duplicate_code(entity: &str) -> Self {
        Self {
            code: Self::CODE_DUPLICATE_CODE.to_string(),
            message: format!("{} code already exists", entity),
            entity: Some(entity.to_lowercase()),
        }
    }

    /// Target row missing (concurrent delete, cross-owner id, or a stale
    /// UI reference).
    pub fn not_found(entity: &str) -> Self {
        Self {
            code: Self::CODE_NOT_FOUND.to_string(),
            message: format!("{} not found", entity),
            entity: Some(entity.to_lowercase()),
        }
    }

    /// Operation refused because the target is the admin user, which
    /// the app protects from deletion / role change / etc. Distinct
    /// from a generic validation error because the UI needs its own
    /// dedicated "administrator cannot be removed" wording, and
    /// separate from `not_found` because the row was located and is
    /// intentionally off-limits.
    pub fn admin_protected(entity: &str) -> Self {
        Self {
            code: Self::CODE_ADMIN_PROTECTED.to_string(),
            message: format!("Admin {} cannot be modified this way", entity.to_lowercase()),
            entity: Some(entity.to_lowercase()),
        }
    }

    /// Delete refused because the master row is still referenced by
    /// other data (transactions, recurring rules, product master, etc.).
    /// The frontend maps this to a "cannot delete: still in use" toast
    /// so the user is directed to disable the row instead of removing it.
    pub fn in_use(entity: &str) -> Self {
        Self {
            code: Self::CODE_IN_USE.to_string(),
            message: format!("{} is still in use and cannot be deleted", entity),
            entity: Some(entity.to_lowercase()),
        }
    }

    /// Product references a manufacturer_id whose owner check failed
    /// (either the row does not exist or belongs to another user).
    /// Kept distinct from generic `not_found` because the UI needs to
    /// blame the manufacturer field, not the product row.
    pub fn manufacturer_not_found() -> Self {
        Self {
            code: Self::CODE_MANUFACTURER_NOT_FOUND.to_string(),
            message: "Manufacturer not found".to_string(),
            entity: Some("manufacturer".to_string()),
        }
    }

    /// Validation error (empty, too long, ...). Passes the original English
    /// message from `validation.rs` through as the fallback text.
    pub fn validation(msg: impl Into<String>) -> Self {
        Self {
            code: Self::CODE_VALIDATION.to_string(),
            message: msg.into(),
            entity: None,
        }
    }

    /// Wraps an unexpected database error. The frontend does not classify
    /// on the message contents — it just shows a generic "save failed"
    /// toast — so any specific text is safe to keep for logs.
    pub fn database(msg: impl Into<String>) -> Self {
        Self {
            code: Self::CODE_DATABASE.to_string(),
            message: msg.into(),
            entity: None,
        }
    }

    /// Login failed because the username / password did not match a
    /// registered user. PR14 (Fable-5 #21): the frontend maps this to
    /// `error.invalid_credentials` so the user sees a localised
    /// "wrong username or password" toast instead of the mixed-language
    /// `<Japanese label>: Invalid username or password`.
    pub fn auth_invalid_credentials() -> Self {
        Self {
            code: Self::CODE_AUTH_INVALID_CREDENTIALS.to_string(),
            message: "Invalid username or password".to_string(),
            entity: None,
        }
    }

    /// Password change refused because the user-supplied "current
    /// password" did not verify. Fable-5 #1/#5: separated from generic
    /// validation so the edit modal can highlight the current-password
    /// field with its own wording (`error.old_password_incorrect`)
    /// instead of showing a raw English "Old password is incorrect"
    /// message in the form-message slot.
    pub fn old_password_incorrect() -> Self {
        Self {
            code: Self::CODE_OLD_PASSWORD_INCORRECT.to_string(),
            message: "Current password is incorrect".to_string(),
            entity: None,
        }
    }

    /// Registration refused because the initial-setup path is one-shot:
    /// admin setup runs once at first launch, and general-user setup
    /// runs once per admin. PR14 (Fable-5 #21): the frontend maps this
    /// to `error.setup_completed`.
    pub fn auth_setup_completed(msg: impl Into<String>) -> Self {
        Self {
            code: Self::CODE_AUTH_SETUP_COMPLETED.to_string(),
            message: msg.into(),
            entity: None,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ApiError {}

impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        Self::database(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_name_carries_lowercased_entity_and_stable_code() {
        let err = ApiError::duplicate_name("Shop");
        assert_eq!(err.code, "duplicate_name");
        assert_eq!(err.entity.as_deref(), Some("shop"));
        assert!(err.message.contains("already exists"));
    }

    #[test]
    fn not_found_carries_lowercased_entity_and_stable_code() {
        let err = ApiError::not_found("Manufacturer");
        assert_eq!(err.code, "not_found");
        assert_eq!(err.entity.as_deref(), Some("manufacturer"));
    }

    #[test]
    fn duplicate_code_carries_lowercased_entity_and_distinct_code() {
        let err = ApiError::duplicate_code("Account");
        assert_eq!(err.code, "duplicate_code");
        assert_ne!(err.code, ApiError::CODE_DUPLICATE_NAME);
        assert_eq!(err.entity.as_deref(), Some("account"));
        assert!(err.message.contains("code already exists"));
    }

    #[test]
    fn admin_protected_carries_lowercased_entity_and_stable_code() {
        let err = ApiError::admin_protected("User");
        assert_eq!(err.code, "admin_protected");
        assert_eq!(err.entity.as_deref(), Some("user"));
        assert!(err.message.contains("Admin"));
    }

    #[test]
    fn in_use_carries_lowercased_entity_and_stable_code() {
        let err = ApiError::in_use("Shop");
        assert_eq!(err.code, "in_use");
        assert_eq!(err.entity.as_deref(), Some("shop"));
        assert!(err.message.contains("still in use"));
    }

    #[test]
    fn manufacturer_not_found_has_its_own_code() {
        let err = ApiError::manufacturer_not_found();
        assert_eq!(err.code, "manufacturer_not_found");
        assert_eq!(err.entity.as_deref(), Some("manufacturer"));
    }

    #[test]
    fn validation_carries_message_through_and_omits_entity() {
        let err = ApiError::validation("Shop name cannot be empty");
        assert_eq!(err.code, "validation");
        assert_eq!(err.message, "Shop name cannot be empty");
        assert!(err.entity.is_none());
    }

    #[test]
    fn database_from_sqlx_row_not_found() {
        let sqlx_err = sqlx::Error::RowNotFound;
        let err: ApiError = sqlx_err.into();
        assert_eq!(err.code, "database");
        assert!(err.entity.is_none());
    }

    #[test]
    fn serialises_with_snake_case_code_and_optional_entity() {
        let err = ApiError::duplicate_name("Shop");
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["code"], "duplicate_name");
        assert_eq!(json["entity"], "shop");
        assert!(json["message"].as_str().unwrap().contains("Shop"));
    }

    #[test]
    fn serialises_without_entity_key_when_none() {
        let err = ApiError::validation("Password too short");
        let json = serde_json::to_value(&err).unwrap();
        assert!(json.get("entity").is_none());
        assert_eq!(json["code"], "validation");
    }
}
