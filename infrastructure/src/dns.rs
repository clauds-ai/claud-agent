use hickory_resolver::Resolver; // Async Resolver
use std::net::IpAddr;

pub struct DnsResolver {
    resolver: Resolver, // Async resolver
}

impl DnsResolver {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let resolver = Resolver::from_system_conf()?;
        Ok(Self { resolver })
    }

    pub async fn resolve(
        &self,
        domain: &str,
    ) -> Result<Vec<IpAddr>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self.resolver.lookup_ip(domain)?; // Now async
        Ok(response.iter().map(|ip| ip.into()).collect())
    }
}
