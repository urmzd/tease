# Changelog

## 0.17.4 (2026-04-09)

### Bug Fixes

- **install**: download bare binary instead of non-existent tarball ([6b78e98](https://github.com/urmzd/teasr/commit/6b78e98658a307ccf94ff8a8590a0c1f96643552))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.17.3...v0.17.4)


## 0.17.3 (2026-04-09)

### Documentation

- **showcase**: update demo assets ([05f6f00](https://github.com/urmzd/teasr/commit/05f6f00a9d58ea90e108aee97dadf01e47abcdb3))

### Refactoring

- **chrome-frame**: remove image scaling constraint ([8044ec2](https://github.com/urmzd/teasr/commit/8044ec2d3c880aa7a90e439f3fa95fbacc7d3a3f))
- **terminal**: simplify grid viewport normalization ([a191707](https://github.com/urmzd/teasr/commit/a191707d424948b0d623ba0afef6a0edb42317bc))

### Miscellaneous

- **deps**: bump version to 0.17.1 ([5729993](https://github.com/urmzd/teasr/commit/57299931bdfb2ca631255bd028e267b7f1cf3d53))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.17.2...v0.17.3)


## 0.17.2 (2026-04-09)

### Bug Fixes

- **ci**: clean up cargo publish flags and checkout release tag ([1bc8e80](https://github.com/urmzd/teasr/commit/1bc8e802a7473ddacf86e6b06dd9b89d86ff09af))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.17.1...v0.17.2)


## 0.17.1 (2026-04-09)

### Bug Fixes

- **showcase**: replace flaky finder scene with self-contained markdown demo ([c15d872](https://github.com/urmzd/teasr/commit/c15d872d37666df604d0527b7b373e28d40eeb6a))

### Documentation

- **showcase**: update demonstration assets ([8fd18f2](https://github.com/urmzd/teasr/commit/8fd18f2e619c506f6397340f72b0b09038c68944))

### Miscellaneous

- **workflows**: trigger ci pipeline on main branch pushes ([563113f](https://github.com/urmzd/teasr/commit/563113f4d55f2777e941ce35e844691a83933007))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.17.0...v0.17.1)


## 0.17.0 (2026-04-09)

### Features

- **cli**: add self-update and version subcommands ([32bb7d8](https://github.com/urmzd/teasr/commit/32bb7d855c6fe4140e2d4698de5398274293eb42))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.16.0...v0.17.0)


## 0.16.0 (2026-04-08)

### Features

- **capture**: normalize grid for unbounded terminal emulators ([0f77d69](https://github.com/urmzd/teasr/commit/0f77d694382a5201f8fa39197a999d87c0de0ac0))
- **orchestrator**: wire vertical alignment through splash pipeline ([0e71a33](https://github.com/urmzd/teasr/commit/0e71a334b95916747a6f174b7395a877dbb8fde6))
- **render**: implement vertical alignment in splash and grid rendering ([d4c966f](https://github.com/urmzd/teasr/commit/d4c966fcd3c94632f9a987be225830234a2bfdc7))
- **types**: add vertical alignment support for splash content ([f7b13b5](https://github.com/urmzd/teasr/commit/f7b13b578fb9126554065fc5facf5aaba99cd221))

### Documentation

- add LICENSE to sub-crates for publishing compliance ([07899c4](https://github.com/urmzd/teasr/commit/07899c4d06cc636673c57ab82ea9948068fc8217))

### Miscellaneous

- **gitignore**: ignore .fastembed_cache ([0c4877a](https://github.com/urmzd/teasr/commit/0c4877a3bbd81713b7f3fd17defe0709c2554ea9))
- **deps**: update crate versions to 0.15.7 ([cc4faaa](https://github.com/urmzd/teasr/commit/cc4faaae0cd848cc887f02f06feb925e2806ee71))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.15.7...v0.16.0)


## 0.15.7 (2026-04-06)

### Bug Fixes

- **action**: hardcode public GitHub URLs for binary download ([56b7171](https://github.com/urmzd/teasr/commit/56b7171b08626e0e1c53a1b693b46129069b1a69))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.15.6...v0.15.7)


## 0.15.6 (2026-04-06)

### Refactoring

- simplify release tag resolution in action ([2500879](https://github.com/urmzd/teasr/commit/25008796278f01ea83c6b32feedccda8e3144c3b))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.15.5...v0.15.6)


## 0.15.5 (2026-04-05)

### Bug Fixes

- scope HTML linguist override to showcase/ only ([f01698a](https://github.com/urmzd/teasr/commit/f01698a31b7bcfcc88d3a443a8c032b9f0544865))

### Miscellaneous

- add linguist overrides to fix language stats (#11) ([23ad04e](https://github.com/urmzd/teasr/commit/23ad04e603f667a2651e96a682cc604bb99c2f95))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.15.4...v0.15.5)


## 0.15.4 (2026-04-02)

### Bug Fixes

- authenticate API curl requests for cross-org support ([0e05362](https://github.com/urmzd/teasr/commit/0e05362d45f9de1ab34258106c2a006caf3eb4d7))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.15.3...v0.15.4)


## 0.15.3 (2026-04-01)

### Bug Fixes

- **action**: handle floating tag resolution under pipefail ([844df6e](https://github.com/urmzd/teasr/commit/844df6ecc0fdb9a31a34f888716a051edb098fb2))
- **action**: remove auth from release API calls for cross-repo compatibility ([f476386](https://github.com/urmzd/teasr/commit/f476386ed853e6eed031398fc5593788246244ad))

### Refactoring

- normalize action.yml with floating tag resolution and consistent metadata ([74788e0](https://github.com/urmzd/teasr/commit/74788e006bd01f2b7887f70622d5bfabd7c811da))

### Miscellaneous

- add diagnostic logging to action.yml ([d7755f0](https://github.com/urmzd/teasr/commit/d7755f0298b0e62181086154f1665f11efc78895))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.15.2...v0.15.3)


## 0.15.2 (2026-03-30)

### Bug Fixes

- download bare binaries and verify sha256 checksum ([4778a6d](https://github.com/urmzd/teasr/commit/4778a6d2bc5dd4ba1638fa4deca31e2869430286))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.15.1...v0.15.2)


## 0.15.1 (2026-03-30)

### Bug Fixes

- add --no-verify to cargo publish to skip redundant rebuild ([2bebabd](https://github.com/urmzd/teasr/commit/2bebabdfdae32acfc36522810427c0485251935f))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.15.0...v0.15.1)


## 0.15.0 (2026-03-30)

### Features

- **capture**: add DOM-based activity tracking helpers ([6a8c478](https://github.com/urmzd/teasr/commit/6a8c478aa94560e807dee67713adba63b2dee02c))
- **capture**: add generic idle detection loop ([b83eada](https://github.com/urmzd/teasr/commit/b83eada390bc7794e845e88c597e7ad669001655))
- **browser**: introduce browser engine abstraction layer ([b6cf0f1](https://github.com/urmzd/teasr/commit/b6cf0f14a7d986824ed285d323b6287109cac424))
- **deps**: add serde_json dependency ([142d5cc](https://github.com/urmzd/teasr/commit/142d5cc0b414404ed002141e2f544fb9a5224048))

### Bug Fixes

- use action ref for binary download, deprecate version input ([2d2c8b5](https://github.com/urmzd/teasr/commit/2d2c8b5308e32acb83f793561f978a2d8243949f))
- **ci**: build before release to sync floating tag assets ([4dad59c](https://github.com/urmzd/teasr/commit/4dad59cc886de10730eae270fe028be4a0260e2f))
- use workspace version inheritance for all crates ([6590a00](https://github.com/urmzd/teasr/commit/6590a00e58a077365a61e9238a529e70b4e6564e))

### Documentation

- update sr action reference from v2 to v3 ([6c757eb](https://github.com/urmzd/teasr/commit/6c757ebc6067998995c9eba55152cf56de24ab48))

### Refactoring

- **chrome-frame**: migrate to browser abstraction ([ba3df66](https://github.com/urmzd/teasr/commit/ba3df669cf70e728e04905664ee6d61dc989c2c6))
- **screen-backend**: implement pixel-based idle detection ([b1cc62f](https://github.com/urmzd/teasr/commit/b1cc62f9f9fe8a674712f3fba70628daae5b393d))
- **web-backend**: migrate to browser abstraction ([7c2dc03](https://github.com/urmzd/teasr/commit/7c2dc032f31cc9d39b5cc708662b1f0a11ce9733))
- **file-backend**: migrate to browser abstraction ([c9d76a9](https://github.com/urmzd/teasr/commit/c9d76a9ae14e02c976d0419efbe0e52b6d20f577))

### Miscellaneous

- update sr action from v2 to v3 ([1e1323c](https://github.com/urmzd/teasr/commit/1e1323cf5165b5bc609fe0051c00a6af06481961))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.14.2...v0.15.0)


## 0.14.2 (2026-03-30)

### Performance

- add capture benchmark and smart wait defaults ([4baa8f9](https://github.com/urmzd/teasr/commit/4baa8f93f44f37a24edc69a3964a4dd29fe5c8de))
- optimize terminal capture pipeline ([1c1d7a1](https://github.com/urmzd/teasr/commit/1c1d7a1fd8c7e3271f20fa1bda8549dc8c341148))

### Documentation

- add crates.io badge to README ([333088e](https://github.com/urmzd/teasr/commit/333088e834e3b6797ac6bc1fadd7b10f61942223))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.14.1...v0.14.2)


## 0.14.1 (2026-03-29)

### Bug Fixes

- remove version pin from teasr-core path dependency in teasr-cli ([80acccb](https://github.com/urmzd/teasr/commit/80acccbb5d6e1a72170349ea003dceaf20a7b5c8))
- remove version pin from teasr-term-render path dependency ([f69b87e](https://github.com/urmzd/teasr/commit/f69b87e6c71929313a3cb7d344e8fe74aa004060))

### Refactoring

- rename teasr-cli to teasr and add publish pipeline ([29a1617](https://github.com/urmzd/teasr/commit/29a1617200c8caa82bc1d4a584b25e3be73940c4))

### Miscellaneous

- standardize CI/CD — add refactor bump, CI gate, workflow_dispatch ([6231bc3](https://github.com/urmzd/teasr/commit/6231bc32ef4986969354119e810585e7982e57f6))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.14.0...v0.14.1)


## 0.14.0 (2026-03-29)

### Features

- **config**: add markdown scene to teasr configuration ([67f7c4c](https://github.com/urmzd/teasr/commit/67f7c4c42a16dc12b815314ede58c6e9601c767f))

### Documentation

- **readme**: add markdown capture showcase section ([daec81e](https://github.com/urmzd/teasr/commit/daec81ec11ce263f62f57984485fb2321ebc95fa))
- **showcase**: refresh showcase assets and add markdown demo ([1b97f97](https://github.com/urmzd/teasr/commit/1b97f97ae4788e01e5675a6b8579c37e55bbaa61))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.13.0...v0.14.0)


## 0.13.0 (2026-03-29)

### Features

- **orchestrator**: integrate markdown scenes and manage temp files ([1fc7ad6](https://github.com/urmzd/teasr/commit/1fc7ad607ee844fa457667e578fd97ad69ca42c0))
- **capture**: enhance backends for markdown and idle timeout ([d888113](https://github.com/urmzd/teasr/commit/d888113872675384bff14bde0b1111930d147c42))
- **render**: implement markdown rendering with HTML templates ([88d177e](https://github.com/urmzd/teasr/commit/88d177e869d9f24f45675c263d5e8c56201b9f0f))
- **types**: add markdown scene and idle timeout support ([85b761f](https://github.com/urmzd/teasr/commit/85b761f15a7ab11b6b04ac13c17f476053e122f0))

### Documentation

- **readme**: document markdown scene type ([f5daecb](https://github.com/urmzd/teasr/commit/f5daecb08f59054ff576fbe323a28f7ff851c667))
- update README ([bd09b63](https://github.com/urmzd/teasr/commit/bd09b63f0574afba6b9f834af94525be8172c958))
- **skills**: align SKILL.md with agentskills.io spec ([64730a0](https://github.com/urmzd/teasr/commit/64730a09c1e6fa179d993a339580c5b1401c1b05))
- update documentation for hidden flag, file capture, and version bumps ([3a84e75](https://github.com/urmzd/teasr/commit/3a84e755cab4009821bfed2958f9a1218ac5f322))

### Refactoring

- extract chrome frame inline HTML into dedicated template file ([13f86e7](https://github.com/urmzd/teasr/commit/13f86e7dc2413708b4cb9c60ba7956d0b3907f9e))

### Miscellaneous

- **deps**: add comrak for markdown rendering ([4bdcfaa](https://github.com/urmzd/teasr/commit/4bdcfaa688285034f23ce7b61354660c5481031a))
- rename action for GitHub Marketplace publishing ([43ed988](https://github.com/urmzd/teasr/commit/43ed98869882d3bcd7cc1a413cbcc050c3bdb485))
- use sr-releaser GitHub App for release workflow (#1) ([9ead7e3](https://github.com/urmzd/teasr/commit/9ead7e3ebf83ed597d269bb97657e36f7c626ebd))
- update semantic-release action to sr@v2 ([c422534](https://github.com/urmzd/teasr/commit/c422534ac60cfc2a5599b642a1fc771ff9d3ca84))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.12.0...v0.13.0)


## 0.12.0 (2026-03-23)

### Features

- **terminal**: improve capture environment isolation ([4b4772b](https://github.com/urmzd/teasr/commit/4b4772b8b7c1e3c1c23726968202a71cf3af7a28))
- **orchestrator**: respect hidden flag when capturing scene ([a3b3ee0](https://github.com/urmzd/teasr/commit/a3b3ee0e73e58a41f979095de7da7db33016480f))
- **types**: add InteractionStep with hidden flag ([f44e7df](https://github.com/urmzd/teasr/commit/f44e7dfa0e09cc378e88931e546bfe47afb1df6e))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.11.2...v0.12.0)


## 0.11.2 (2026-03-23)

### Bug Fixes

- **showcase**: use height instead of min-height for reliable centering ([d42c9ee](https://github.com/urmzd/teasr/commit/d42c9ee66295844ec5cde168d30a44d2060b834a))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.11.1...v0.11.2)


## 0.11.1 (2026-03-23)

### Bug Fixes

- **capture**: always cd into cwd with sensible default ([708b69f](https://github.com/urmzd/teasr/commit/708b69f7c772e0cd718010ae80f53944427ff81f))
- **terminal**: default font to monospace for universal availability ([a32eab6](https://github.com/urmzd/teasr/commit/a32eab689732a088248ea8bc5f7c300042ad864f))
- **file**: hide PDF viewer UI in screenshots ([62c8921](https://github.com/urmzd/teasr/commit/62c8921eb26c121239e91308151b0e715ce1a6f5))

### Documentation

- update showcase media assets and add file demo ([b24bbc3](https://github.com/urmzd/teasr/commit/b24bbc328195e57ec695837be1f072083e90c514))

### Miscellaneous

- **release**: bump versions to 0.11.0 ([268fda3](https://github.com/urmzd/teasr/commit/268fda31ee383f8f6cf86940458e79d2bb1c2d7c))
- **build**: install cli directly from local path ([c96b3f2](https://github.com/urmzd/teasr/commit/c96b3f2733ce7c3f5b472ad95fe0347ddc072ceb))
- **justfile**: add Justfile with standardized recipes ([860c404](https://github.com/urmzd/teasr/commit/860c4045c76073d697f145aad3419411a370a5ec))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.11.0...v0.11.1)


## 0.11.0 (2026-03-22)

### Features

- **orchestrator**: integrate file capture backend ([734b744](https://github.com/urmzd/teasr/commit/734b74487c2c35427578aaf2a79c6db59302f044))
- **capture**: add file backend for local file rendering ([3fabc7f](https://github.com/urmzd/teasr/commit/3fabc7fcafa565a8f71b7dcf036deb61474e7c2c))
- **types**: add File scene configuration variant ([f8297b5](https://github.com/urmzd/teasr/commit/f8297b58c99fca26fe4e4fa21928ad8f4b219575))

### Miscellaneous

- **hooks**: add sr commit message validation hooks ([2c446ef](https://github.com/urmzd/teasr/commit/2c446efeb588b00a0a54e04c900b83c8a2a1dc71))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.10.2...v0.11.0)


## 0.10.2 (2026-03-22)

### Performance

- **render**: reuse Arc<fontdb> instead of cloning entire font database per frame ([13b34ba](https://github.com/urmzd/teasr/commit/13b34baa2d6afc127f08b6aada4a80992707bff9))

### Documentation

- **showcase**: regenerate showcase assets ([e5b0cf7](https://github.com/urmzd/teasr/commit/e5b0cf7d166718f4efc8db2834d6ab0163a65130))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.10.1...v0.10.2)


## 0.10.1 (2026-03-21)

### Bug Fixes

- **orchestrator**: change full_page default to false ([6d316ed](https://github.com/urmzd/teasr/commit/6d316ed32a3210facf98fab0e097c96d7a75d62b))
- **capture**: remove unused sleep in web capture ([097ea5a](https://github.com/urmzd/teasr/commit/097ea5a9a3c6f549a49c72f42b43e800b093e6f4))

### Documentation

- **showcase**: regenerate showcase assets with new timings ([3663cf2](https://github.com/urmzd/teasr/commit/3663cf2bc34b47e52c40ad23a8fc7969032c151e))

### Refactoring

- **svg**: use grid.dimensions() helper method ([1638c7e](https://github.com/urmzd/teasr/commit/1638c7e979b8aaffa0075a39a663294bdfbe54b6))

### Miscellaneous

- **config**: update showcase scene configuration ([7c44a57](https://github.com/urmzd/teasr/commit/7c44a5765df6c41c3af7d5a7a615afc67461790d))
- **deps**: bump version to 0.9.0 ([2069773](https://github.com/urmzd/teasr/commit/206977319f31eaf85f619d19d875e34c77efce75))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.10.0...v0.10.1)


## 0.10.0 (2026-03-21)

### Features

- **cli**: add --scenes filter to showme command ([438aaaa](https://github.com/urmzd/teasr/commit/438aaaae1cd554bd7e5f9308f9ae93b3018e0548))
- **orchestrator**: use UI module for progress and status ([9ae2491](https://github.com/urmzd/teasr/commit/9ae249187623c18abbc622a64f52a3e6fd706fc3))
- **setup**: add Monaspace Nerd Font and use UI module ([c1fce25](https://github.com/urmzd/teasr/commit/c1fce25224e541da395cea533b42cd15198d204f))
- **core**: create UI module for formatted terminal output ([6aea93a](https://github.com/urmzd/teasr/commit/6aea93a2eaefbc86524c37d95fb5a3aacfa4cce9))

### Documentation

- update README and AGENTS with new showcase configuration ([ff536c3](https://github.com/urmzd/teasr/commit/ff536c30fee0adc5b3bb109b4052c6ef2421b328))

### Miscellaneous

- update showcase assets and remove old demo files ([d153ce1](https://github.com/urmzd/teasr/commit/d153ce1af35b2819ba0cb6d6060c95cadd2a95ab))
- add font setup and scene filtering to GitHub Actions ([5a3d3fb](https://github.com/urmzd/teasr/commit/5a3d3fbda3187e7cfcd561f23155cf4aca6ddf25))
- **deps**: add crossterm and indicatif for terminal UI ([ac916f0](https://github.com/urmzd/teasr/commit/ac916f012addf10db39a968264822c915c7e6cce))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.9.0...v0.10.0)


## 0.9.0 (2026-03-21)

### Features

- **cli**: add setup command for font installation and checking ([48b1726](https://github.com/urmzd/teasr/commit/48b1726b28626c7f9f4d842a1e28335dfd3efd20))
- **core**: integrate splash screens and custom fonts ([5bb5003](https://github.com/urmzd/teasr/commit/5bb5003c6a454d82ebc670f453e0e24ef873b49f))
- **capture**: add cwd and font options to terminal backend ([35e2483](https://github.com/urmzd/teasr/commit/35e2483a212cbea6b1a2647790dd3c12d6eb64c9))
- **render**: make fonts configurable in rendering ([689fe34](https://github.com/urmzd/teasr/commit/689fe344974f7192accf40e112f631ab2591d425))
- **render**: add splash screen rendering module ([8376616](https://github.com/urmzd/teasr/commit/837661673ab45e2a5bc0021fc45574bb7fd4c3bf))
- **core**: add font setup and management module ([f01eb50](https://github.com/urmzd/teasr/commit/f01eb507a975021e21b7d720a5493224299ff933))
- **core**: add font configuration to types system ([ed771c7](https://github.com/urmzd/teasr/commit/ed771c72a123616c39bb6e51200fde3afd7d1174))

### Bug Fixes

- **convert**: handle variable-sized frames in GIF encoding ([5f52af0](https://github.com/urmzd/teasr/commit/5f52af07de151d60bdedd359335e9c0934be416f))

### Documentation

- align docs with actual code and simplify action ([57f69d5](https://github.com/urmzd/teasr/commit/57f69d591d631d4cca70a49647024c9720104e89))

### Miscellaneous

- **deps**: remove embedded fonts and add zip/image dependencies ([5f227bb](https://github.com/urmzd/teasr/commit/5f227bb462553aba34fe92048404fc1c19fa72d7))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.8.0...v0.9.0)


## 0.8.0 (2026-03-20)

### Features

- **capture**: warn when capturing entire monitor ([275ee41](https://github.com/urmzd/teasr/commit/275ee412ca777d27113a54fb8e52362fbdf96f24))
- **config**: add per-scene wall-clock timeout configuration ([b84ef1e](https://github.com/urmzd/teasr/commit/b84ef1e81d5cf89854f424e5a7ca5b45d37ae11a))

### Miscellaneous

- **showcase**: add desktop demo and update guides ([bfa373a](https://github.com/urmzd/teasr/commit/bfa373a17e69131164e2b8cdfe7f46afc4e04570))
- **config**: add test for expanded output formats ([1fc2c66](https://github.com/urmzd/teasr/commit/1fc2c666e97a1532efff0b6a409b5d3424ad35ff))
- upgrade dependencies to latest versions ([d6672c3](https://github.com/urmzd/teasr/commit/d6672c312b9de6265532b7e52c1363b42f79b320))

[Full Changelog](https://github.com/urmzd/teasr/compare/v0.7.0...v0.8.0)


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
