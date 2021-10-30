# Minecraft Chat Rust
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FGrizzlT%2FMinecraftChatRust.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FGrizzlT%2FMinecraftChatRust?ref=badge_shield)


[![Latest Version](https://img.shields.io/crates/v/mc_chat)](https://crates.io/crates/mc_chat)

This rust crate aims to provide an easy-to-use object hierarchy
to manage raw JSON-messages sent over the minecraft protocol.

Serialization and Deserialization is done using [serde](https://serde.rs) (such a nice library! ❤️).

Using a version indication, different styles are automatically ignored for older versions.

### Dependencies

To depend on `mc_chat`, use:

```toml
[dependencies]
mc_chat = "0.3"
```

To enable serialization/deserialization support, use:

```toml
[dependencies]
mc_chat = { version = "0.3", features = ["serde"] }
```

### Todo
- [x] Serialization/Deserialization to json possible.
- [ ] Complete missing elements.
- [ ] Add a 'legacy' text format.
- [ ] Better documentation (**examples**!).

### Contribution
Please feel free to contribute to this repository. Any help is welcome!

(I'm also pretty sure there are some people out there who know how to improve this crate by a bunch,
so any advice is appreciated!)


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FGrizzlT%2FMinecraftChatRust.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FGrizzlT%2FMinecraftChatRust?ref=badge_large)