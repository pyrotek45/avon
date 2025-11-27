# Simple Config File Examples

This directory contains simple, beginner-friendly examples for generating common configuration files.

## Vim/Neovim Configuration Examples

### Basic Examples (No Plugins)

- **vim_simple.av** - Minimal vim configuration with basic settings
  ```bash
  avon eval examples/vim_simple.av
  ```
  Features: line numbers, indentation, search settings, basic keybindings

- **neovim_simple.av** - Minimal neovim configuration without plugins
  ```bash
  avon eval examples/neovim_simple.av
  ```
  Features: relative line numbers, mouse support, clipboard, modern keybindings

- **neovim_lua_simple.av** - Modern Neovim config using Lua
  ```bash
  avon eval examples/neovim_lua_simple.av
  ```
  Features: Lua-based config, modern keybindings, auto-commands

### With Plugins

- **vim_plugins.av** - Vim config with vim-plug and popular plugins
  ```bash
  avon eval examples/vim_plugins.av
  ```
  Plugins: vim-sensible, gruvbox theme, NERDTree, FZF

### Advanced Examples

- **neovim_init.av** - Full-featured Neovim configuration
  - LSP support, treesitter, plugin management
  - Comprehensive keybindings and plugin configurations

- **neovim_config_fn.av** / **neovim_config_gen.av** - Advanced generation examples

- **emacs_init.av** - Full Emacs configuration generator
  - use-package, LSP, org-mode, magit

## Customizing Examples

All examples use top-level `let` bindings for easy customization. For most configs you will see something like:

```avon
let user = "your_username" in
let theme = "your_theme" in
let tab_width = "4" in
```

You can:

- **Edit the file directly**: change the default values in the `let` bindings and re-run the example.
- **Make a copy**: copy an example to your own file (for example `my_vim_config.av`) and customize it there.

Some advanced examples may also use Avon function parameters with default values (for example `\user ? "developer"`). In those cases you can either edit the default in the file or override it from the CLI; see the tutorial’s CLI section for details on passing arguments.

## Deploying Configs

To actually write the configuration files to disk:

```bash
avon deploy examples/vim_simple.av --root ~/.config
```

Use `--force` to overwrite existing files:

```bash
avon deploy examples/vim_simple.av --root ~/.config --force
```
