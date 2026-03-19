# Changelog

## 0.7.0 (2026-03-19)

### Features

- **terminal**: support unbounded rows and improved frame capture ([a958deb](https://github.com/urmzd/teasr/commit/a958deb1e04ef249bc274f5da3298e9da5bae182))
- **cli**: add fps and seconds override flags ([ac34f26](https://github.com/urmzd/teasr/commit/ac34f26c4fd0b1747742b79268dd38797239c2c8))
- **convert**: support configurable gif encoding ([43f4bdb](https://github.com/urmzd/teasr/commit/43f4bdb82adb04ade3b20e4bf93503b3902dd89a))
- **core**: introduce configurable output formats and frame timing ([7e22db5](https://github.com/urmzd/teasr/commit/7e22db5f201bbb659d421d3396991fb1b5633e34))
- **chrome-frame**: add image scaling to prevent oversized renders ([ca3747b](https://github.com/urmzd/teasr/commit/ca3747bc56a6b21c2497aa4b21c73e585c163ca9))
- add showme subcommand, show help on bare teasr invocation ([5079031](https://github.com/urmzd/teasr/commit/507903199d1af48836c01505a5767f73ad23ea22))
- add chrome-framed screen capture, resilient orchestrator, and CI headless support ([4f44da0](https://github.com/urmzd/teasr/commit/4f44da07519f28ee8d64ce67d3c594d47d7b06d0))
- add dynamic web capture with GIF support, shell login mode, and font fallback ([b0a4b61](https://github.com/urmzd/teasr/commit/b0a4b616b7d01b423a5feca1955d70d746cb4988))
- add native terminal session recording with animated GIF output ([bbbe996](https://github.com/urmzd/teasr/commit/bbbe9962d0cedf633937e4884354e896147cd13e))
- rewrite in Rust as teasr ([c976e54](https://github.com/urmzd/teasr/commit/c976e549017ee75e990e2276934f740596d700a8))
- add dogfooding setup with demo page, real config, and CI workflow ([83382b6](https://github.com/urmzd/teasr/commit/83382b646b90b503b7c6fbc0a01538c6193718cf))
- add video conversion, Ollama AI mode, and orchestrator ([13134ca](https://github.com/urmzd/teasr/commit/13134ca193d4716616bbf11e48e64fb1384a18c9))
- add terminal capture with styled ANSI rendering ([8712118](https://github.com/urmzd/teasr/commit/8712118ff96f128e3be7eb7321e00bcdcfec2963))
- add screen capture for desktop and native apps ([e1979e1](https://github.com/urmzd/teasr/commit/e1979e1722df4eef790688158f524a504574cf93))
- add web capture with Playwright and dev server lifecycle ([9c49de3](https://github.com/urmzd/teasr/commit/9c49de3cabc0b06fe6b80dac24637900fafda630))
- add core modules — types, config, logger, CLI, and public API ([bc364c3](https://github.com/urmzd/teasr/commit/bc364c30adc404745e4946648938a2775553f3cb))

### Bug Fixes

- **chrome**: use temp files instead of data URLs for HTML frames ([318d6b0](https://github.com/urmzd/teasr/commit/318d6b005baca805a2ea963c0c1cb7dbef681825))
- **ci**: fix asset upload by using nullglob for platform-specific files ([b3a1f02](https://github.com/urmzd/teasr/commit/b3a1f02692ecf6404dab4f6310eca06b997209ba))
- **ci**: use workspace version inheritance to prevent crate version mismatches ([5e6549c](https://github.com/urmzd/teasr/commit/5e6549cd38d14bd546d9f98d76ff8baa8a28bed6))
- **ci**: use bash shell for upload step to fix Windows compatibility ([5bd9d11](https://github.com/urmzd/teasr/commit/5bd9d1141ac5c951eb1d35217a4767b1d143d871))
- **ci**: add clang and libclang-dev for bindgen in cross image ([6792361](https://github.com/urmzd/teasr/commit/67923613334f0b1589d2d74bc63e0ac378a0a676))
- **ci**: remove hardcoded version from internal path dependencies ([01e75a7](https://github.com/urmzd/teasr/commit/01e75a7ae8c63bc1115cb8e992160bd6c3c4bb4b))
- **ci**: use Ubuntu 24.04 base for aarch64 cross image to fix GLIBC mismatch ([f510f5c](https://github.com/urmzd/teasr/commit/f510f5c29a77f97965c9d1c9f8a24b939ee9465b))
- **ci**: use cross main image (modern Ubuntu with all system deps) ([d773698](https://github.com/urmzd/teasr/commit/d77369820e56e5f6083ae1ac2a1d3f8ac592f1da))
- **ci**: use mesa dev packages for EGL/GL in cross image ([da4c269](https://github.com/urmzd/teasr/commit/da4c269278c1d6ad5820f510da58cf18819c411a))
- **ci**: use custom Dockerfile for aarch64 cross-compilation ([04784e7](https://github.com/urmzd/teasr/commit/04784e795950f8be140afef6a8eb7254848582d9))
- **ci**: add Cross.toml with system deps for aarch64 cross-compilation ([04f9ffb](https://github.com/urmzd/teasr/commit/04f9ffb16caa36800ec67c0a36dda8e85e77994b))
- **ci**: add libgbm-dev and libvulkan-dev to system dependencies ([8e916b3](https://github.com/urmzd/teasr/commit/8e916b30f178253602fbd9e67b40c481b6b52638))
- **ci**: add libegl-dev and libgl-dev to system dependencies ([04276a9](https://github.com/urmzd/teasr/commit/04276a984965ad21d61c7f2bfaf9833b8224dc4a))
- **ci**: add libpipewire-0.3-dev to system dependencies ([a782c01](https://github.com/urmzd/teasr/commit/a782c013f76c204df427ced853cf9790348ef6a6))
- add showcase assets to repo, update README with GIF demos, and fix CI ([425790c](https://github.com/urmzd/teasr/commit/425790c5c0999d0a08e1dbf5256045fb5c334bcd))
- **ci**: use semantic-release@v1 (v3 does not exist) ([13dc955](https://github.com/urmzd/teasr/commit/13dc9554dad53f0928862c5842baf64996946ac2))
- update demo page to reflect current Rust/TOML-based functionality ([17c137b](https://github.com/urmzd/teasr/commit/17c137b06ac21b6fd0fa583b944efffa98bc3887))
- update all references from urmzd/tease to urmzd/teasr ([24cb02b](https://github.com/urmzd/teasr/commit/24cb02be234a50f088ff66c50fd372e1f809d5aa))
- explicitly exit after successful capture ([c4b5d29](https://github.com/urmzd/teasr/commit/c4b5d298a1fd9918632ab9f62d86428f4d02935e))
- add global timeout and replace networkidle with load ([65bd849](https://github.com/urmzd/teasr/commit/65bd849d1bcada782a147b5556dc16193de85e6d))
- regenerate package-lock.json to include linux rollup binaries ([71556e5](https://github.com/urmzd/teasr/commit/71556e54f7e2b5458cae4d2f2fd24254e3d345f1))
- use scoped package name @urmzd/tease for npm publishing ([f94b219](https://github.com/urmzd/teasr/commit/f94b2199838cb7fcfd1d5b61933bd6f3ae5af4da))
- rename action dir strip-tease to tease, add npm trusted publishing ([355ec56](https://github.com/urmzd/teasr/commit/355ec5653253956510078d172ac4415b450f4dd4))

### Documentation

- **config**: update example config to new output format syntax ([600791f](https://github.com/urmzd/teasr/commit/600791f6d29dcdc00b11178ed6557c3f37f39c32))
- update cli help screenshots ([186ca03](https://github.com/urmzd/teasr/commit/186ca03f4786ad85612064d3ad49e71b71542349))
- document frame_duration for web scenes and chrome frame options for screen capture ([8b03c1f](https://github.com/urmzd/teasr/commit/8b03c1f8dca2d51c8ab75fcea97a047066d56325))
- **readme**: stack showcase images vertically instead of side-by-side ([36ae305](https://github.com/urmzd/teasr/commit/36ae3055bc88ffb51821bb90981b594472b03bb7))
- **showcase**: update CLI help and colorful modes demo assets ([a3653fd](https://github.com/urmzd/teasr/commit/a3653fd3f531591601f25e0c2ae7ba9d32acde61))
- add AGENTS.md project documentation ([1db099a](https://github.com/urmzd/teasr/commit/1db099a5fe069eb7cf0f06804c1703c40c7723c7))
- add README for root and each workspace crate ([976ce83](https://github.com/urmzd/teasr/commit/976ce834b76bd7a0357dcb275522222268cde7f5))
- add README, example config, GitHub Action, and CI workflow ([34106dc](https://github.com/urmzd/teasr/commit/34106dcd64f9eb5b65f424e192874f1786a3c5ca))

### Refactoring

- **install**: rename tmpdir to TMPDIR_CLEANUP for clarity ([8e0e15d](https://github.com/urmzd/teasr/commit/8e0e15dd195b8b68f6707a25e99a51605c92b407))

### Miscellaneous

- **assets**: update showcase materials ([39a2e5d](https://github.com/urmzd/teasr/commit/39a2e5db32a3ffc192a108b20bfb0fc4e8d80816))
- **deps**: bump teasr crates to 0.5.0 ([28eba35](https://github.com/urmzd/teasr/commit/28eba35e4b5039b25ffa8b727e215f93becc6f4c))
- update Cargo.lock ([3a217f7](https://github.com/urmzd/teasr/commit/3a217f7933875912f4dce6566e760e4da8288ce7))
- stack demo page cards vertically for simpler layout ([ac3c0e2](https://github.com/urmzd/teasr/commit/ac3c0e28a054005401fdfc11f3355fee8b5605fc))
- standardize project files and README header ([b6c9f9b](https://github.com/urmzd/teasr/commit/b6c9f9b6e1cfde641e1b59e7c2e607d90011c697))
- **skills**: add teasr-dev skill definition ([0fa31b1](https://github.com/urmzd/teasr/commit/0fa31b131ebbf7c33edf23a52ee09737610eeb06))
- add semantic-release config and fix crate publish metadata ([4101820](https://github.com/urmzd/teasr/commit/4101820725764576961ffa994171840b54d677ee))
- switch license from MIT to Apache-2.0 ([eb61337](https://github.com/urmzd/teasr/commit/eb6133761b414316546db820b4c63307b2c2696d))
- add project skeleton with build tooling ([a194f63](https://github.com/urmzd/teasr/commit/a194f635fcf6693671a901cb4ef6ae4f866f6c8e))
- initialize repository with .gitignore ([0c16d9c](https://github.com/urmzd/teasr/commit/0c16d9cb819276238f7986112889ef6491a6a940))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.6.0...v0.7.0)


## 0.5.0 (2026-03-19)

### Features

- **chrome-frame**: add image scaling to prevent oversized renders ([b7bd993](https://github.com/urmzd/teasr/commit/b7bd993df9936b0322ce92b220a1eafc3637610f))

### Documentation

- update cli help screenshots ([b6356dc](https://github.com/urmzd/teasr/commit/b6356dc1e249c4b5b46044e1a5acbef839fee154))

### Miscellaneous

- update Cargo.lock ([a89ed77](https://github.com/urmzd/teasr/commit/a89ed77ec7fb2591b1590d0c7310085fbbb011e2))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.4.0...v0.5.0)


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
