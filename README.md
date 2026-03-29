# pwd-types

Core types for password management in Rust.

## Features

- **PasswordScore** - A score from 0-100 representing password strength
- **PasswordStrength** - Enum with levels: `NotEvaluated`, `WEAK`, `MEDIUM`, `STRONG`, `EPIC`, `GOD`
- **PasswordEvaluation** - Result containing score and evaluation reasons
- **PasswordStats** - Statistics for password analysis

### Optional Features

| Feature | Description |
|---------|-------------|
| `secrecy` (default) | `SecretString` and `SecretBox` wrappers for sensitive data |
| `sqlx` | Database types: `StoredPassword`, `UserAuth`, `DbSecretString` |
| `generator` | Password generator configuration types |

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
pwd-types = { git = "https://github.com/LucioPg/pwd-types" }
```

### Minimal (no default features)

```toml
[dependencies]
pwd-types = { git = "https://github.com/LucioPg/pwd-types", default-features = false }
```

### With SQLx support

```toml
[dependencies]
pwd-types = { git = "https://github.com/LucioPg/pwd-types", features = ["sqlx"] }
```

## Example

```rust
use pwd_types::{PasswordScore, PasswordStrength, PasswordEvaluation};

// Create a score
let score = PasswordScore::new(85);
println!("Score: {}", score); // "85"

// Get strength from score
let strength = PasswordScore::get_strength(Some(85));
assert_eq!(strength, PasswordStrength::EPIC);

// Strength thresholds:
// 96+  → GOD
// 85+  → EPIC
// 70+  → STRONG
// 50+  → MEDIUM
// 0-49 → WEAK
```

## License and Commercial Use

This project is licensed under the **Prosperity Public License 3.0.0**.

### What does this mean for you?

- **Personal and Non-Profit Use:** You are free to use, study, and modify this software at no cost for personal,
  educational, or research purposes.
- **Commercial Use:** If you are a company or a professional using this software for profit-making activities, you are
  granted a **30-day trial period**.

### How to Obtain a Commercial License

To continue using the software for commercial purposes after the 30-day trial, you must purchase a dedicated commercial
license.

To request a quote or activate your license, please contact:
**ldcproductions@proton.me**

*Please use the subject line: "Commercial License Request - pwd-types"*

---
*Note: All third-party open-source components remain
subject to their respective licenses.*