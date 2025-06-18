<div align="center">
 <!-- <img src=".github/assets/logo.png" alt="Discord Bot Logo" height="100px"> -->
 <h1>Discord Bot</h1>
</div>

<br/>

A Discord Bot written in Rust, configurable through YAML and supporting WASM plugins for enhanced functionality.

> [!NOTE]  
> We are still in active development working towards an initial stable release.

## Features

Work in Progress:

- **YAML Configuration:** Easily customize the bot's configuration using simple YAML files.
- **WASM Plugin Support:** Extend the bot's capabilities with custom WebAssembly plugins.

## Installation

All stable builds and the latest development build can be downloaded directly from the [releases page](https://github.com/paperback-community/discord-bot/releases). Make sure to select the build specific for your OS and chipset.

> [!NOTE]  
> You may need to build the application yourself if your OS or chipset is not listed.

### Stable Version

The stable build is released less frequently, typically after a thorough testing period. Because of this, it generally experiences fewer bugs and is recommended for production use.

### Development build

The development build is generated automatically after every push to the `master` branch. While it allows users to access the latest features and improvements, it is generally discouraged for production environments, as bugs can be introduced at any time.

> [!WARNING]  
> Only use this version if you are familiar with potential risks. We are not liable for any issues you may encounter.

## discouraged

```yaml
name: <config-name>
version: <discord-bot-major-version>

plugins:
  plugin_1:
    environment:
      VAR_1: "env_var_1"
      VAR_2: "env_var_2"
    settings:
      key1: "setting_value_1"
      key2:
        - "setting_value_2"
        - "setting_value_3"
      key3:
        subkey1: "setting_value_4"
        subkey2: "setting_value_5"
```

## Support Guidelines

The support guidelines can be found [here](https://github.com/paperback-community/discord-bot/blob/master/.github/SUPPORT.md).

## Contributing Guidelines

The contributing guidelines can be found [here](https://github.com/paperback-community/discord-bot/blob/master/.github/CONTRIBUTING.md).
