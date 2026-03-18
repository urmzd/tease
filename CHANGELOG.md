# Changelog

## 0.4.0 (2026-03-18)

### Features

- add showme subcommand, show help on bare teasr invocation ([7d4698e](https://github.com/urmzd/teasr/commit/7d4698e8cccd82a04fae2b4eae2b8f4044cad8b7))

### Documentation

- document frame_duration for web scenes and chrome frame options for screen capture ([bb15ef2](https://github.com/urmzd/teasr/commit/bb15ef22e60be2c699fa78938e3ae53411708bfd))

### Refactoring

- **install**: rename tmpdir to TMPDIR_CLEANUP for clarity ([3f030b0](https://github.com/urmzd/teasr/commit/3f030b0b07ac3389a509e564dc253a45a4bf2c20))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.3.0...v0.4.0)


## 0.3.0 (2026-03-17)

### Features

- add chrome-framed screen capture, resilient orchestrator, and CI headless support ([5a05307](https://github.com/urmzd/teasr/commit/5a053078cc491a65aae344358ac45709c86df5fb))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.2.2...v0.3.0)


## 0.2.2 (2026-03-17)

### Bug Fixes

- **ci**: fix asset upload by using nullglob for platform-specific files ([76f742f](https://github.com/urmzd/teasr/commit/76f742f77a4040bc3e31c1de0ac83ba1f0b1c254))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.2.1...v0.2.2)


## 0.2.1 (2026-03-17)

### Bug Fixes

- **ci**: use workspace version inheritance to prevent crate version mismatches ([258a1b0](https://github.com/urmzd/teasr/commit/258a1b006c0ebddf8687e56ae1175f66115b23dd))

### Documentation

- **readme**: stack showcase images vertically instead of side-by-side ([1732f07](https://github.com/urmzd/teasr/commit/1732f07b99597d8ea67d40069173b36a2afe74c9))
- **showcase**: update CLI help and colorful modes demo assets ([e015692](https://github.com/urmzd/teasr/commit/e015692a31a91512c1f377dd4ce63611cfe97eed))

### Miscellaneous

- stack demo page cards vertically for simpler layout ([c9f0c1c](https://github.com/urmzd/teasr/commit/c9f0c1c494ae7aaeb5950fef365e0ff91f0ad7c8))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.2.0...v0.2.1)


## 0.2.0 (2026-03-17)

### Features

- add dynamic web capture with GIF support, shell login mode, and font fallback ([53fe3e8](https://github.com/urmzd/teasr/commit/53fe3e8e408fb4a89385d843e4d56c77a113e317))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.1.12...v0.2.0)


## 0.1.12 (2026-03-17)

### Bug Fixes

- **ci**: use bash shell for upload step to fix Windows compatibility ([46dcc59](https://github.com/urmzd/teasr/commit/46dcc59d1e9c4d308ed20f852fec1f874f5871d0))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.1.11...v0.1.12)


## 0.1.11 (2026-03-16)

### Bug Fixes

- **ci**: add clang and libclang-dev for bindgen in cross image ([d68833f](https://github.com/urmzd/teasr/commit/d68833fd92c4a9905fd1c9348ca6b3087f9e4137))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.1.10...v0.1.11)


## 0.1.10 (2026-03-16)

### Bug Fixes

- **ci**: remove hardcoded version from internal path dependencies ([67c2712](https://github.com/urmzd/teasr/commit/67c271259ade77eb39d2c64816f322f091937009))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.1.9...v0.1.10)


## 0.1.9 (2026-03-16)

### Bug Fixes

- **ci**: use Ubuntu 24.04 base for aarch64 cross image to fix GLIBC mismatch ([2c7bcc9](https://github.com/urmzd/teasr/commit/2c7bcc9ea78f0e8f4a4178f27305e1bea2e15a6d))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.1.8...v0.1.9)



## 0.1.8 (2026-03-16)

### Bug Fixes

- **ci**: use cross main image (modern Ubuntu with all system deps) ([4a62419](https://github.com/urmzd/teasr/commit/4a624198b3b9c594018076f231ee25c4682fe03d))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.1.7...v0.1.8)
