# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2024-09-12

### 📇 Features

- *(cargo)* Add dependency skim and webbrowser - ([6814b46](https://github.com/k3ii/revq/commit/6814b46a3a0872d9eb77bc598cdf20346fcd5236))
- *(cargo)* Add dependency Tokio and serde_json - ([8a9b053](https://github.com/k3ii/revq/commit/8a9b053414c2bbcb8d21e6adae45fb6cf709bf18))
- *(cargo)* Add dep serde_json - ([db61c75](https://github.com/k3ii/revq/commit/db61c757628a3cbafd882ab1f1621749ff1e526c))
- *(cargo)* Add dependency clap - ([4295ea8](https://github.com/k3ii/revq/commit/4295ea8b4d98b82e608b567a649151e3cd09c10c))
- *(cargo)* Add dependencies anyhow, directories, serde, toml - ([e2cbb02](https://github.com/k3ii/revq/commit/e2cbb02c6ad18dd8817f92b756e0e80dda5f10bb))
- *(cli)* Add author and About - ([18b841f](https://github.com/k3ii/revq/commit/18b841f9eb62b849f53172cf8f18ef145899da89))
- *(cli)* Add flag force to subcommand init - ([368d78c](https://github.com/k3ii/revq/commit/368d78c1bc2509027dda9637b83742296361d440))
- *(cli)* Make req and all mutually exclusive - ([3849246](https://github.com/k3ii/revq/commit/38492468e908f3458a5196018b8de67308fd1027))
- *(cli)* Add new flag --all for org - ([d527222](https://github.com/k3ii/revq/commit/d5272220795adfa0ca3a11346e22c8647b2f745a))
- *(cli)* Add subcommand init - ([60580b9](https://github.com/k3ii/revq/commit/60580b9e5a62b1d9c157b30ccc581ce1f2a6c0ce))
- *(cli)* Add aliases to flag req - ([8e307b0](https://github.com/k3ii/revq/commit/8e307b06e6f97d355fa0889eb97e712481d4fe4b))
- *(cli)* Add styling(color) to help and subcommands - ([05efefb](https://github.com/k3ii/revq/commit/05efefbdbe3bbc443d61704bb1f627696a682232))
- *(cli)* Add flag org and req - ([c4ce9e2](https://github.com/k3ii/revq/commit/c4ce9e24f84ef73eb145ebc303d83001c5f18173))
- *(cli)* Add cli.rs - ([823f9b7](https://github.com/k3ii/revq/commit/823f9b7306af1b703af9180364bafd6b2cf82e56))
- *(config)* Add org_setting - ([b6f44d8](https://github.com/k3ii/revq/commit/b6f44d8c53e1d1f49cf5e6766a9249c8a8b7003a))
- *(config)* Add config.rs - ([2898c39](https://github.com/k3ii/revq/commit/2898c39b057c1edccd9be3baa44c69aa52f2851d))
- *(github)* Add github.rs - ([35b4462](https://github.com/k3ii/revq/commit/35b4462df249605493620c0626a4b68a16d09037))
- *(init)* Make spinner blue - ([b4718a1](https://github.com/k3ii/revq/commit/b4718a13e836695b3ac362a0c11ab0d2bf02eece))
- *(init)* Add init.rs - ([c9015bc](https://github.com/k3ii/revq/commit/c9015bcc27ef94fa8d266129e5743ae659b34e6b))
- *(main)* Cater for flag all - ([eeabb1e](https://github.com/k3ii/revq/commit/eeabb1ebae2ddc6d04dc7b7be0f133a2d155bc5b))
- *(main)* Add fn init and exit(0) - ([86369a4](https://github.com/k3ii/revq/commit/86369a436f039fa4749918c0e434eb4154f12872))
- *(main)* Add org and req options - ([8cadd42](https://github.com/k3ii/revq/commit/8cadd425c1d08f1ee6b306485bc7bb22146e61ef))
- *(main)* Filter PRs and open via browser - ([a330fe2](https://github.com/k3ii/revq/commit/a330fe250842faa4507cad62c83dd95c4741a005))
- *(main)* Add Tokio runtime for async - ([7983727](https://github.com/k3ii/revq/commit/79837274864dbf01c8aeb5dcc3cb87acda07075b))
- *(main)* Printout fetched PRs - ([ad0ef5c](https://github.com/k3ii/revq/commit/ad0ef5c200383abdf514c47b2b538ff10ed0621a))
- *(main)* Add github auth - ([cf47ad1](https://github.com/k3ii/revq/commit/cf47ad1fa53b4566d979a08fd652eaecd3a81ab2))
- *(main)* Get username from cli and config - ([dbe2af5](https://github.com/k3ii/revq/commit/dbe2af5f7361607ea961674f2d478a970ed548eb))
- *(main)* Add mod query - ([5312677](https://github.com/k3ii/revq/commit/53126770055c842fda8ac35fe0d4c7a5f04302e7))
- *(main)* Add mod cli - ([b59020c](https://github.com/k3ii/revq/commit/b59020c98a10f6d64cdf6348594d724439fdcb4e))
- *(main)* Add mod config - ([fd60c40](https://github.com/k3ii/revq/commit/fd60c401b8afddbc1e490ebfb0d3eb68ebc46cfb))
- *(pr)* Add pr.rs - ([165f522](https://github.com/k3ii/revq/commit/165f5224f6da93fa52301059980f456b2828f8cf))
- *(query)* Add new query to show all PR in org - ([47f83a9](https://github.com/k3ii/revq/commit/47f83a934142f136d7526fd91ad8d9f88d40cd16))
- *(query)* Add base_query and conditionals for orgs,req and user - ([973d9c5](https://github.com/k3ii/revq/commit/973d9c5767f1ad40910f00c7db4ba71c30adf1ff))
- *(query)* Add query.rs - ([1ba7e39](https://github.com/k3ii/revq/commit/1ba7e3975e23b0f1ec2f741ed5dd5534ed23a6d0))

### 🐛 Bug Fixes

- *(cli)* Fix error in short("u") - the trait `IntoResettable<char>` is not implemented for `&str` - ([5a252e6](https://github.com/k3ii/revq/commit/5a252e61b9a2056de296fd82fafdbd3f337add82))
- *(github)* Add context to get proper error message - ([e74d433](https://github.com/k3ii/revq/commit/e74d43323d80669bc649886092e8ca6677fcb01e))
- *(init)* Rename struct field for organization_setting - ([e5e822a](https://github.com/k3ii/revq/commit/e5e822a96816e07dabf5aa4cf3d94dc637f25ad6))
- *(main)* Handle errors more gracefully - ([9bbf70b](https://github.com/k3ii/revq/commit/9bbf70bc522a550799410714689d71eedb79f7ff))
- *(query)* Fix syntax error in Graphql query - ([bbf189d](https://github.com/k3ii/revq/commit/bbf189d911a93fea627ea7c47443174df2927148))
- *(quwye)* Correct queries arguments - ([6a340d8](https://github.com/k3ii/revq/commit/6a340d8b71cbd2698b119d698ed7261e8d0f50c4))
- Settle path and file convention - ([8e7ecd0](https://github.com/k3ii/revq/commit/8e7ecd0707458fefad49c45d0ad5afe2b15b81d3))

### 🚜 Refactor

- *(config)* Put org under org_setting - ([a11dfbf](https://github.com/k3ii/revq/commit/a11dfbfc91be6b48eb3a88ad7af150b94693dcb5))
- *(init)* Add force argument to fn init - ([3302706](https://github.com/k3ii/revq/commit/3302706c13937bd75d11134ca243cbf93bae393c))
- *(init)* Remove function use save_to_xdg_config - ([c2f7695](https://github.com/k3ii/revq/commit/c2f7695f45f8b079c6be1ec25d559668875c0e85))
- *(init)* Make function show_spinner public - ([bfa6e5b](https://github.com/k3ii/revq/commit/bfa6e5bfa93d07a0a4cb4c9b97223792f5d0732e))
- *(init)* Move spinner into its own function - ([6141472](https://github.com/k3ii/revq/commit/61414727ad5c9ac033326dca7a57481989f02b0d))
- *(main)* Handle initialization of config file better - ([c3129f6](https://github.com/k3ii/revq/commit/c3129f6f24accc889e0b0498d34d755adeb3b7b0))
- *(main)* Handle the result returned by the init function - ([2f6ea21](https://github.com/k3ii/revq/commit/2f6ea21cd367896c1f3a1401cdd2f07d39e835d2))
- *(pr)* Update build_pr_list fn to return a result instead of panicking - ([7914b80](https://github.com/k3ii/revq/commit/7914b8052bf9bdd37655bce61ce38c25213c30f7))

### 📚 Documentation

- *(README)* Update README.md - ([059f11e](https://github.com/k3ii/revq/commit/059f11e641a047740b49b86bfc60dbe51d78cda4))
- Add README.md - ([970c74f](https://github.com/k3ii/revq/commit/970c74f63d05e13b3b1182770f649db6a8b61002))

### ⚙️ Miscellaneous Tasks

- *(cargo)* Add cargo info - ([6601ae5](https://github.com/k3ii/revq/commit/6601ae50bb8724153e9d0d62e1a0cc6b4c4ecc05))
- *(cargo)* Add dependency colored - ([4e4c353](https://github.com/k3ii/revq/commit/4e4c3531477fdd24c48568fa526e08c96d91a157))
- *(cargo)* Add feature color for clap - ([850dab7](https://github.com/k3ii/revq/commit/850dab73b86d496c4be2a381f9d4a65fd72967e4))
- *(cargo)* Add dependency inquire - ([a43299a](https://github.com/k3ii/revq/commit/a43299a76c2f3b9e7ec3412840191aef147ecdde))
- *(ci)* Add release-plz - ([8407ada](https://github.com/k3ii/revq/commit/8407adadcb30cd3c3decaab2c074084127e93332))
- *(ci)* Add cargo dist - ([7550968](https://github.com/k3ii/revq/commit/75509680a2f172f1095a5014cd8180709988e739))
- *(cliff)* Add cliff.toml - ([bc2b846](https://github.com/k3ii/revq/commit/bc2b846b052c893bbb1bd3f1624ea9c833b8890e))
- *(init)* Remove unused module - ([15a4ce9](https://github.com/k3ii/revq/commit/15a4ce9223157ff9cfbc0f4f2c03a46e1250160b))
- Add ISSUE_TEMPLATE - ([46fb810](https://github.com/k3ii/revq/commit/46fb810d9b846a548332e3137e96778b0da37bf3))
- Add LICENSE - ([b5ee864](https://github.com/k3ii/revq/commit/b5ee864b864836790fe814d6f900a8d192acb144))
- Add cliff.toml - ([88ce55e](https://github.com/k3ii/revq/commit/88ce55eccb0d055c07ec2e57f1cfed194743cbb2))
- Add CHANGELOG - ([da45663](https://github.com/k3ii/revq/commit/da456633cff56a17c91031ae2422435afc8a42a5))

## [unreleased]

### 📇 Features

- *(cli)* Add flag force to subcommand init - ([368d78c](https://github.com/k3ii/revq/commit/368d78c1bc2509027dda9637b83742296361d440))
- *(cli)* Make req and all mutually exclusive - ([3849246](https://github.com/k3ii/revq/commit/38492468e908f3458a5196018b8de67308fd1027))
- *(cli)* Add new flag --all for org - ([d527222](https://github.com/k3ii/revq/commit/d5272220795adfa0ca3a11346e22c8647b2f745a))
- *(cli)* Add subcommand init - ([60580b9](https://github.com/k3ii/revq/commit/60580b9e5a62b1d9c157b30ccc581ce1f2a6c0ce))
- *(cli)* Add aliases to flag req - ([8e307b0](https://github.com/k3ii/revq/commit/8e307b06e6f97d355fa0889eb97e712481d4fe4b))
- *(cli)* Add styling(color) to help and subcommands - ([05efefb](https://github.com/k3ii/revq/commit/05efefbdbe3bbc443d61704bb1f627696a682232))
- *(cli)* Add flag org and req - ([c4ce9e2](https://github.com/k3ii/revq/commit/c4ce9e24f84ef73eb145ebc303d83001c5f18173))
- *(cli)* Add cli.rs - ([823f9b7](https://github.com/k3ii/revq/commit/823f9b7306af1b703af9180364bafd6b2cf82e56))
- *(config)* Add org_setting - ([b6f44d8](https://github.com/k3ii/revq/commit/b6f44d8c53e1d1f49cf5e6766a9249c8a8b7003a))
- *(config)* Add config.rs - ([2898c39](https://github.com/k3ii/revq/commit/2898c39b057c1edccd9be3baa44c69aa52f2851d))
- *(github)* Add github.rs - ([35b4462](https://github.com/k3ii/revq/commit/35b4462df249605493620c0626a4b68a16d09037))
- *(init)* Make spinner blue - ([b4718a1](https://github.com/k3ii/revq/commit/b4718a13e836695b3ac362a0c11ab0d2bf02eece))
- *(init)* Add init.rs - ([c9015bc](https://github.com/k3ii/revq/commit/c9015bcc27ef94fa8d266129e5743ae659b34e6b))
- *(main)* Cater for flag all - ([eeabb1e](https://github.com/k3ii/revq/commit/eeabb1ebae2ddc6d04dc7b7be0f133a2d155bc5b))
- *(main)* Add fn init and exit(0) - ([86369a4](https://github.com/k3ii/revq/commit/86369a436f039fa4749918c0e434eb4154f12872))
- *(main)* Add org and req options - ([8cadd42](https://github.com/k3ii/revq/commit/8cadd425c1d08f1ee6b306485bc7bb22146e61ef))
- *(main)* Filter PRs and open via browser - ([a330fe2](https://github.com/k3ii/revq/commit/a330fe250842faa4507cad62c83dd95c4741a005))
- *(main)* Add Tokio runtime for async - ([7983727](https://github.com/k3ii/revq/commit/79837274864dbf01c8aeb5dcc3cb87acda07075b))
- *(main)* Printout fetched PRs - ([ad0ef5c](https://github.com/k3ii/revq/commit/ad0ef5c200383abdf514c47b2b538ff10ed0621a))
- *(main)* Add github auth - ([cf47ad1](https://github.com/k3ii/revq/commit/cf47ad1fa53b4566d979a08fd652eaecd3a81ab2))
- *(main)* Get username from cli and config - ([dbe2af5](https://github.com/k3ii/revq/commit/dbe2af5f7361607ea961674f2d478a970ed548eb))
- *(main)* Add mod query - ([5312677](https://github.com/k3ii/revq/commit/53126770055c842fda8ac35fe0d4c7a5f04302e7))
- *(main)* Add mod cli - ([b59020c](https://github.com/k3ii/revq/commit/b59020c98a10f6d64cdf6348594d724439fdcb4e))
- *(main)* Add mod config - ([fd60c40](https://github.com/k3ii/revq/commit/fd60c401b8afddbc1e490ebfb0d3eb68ebc46cfb))
- *(pr)* Add pr.rs - ([165f522](https://github.com/k3ii/revq/commit/165f5224f6da93fa52301059980f456b2828f8cf))
- *(query)* Add new query to show all PR in org - ([47f83a9](https://github.com/k3ii/revq/commit/47f83a934142f136d7526fd91ad8d9f88d40cd16))
- *(query)* Add base_query and conditionals for orgs,req and user - ([973d9c5](https://github.com/k3ii/revq/commit/973d9c5767f1ad40910f00c7db4ba71c30adf1ff))
- *(query)* Add query.rs - ([1ba7e39](https://github.com/k3ii/revq/commit/1ba7e3975e23b0f1ec2f741ed5dd5534ed23a6d0))

### 🐛 Bug Fixes

- *(cli)* Fix error in short("u") - the trait `IntoResettable<char>` is not implemented for `&str` - ([5a252e6](https://github.com/k3ii/revq/commit/5a252e61b9a2056de296fd82fafdbd3f337add82))
- *(github)* Add context to get proper error message - ([e74d433](https://github.com/k3ii/revq/commit/e74d43323d80669bc649886092e8ca6677fcb01e))
- *(init)* Rename struct field for organization_setting - ([e5e822a](https://github.com/k3ii/revq/commit/e5e822a96816e07dabf5aa4cf3d94dc637f25ad6))
- *(main)* Handle errors more gracefully - ([9bbf70b](https://github.com/k3ii/revq/commit/9bbf70bc522a550799410714689d71eedb79f7ff))
- *(query)* Fix syntax error in Graphql query - ([bbf189d](https://github.com/k3ii/revq/commit/bbf189d911a93fea627ea7c47443174df2927148))
- *(quwye)* Correct queries arguments - ([6a340d8](https://github.com/k3ii/revq/commit/6a340d8b71cbd2698b119d698ed7261e8d0f50c4))
- Settle path and file convention - ([8e7ecd0](https://github.com/k3ii/revq/commit/8e7ecd0707458fefad49c45d0ad5afe2b15b81d3))

### 🚜 Refactor

- *(config)* Put org under org_setting - ([a11dfbf](https://github.com/k3ii/revq/commit/a11dfbfc91be6b48eb3a88ad7af150b94693dcb5))
- *(init)* Add force argument to fn init - ([3302706](https://github.com/k3ii/revq/commit/3302706c13937bd75d11134ca243cbf93bae393c))
- *(init)* Remove function use save_to_xdg_config - ([c2f7695](https://github.com/k3ii/revq/commit/c2f7695f45f8b079c6be1ec25d559668875c0e85))
- *(init)* Make function show_spinner public - ([bfa6e5b](https://github.com/k3ii/revq/commit/bfa6e5bfa93d07a0a4cb4c9b97223792f5d0732e))
- *(init)* Move spinner into its own function - ([6141472](https://github.com/k3ii/revq/commit/61414727ad5c9ac033326dca7a57481989f02b0d))
- *(main)* Handle initialization of config file better - ([c3129f6](https://github.com/k3ii/revq/commit/c3129f6f24accc889e0b0498d34d755adeb3b7b0))
- *(main)* Handle the result returned by the init function - ([2f6ea21](https://github.com/k3ii/revq/commit/2f6ea21cd367896c1f3a1401cdd2f07d39e835d2))
- *(pr)* Update build_pr_list fn to return a result instead of panicking - ([7914b80](https://github.com/k3ii/revq/commit/7914b8052bf9bdd37655bce61ce38c25213c30f7))

### 📚 Documentation

- Add README.md - ([970c74f](https://github.com/k3ii/revq/commit/970c74f63d05e13b3b1182770f649db6a8b61002))
- Add CHANGELOG - ([de3e7b6](https://github.com/k3ii/revq/commit/de3e7b69e89456dd334c109b263fea26b9547e2b))

### ⚙️ Miscellaneous Tasks

- *(cargo)* Add dependency colored - ([4e4c353](https://github.com/k3ii/revq/commit/4e4c3531477fdd24c48568fa526e08c96d91a157))
- *(cargo)* Add feature color for clap - ([850dab7](https://github.com/k3ii/revq/commit/850dab73b86d496c4be2a381f9d4a65fd72967e4))
- *(cargo)* Add dependency inquire - ([a43299a](https://github.com/k3ii/revq/commit/a43299a76c2f3b9e7ec3412840191aef147ecdde))
- *(cargo)* Add dependency skim and webbrowser - ([6814b46](https://github.com/k3ii/revq/commit/6814b46a3a0872d9eb77bc598cdf20346fcd5236))
- *(cargo)* Add dependency Tokio and serde_json - ([8a9b053](https://github.com/k3ii/revq/commit/8a9b053414c2bbcb8d21e6adae45fb6cf709bf18))
- *(cargo)* Add dep serde_json - ([db61c75](https://github.com/k3ii/revq/commit/db61c757628a3cbafd882ab1f1621749ff1e526c))
- *(cargo)* Add dependency clap - ([4295ea8](https://github.com/k3ii/revq/commit/4295ea8b4d98b82e608b567a649151e3cd09c10c))
- *(cargo)* Add dependencies anyhow, directories, serde, toml - ([e2cbb02](https://github.com/k3ii/revq/commit/e2cbb02c6ad18dd8817f92b756e0e80dda5f10bb))
- *(cliff)* Add cliff.toml - ([d224f9b](https://github.com/k3ii/revq/commit/d224f9be8fcd8aa269e93cf2094b9d5825bb038d))
- *(init)* Remove unused module - ([15a4ce9](https://github.com/k3ii/revq/commit/15a4ce9223157ff9cfbc0f4f2c03a46e1250160b))

