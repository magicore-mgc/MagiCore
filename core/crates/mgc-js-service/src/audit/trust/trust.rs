// NPM tarball host trust policy. / Policy trust host tarball NPM.
pub fn is_tarball_url_trusted(tarball_url: &str, registry_url: &str) -> bool {
    let Ok(tarball_parsed) = url::Url::parse(tarball_url) else {
        return false;
    };

    let Ok(registry_parsed) = url::Url::parse(registry_url) else {
        return false;
    };

    let Some(tarball_host) = tarball_parsed.host_str() else {
        return false;
    };

    let Some(registry_host) = registry_parsed.host_str() else {
        return false;
    };

    // SEC-09: a loopback tarball is only trustworthy when the registry itself
    // is loopback. Otherwise a public packument could point `dist.tarball` at
    // a local service (or at an attacker's tunnel on 127.0.0.1) and the bytes
    // would be fetched without any host check.
    // (SEC-09: tarball loopback chỉ đáng tin khi registry cũng loopback. Nếu
    // không, một packument công khai có thể trỏ `dist.tarball` về service local
    // (hoặc tunnel của kẻ tấn công trên 127.0.0.1) và bytes sẽ được tải mà
    // không kiểm host.)
    let is_loopback = |host: &str| host == "127.0.0.1" || host == "localhost" || host == "::1";
    if is_loopback(tarball_host) {
        return is_loopback(registry_host);
    }

    if tarball_host == registry_host {
        return true;
    }

    if registry_host == "registry.npmjs.org" {
        // SEC-09: only the two canonical CDN hosts. A suffix match accepted
        // every `*.npmjs.org`, so a compromised DNS or a crafted subdomain was
        // treated as the public registry.
        // (SEC-09: chỉ hai host CDN chính thức. Khớp hậu tố chấp nhận mọi
        // `*.npmjs.org`, nên DNS bị chiếm hoặc subdomain dựng sẵn được coi như
        // registry công khai.)
        return tarball_host == "registry.npmjs.org" || tarball_host == "registry.yarnpkg.com";
    }

    if let Ok(allowed) = std::env::var("MAGICORE_WEB_ALLOWED_TARBALL_HOSTS") {
        let allowed_hosts: Vec<&str> = allowed
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if allowed_hosts.contains(&tarball_host) {
            return true;
        }
    }

    false
}

#[cfg(test)]
#[path = "test/trust.rs"]
mod tests;
