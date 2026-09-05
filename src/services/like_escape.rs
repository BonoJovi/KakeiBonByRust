//! Shared LIKE-pattern escaping for user-supplied search text.
//!
//! Fable-5 review #23: `search_products_by_name` used to pass the
//! user's query straight into `format!("%{}%", trimmed)`, so a
//! product name containing `%` or `_` matched unintended candidates
//! (`%` → any-substring wildcard, `_` → any-single-char wildcard).
//! The keyword-search path in `transaction.rs` had already fixed
//! this with a private `escape_like_pattern`; extracting it here
//! and pairing every callsite with `LIKE ? ESCAPE '\'` keeps the
//! contract in one place for future search additions.
//!
//! Pair with `LIKE ? ESCAPE '\'` in the SQL — SQLite's default
//! LIKE has no escape character, so the query must declare `'\'`
//! explicitly for the escapes we emit here to be recognised.

/// Escape SQL LIKE metacharacters so user-supplied text matches
/// literally. Backslash is escaped first so we don't re-escape the
/// escapes we just added.
///
/// The returned string is meant to be wrapped in `%...%` (or `%...`,
/// `...%`) by the caller depending on match kind, then bound to a
/// `LIKE ? ESCAPE '\'` parameter.
pub fn escape_like_pattern(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_passes_through_unchanged() {
        assert_eq!(escape_like_pattern("apple"), "apple");
    }

    #[test]
    fn percent_is_escaped() {
        assert_eq!(escape_like_pattern("100%"), "100\\%");
    }

    #[test]
    fn underscore_is_escaped() {
        assert_eq!(escape_like_pattern("foo_bar"), "foo\\_bar");
    }

    #[test]
    fn backslash_is_escaped_first_then_metacharacters() {
        // Order matters: escaping % / _ after \ would produce
        // \\\% (backslash + escaped percent) — the naïve wrong
        // form would produce \% first and then \\% (double-
        // escaped-backslash + percent), breaking the match.
        assert_eq!(escape_like_pattern("a\\%b"), "a\\\\\\%b");
    }

    #[test]
    fn multiple_metacharacters_all_escaped() {
        assert_eq!(escape_like_pattern("50%_off"), "50\\%\\_off");
    }

    #[test]
    fn empty_input_yields_empty_output() {
        assert_eq!(escape_like_pattern(""), "");
    }

    #[test]
    fn japanese_text_with_percent_escapes_only_the_metacharacter() {
        // The exact pin scenario from Fable-5 #23: "果汁100%ジュース"
        // — the `%` between 100 and ジ must be escaped, everything
        // else must pass through unchanged so the SQL matches the
        // literal Japanese product name.
        assert_eq!(escape_like_pattern("果汁100%ジュース"), "果汁100\\%ジュース");
    }
}
