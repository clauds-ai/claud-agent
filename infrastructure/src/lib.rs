pub mod auth;
pub mod dns;

pub use auth::{AuthToken, Claims, TokenError};
pub use dns::DnsResolver;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[tokio::test]
    async fn test_auth_service() {
        let auth = AuthToken::new("test_secret");
        let token = auth
            .create_token("test_subject", Duration::hours(1))
            .unwrap();
        let claims = auth.validate_token(&token).unwrap();
        assert_eq!(claims.sub, "test_subject");
    }

    #[tokio::test]
    async fn test_dns_resolver() {
        let resolver = DnsResolver::new().unwrap();
        let ips = resolver.resolve("example.com").await.unwrap();
        assert!(!ips.is_empty());
    }
}
