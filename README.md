# AnixRun

This is an [anyrun](https://github.com/anyrun-org/anyrun) plugin to quickly search and run nixpkgs programs.

## Requirements

This plugin uses [nix-index](https://github.com/nix-community/nix-index) under the hood. You need to generate the index once before using this plugin:

```sh
nix run github:nix-community/nix-index#nix-index
```

## Configuration

```ron
// <Anyrun config dir>/nix.ron
Config(
  prefix: ":nix",
  max_entries: 3,
  // If true, will only match for exact executable name (regex is "^/bin/{query}$" instead of "/bin/{query}")
  exact_match: false,
  index_database_path: "$HOME/.cache/nix-index/files"
)
```
