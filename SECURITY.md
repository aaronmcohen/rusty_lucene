# Security Policy

 ## Security Philosophy

 rusty_lucene is committed to maintaining the security and integrity of the project. We take security seriously and appreciate responsible disclosure of vulnerabilities.

 ## Reporting Process

 ### Security Issue Reporting

 If you discover a security vulnerability in rusty_lucene, please report it responsibly:

 **Email**: security@rusty-lucene.org

 Include in your report:
 - Description of the vulnerability
 - Steps to reproduce (if possible)
 - Affected versions (if known)
 - Any potential impact

 ### Acknowledgement Timeline

 - Initial acknowledgement: Within 24 hours
 - Investigation update: Within 72 hours
 - Resolution timeline: Communicated within 5 business days

 ## Security Updates

 ### Supported Versions

 We currently support the latest minor version and the previous minor version:

 - **v0.1.x** - Current stable release (supported)
 - **v0.0.x** - Previous releases (security fixes only)

 Older versions are considered end-of-life and no longer receive security updates.

 ### Security Fix Process

 When a security vulnerability is confirmed:
 1. We assess the severity and impact
 2. We develop and test a fix
 3. We coordinate disclosure with reporters (when possible)
 4. We release security patches and CVEs as needed

 ### Public Disclosure

 Security advisories are published on:
 - GitHub Security Advisories
 - Our project website
 - Mailing list announcements (for critical issues)

 ## Security Team

 - **Primary Contact**: security@rusty-lucene.org
 - **PGP Key**: Available upon request (for encrypted reports)
 - **Bug Bounty**: Not currently available (considering for future)

 ## Best Practices for Users

 To keep your usage of rusty_lucene secure:
 - Always use the latest supported version
 - Review release notes for security fixes
 - Report any suspicious behavior to security@rusty-lucene.org

 ## Security Fix Communication

 When a critical security vulnerability is discovered and fixed:

 1. **Immediate Notification**: All users on the security mailing list receive an early notification (if subscribed)
 2. **Coordinated Disclosure**: We work with the reporter to ensure responsible disclosure timing
 3. **Patch Release**: A new version with the fix is published to crates.io
 4. **Public Advisory**: Within 24 hours of patch release, we publish:
    - A security advisory on GitHub Security Advisories
    - An entry in the RustSec database (if applicable)
    - Update to the Security Policy document with CVE assignment (if applicable)
 5. **Follow-up**: We provide guidance on upgrading and may offer additional support for affected users

 **To stay informed**:
 - Watch the repository on GitHub
 - Subscribe to the security mailing list (contact security@rusty-lucene.org)
 - Monitor crates.io for new versions of rusty_lucene
