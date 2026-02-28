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

## License

MIT
