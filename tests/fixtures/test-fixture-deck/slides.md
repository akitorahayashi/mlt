---
marp: true
theme: mlt-default
paginate: true
header: 'Testing Patterns'
footer: 'Rust Best Practices'
---

<!-- _class: title-slide -->

# Testing Patterns in Rust

<div class="subtitle">Practical strategies for reliable test suites</div>

---

## Unit Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_input_correctly() {
        let result = validate("valid-input");
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_invalid_input() {
        let result = validate("");
        assert!(result.is_err());
    }
}
```

---

## Integration Test Layout

- `tests/` directory alongside `src/`
- Tests have full public API access
- No internal implementation freezing
- Observable behavior validation only

---

## Temporary Files in Tests

```rust
use tempfile::TempDir;

#[test]
fn process_writes_output_file() {
    let temp = TempDir::new().expect("temp dir");
    let output = temp.path().join("output.txt");
    
    process_and_write(&output);
    assert!(output.exists());
}
```

---

## CSS Asset Validation

- Verify materialized CSS contains expected declarations
- Check that theme overrides are properly inlined
- Validate that shared theme assets load correctly

---

## Test Fixture Assets

![width:800px](./1600x900.png)

Test image demonstrates asset embedding in HTML export.

---

# Summary

- Write tests at appropriate boundaries
- Use fixtures for repeatable test scenarios
- Validate observable output, not implementation
- Keep test state isolated and temporary
