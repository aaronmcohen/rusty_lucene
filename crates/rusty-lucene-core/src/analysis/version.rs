#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub const LATEST: Self = Version {
        major: 8,
        minor: 10,
        patch: 0,
    };

    pub fn parse_leniently(self_version: &str) -> Self {
        let lowered = self_version.to_lowercase();
        if lowered == "latest" {
            return Self::LATEST;
        }
        let parts: Vec<String> = self_version
            .split('.')
            .map(|s| s.trim().to_string())
            .collect();
        let major = parts[0].parse().unwrap_or(8);
        let minor = parts[1].parse().unwrap_or(10);
        let patch = parts[2].parse().unwrap_or(0);
        Self {
            major,
            minor,
            patch,
        }
    }
}
