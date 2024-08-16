# Other useful cargo stuff

You can login to your crates.io account like this
- cargo login KEY_GENERATED_FROM_CRATESIO

You can publish your crate like this
- cargo publish

You'll need to specify different metadata like this
``` toml
[package]
name = "package_name"
version = "1.0.1"
edition = "2021"
description = "whaever you want"
license = "MIT OR Apache-2.0"
```

To publish a new version you just need to bump the version number up. It uses semver, like npm. 

You can't delete a version when published, but you can deprecate one like this
- cargo yank --vers 1.0.1
- cargo yank --vers 1.0.1 --undo
