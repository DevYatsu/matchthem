# `matchthem`

[<img alt="crates.io" src="https://img.shields.io/crates/v/matchthem?style=for-the-badge" height="25">](https://crates.io/crates/matchthem)
[<img alt="github" src="https://img.shields.io/badge/github-matchthem-blue?style=for-the-badge" height="25">](https://github.com/DevYatsu/matchthem)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/matchthem?style=for-the-badge" height="25">](https://docs.rs/matchthem)

A high performance, zero-copy URL router with multi-match support.

```rust
use matchthem::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();
    router.insert("/home", "Welcome!")?;
    router.insert("/users/{id}", "A User")?;

    let matched = router.at("/users/978")?;
    assert_eq!(matched.params.get("id"), Some("978"));
    assert_eq!(*matched.value, "A User");

    Ok(())
}
```

## About

matchthem is a fork of matchit, with all its performance and features — and one major addition: _Multi-Match Traversal_.

Instead of returning just the single most specific match, matchthem can return all matching routes for a given path.

While matchit returns the single most specific route that matches a path, matchthem adds two new methods:

### 🔍 Router::all_matches(&self, path: &str)

Returns all routes that match the given path. Each result includes both the matched value and any extracted parameters.

```rust
use matchthem::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();
    router.insert("/users/test", "Welcome!")?;
    router.insert("/users/{id}", "A User")?;

    let matches = router.all_matches("/users/test");
    for m in matches {
        println!("Matched value: {}", m.value);
    }

    Ok(())
}
```

> ![WARNING]
> Returned matches are not ordered by insertion. The order is determined by internal trie traversal.

Router exposes 2 more functions: all_matches, all_matches_mut.

### ✏️ Router::all_matches_mut(&mut self, path: &str)

Similar to all_matches, but provides mutable access to each matched value.

This is especially useful for collecting, modifying, or aggregating multiple matches in-place:

```rust
let mut router = Router::new();
router.insert("/{*baz}", vec!["a"]).unwrap();
router.insert("/foo/bar", vec!["b"]).unwrap();

for m in router.all_matches_mut("/foo/bar") {
    m.value.push("mutated");
}
```

## Credits

[See](https://github.com/ibraheemdev/matchit?tab=readme-ov-file#credits).
