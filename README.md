# About
System bindings for RIOT OS generated by [bindgen]

[bindgen]: https://github.com/rust-lang-nursery/rust-bindgen.git

# How to add to X
## Board
To add a another board you must edit the `Cargo.toml` and the `config/board`
In the `Cargo.toml`, you must add the new board as an feature. The feature must match the board configuration in the `config/board`.

If you discover more common preprocessor configuration you can add them within `[all]`.

## Functionality
Modify the `whitelist_[function|var|type]` and add a new header for which bindings should be generated.
It will likely be the case, that after adding a new header, that some board modification must be extended with more preprocessor configuration.

