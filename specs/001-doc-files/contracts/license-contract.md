# Contract: LICENSE Specification

**Feature**: Documentation Files
**Contract File**: `contracts/license-contract.md`
**Last Updated**: 2026-11-21

## Purpose

Define the exact requirements for the LICENSE file. This contract ensures the project has a valid, appropriate open-source license that protects both the project maintainers and users.

## Contract Terms

### 1. File Identity
- **File Name**: `LICENSE` (or `LICENSE.md`)
- **Location**: Project root (`/`)
- **Format**: Plain text with license header
- **Encoding**: UTF-8

### 2. License Selection Criteria

#### 2.1 Approved Licenses (Tier 1)
The following licenses are approved for use:

| License | Type | Key Features | When to Use |
|---------|------|--------------|-------------|
| MIT | Permissive | Minimal restrictions, allows commercial use | Default for libraries, simple projects |
| Apache 2.0 | Permissive | Patent grant, trademark use, NOTICE file | When patent concerns exist |
| BSD 3-Clause | Permissive | Similar to MIT, with attribution clause | Alternative to MIT |
| ISC | Permissive | Similar to MIT, shorter | When minimal text preferred |
| CC0 1.0 | Public Domain | Waives all rights | For data, documentation only |

#### 2.2 License Selection Process
1. **Default**: MIT License (unless specific requirements dictate otherwise)
2. **Consider Apache 2.0 if**: 
   - Project has patent concerns
   - Corporate contribution is expected
   - Explicit patent grant is desired
3. **Consider GPL if**: 
   - Copyleft is required
   - All derivatives must be open
   - (Note: Not recommended for libraries)

### 3. Required Content

#### 3.1 Copyright Notice
- **Required**: Yes
- **Format**: 
  ```
  Copyright (c) {YEAR} {COPYRIGHT HOLDER}
  ```
- **Year**: Current year or range of years of contribution
- **Holder**: Individual name, organization name, or "the Rusty Lucene contributors"

#### 3.2 Permission Grant
The license MUST grant the following permissions:
- [ ] Use
- [ ] Copy
- [ ] Modify
- [ ] Merge
- [ ] Publish
- [ ] Distribute
- [ ] Sublicense
- [ ] Sell copies

#### 3.3 Condition Requirements
The license MUST require:
- [ ] Include copyright notice in all copies
- [ ] Include license and copyright notice in distribution

#### 3.4 Limitation Clauses
The license MUST include:
- [ ] Limitation of liability
- [ ] Disclaimer of warranty
- [ ] No trademark rights (unless explicitly granted)

### 4. Quality Gates


#### 4.1 Mandatory Checks
- [ ] Valid SPDX identifier recognized by SPDX.org
- [ ] Complete copyright notice present
- [ ] All permission grants present
- [ ] All condition requirements present
- [ ] All limitation clauses present
- [ ] License text is complete (no truncated sections)
- [ ] License matches the filename (LICENSE vs LICENSE.md consistency)

#### 4.2 Recommended Checks
- [ ] License file includes year and copyright holder
- [ ] License is the full text (not a summary)
- [ ] License matches project's intended use case
- [ ] No additional restrictions beyond the license text
- [ ] License file passes `licensecheck` or similar tool validation

### 5. Change Management


#### 5.1 When to Update License
- Project scope changes significantly
- New contributors require different licensing terms
- Legal requirements change
- Project moves to a different organization
- License becomes deprecated or problematic

#### 5.2 Update Process
1. Review new license for compatibility with existing code
2. Check for any code with conflicting licenses
3. Update LICENSE file with new text
4. Update LICENSE year/holder if needed
5. Add NOTICE file if required by new license (Apache 2.0)
6. Update README.md license badge
7. Update CHANGELOG.md with license change note
8. Obtain approval from all significant contributors

### 6. Validation Methods


#### 6.1 Automated Validation
```bash
# Check SPDX identifier validity
spdx-license-ids list | grep -i {license-name}

# Check license compatibility
licensecheck --json .

# Verify full license text
diff <(curl -s https://opensource.org/licenses/{license-id}) LICENSE
```

#### 6.2 Manual Validation
1. Read entire license text
2. Verify all required sections are present
3. Confirm copyright notice accuracy
4. Check for any unexpected additional terms

### 7. References


- [SPDX License List](https://spdx.org/licenses/)
- [Open Source Initiative](https://opensource.org/licenses)
- [Choose a License](https://choosealicense.com/)
- [TL;DR Legal](https://tldrlegal.com/)

### 8. Contract Validation


This contract is validated by:
- Manual review against SPDX license list
- Automated license compatibility checking
- Copyright notice verification
- License text completeness check

## Revision History


| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-11-21 | speckit-agent | Initial contract definition |