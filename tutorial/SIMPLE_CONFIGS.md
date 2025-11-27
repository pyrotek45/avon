# Simple Config File Examples

This guide demonstrates how to use Avon to generate common configuration files. These examples are perfect for beginners and show practical patterns you can adapt for your own needs.

## Why Use Avon for Config Files?

- **Variables**: Make configs reusable across machines and environments
- **Functions**: Generate repetitive sections automatically
- **Conditionals**: Adapt configs based on OS, environment, or preferences
- **Sharing with `--git`**: Keep one template in git, deploy customized versions everywhere with a single command
- **No dependencies**: Just the Avon binary, no runtime requirements

### The Power of `--git`: Share Configs Instantly

One of Avon's most powerful features is the `--git` flag, which lets you fetch and deploy configs directly from GitHub. This enables a powerful workflow:

**One template in git → Many customized deployments**

Instead of copying config files around or maintaining separate versions, you can:
1. Keep one `.av` template file in a GitHub repository
2. Anyone can deploy it with custom values using CLI arguments
3. Updates to the template are automatically available to everyone

**Example:**
```bash
# Deploy a vim config from GitHub with your custom settings
avon deploy --git user/repo/vimrc.av --root ~ -username alice -theme gruvbox

# Deploy the same config with different settings on another machine
avon deploy --git user/repo/vimrc.av --root ~ -username bob -theme solarized
```

This makes sharing dotfiles, team configs, and infrastructure templates incredibly easy. See the [Sharing Configs with `--git`](#sharing-configs-with-git) section below for detailed examples.

## Quick Start Pattern

The basic pattern for any config file:

```avon
# Define your variables
let user = "alice" in
let theme = "solarized" in

# Generate the config file
@.vimrc {"
  " Configuration for {user}
  set number
  colorscheme {theme}
"}
```

Deploy it:
```bash
avon deploy config.av --root ~
```

## Editor Configurations

### Vim/Neovim Configuration Examples

#### Basic Examples (No Plugins)

**vim_simple.av** - Minimal vim configuration with basic settings
```bash
avon eval examples/vim_simple.av
avon deploy examples/vim_simple.av --root ~ --force
```

Features:
- Line numbers
- Indentation settings
- Search configuration
- Basic keybindings
- Syntax highlighting

**neovim_simple.av** - Minimal neovim configuration without plugins
```bash
avon eval examples/neovim_simple.av
avon deploy examples/neovim_simple.av --root ~/.config/nvim --force
```

Features:
- Relative line numbers
- Mouse support
- Clipboard integration
- Modern keybindings
- Better defaults

**neovim_lua_simple.av** - Modern Neovim config using Lua
```bash
avon eval examples/neovim_lua_simple.av
avon deploy examples/neovim_lua_simple.av --root ~/.config/nvim --force
```

Features:
- Lua-based configuration
- Modern keybindings
- Auto-commands
- Better performance

#### With Plugins

**vim_plugins.av** - Vim config with vim-plug and popular plugins
```bash
avon eval examples/vim_plugins.av
avon deploy examples/vim_plugins.av --root ~ --force
```

Plugins included:
- vim-sensible (sane defaults)
- gruvbox (color scheme)
- NERDTree (file explorer)
- FZF (fuzzy finder)

#### Advanced Examples

**neovim_init.av** - Full-featured Neovim configuration
```bash
avon deploy examples/neovim_init.av --root ~/.config/nvim --force
```

Features:
- LSP support
- Treesitter integration
- Plugin management
- Comprehensive keybindings
- Advanced plugin configurations

**neovim_config_fn.av** / **neovim_config_gen.av** - Advanced generation examples showing function-based config generation

**emacs_init.av** - Full Emacs configuration generator
```bash
avon deploy examples/emacs_init.av --root ~ --force
```

Features:
- use-package integration
- LSP support
- org-mode configuration
- magit setup
- Modern Emacs patterns

### Example: Customizable Vim Config

Here's a pattern you can use for any editor config. **This example is perfect for sharing via `--git`:**

```avon
# Customizable vim configuration
# Share this in GitHub and deploy with: avon deploy --git user/repo/vim_config.av --root ~ -username alice -theme gruvbox

\username ? "developer" \theme ? "solarized" \tab_width ? "4" @.vimrc {"
  " Vim configuration for {username}
  
  " Appearance
  set number
  colorscheme {theme}
  
  " Indentation
  set tabstop={tab_width}
  set shiftwidth={tab_width}
  set expandtab
  
  " Search
  set hlsearch
  set incsearch
  set ignorecase
  set smartcase
  
  " Keybindings
  nnoremap <leader>w :w<CR>
  nnoremap <leader>q :q<CR>
"}
```

Deploy locally:
```bash
avon deploy vim_config.av --root ~ -username alice -theme gruvbox -tab_width 2
```

Or share via GitHub and deploy from anywhere:
```bash
# After pushing to GitHub
avon deploy --git user/repo/vim_config.av --root ~ -username alice -theme gruvbox -tab_width 2
```

## Shell Configuration Examples

### Bash Configuration

```avon
let prompt_color = "blue" in
let show_git_branch = "true" in

@.bashrc {"
  # Bash configuration
  
  # Colors
  export PS1='\\[\\033[{prompt_color}m\\]\\u@\\h:\\w\\$\\[\\033[0m\\] '
  
  # Aliases
  alias ll='ls -lah'
  alias la='ls -A'
  alias l='ls -CF'
  alias ..='cd ..'
  alias ...='cd ../..'
  
  # Git aliases
  alias gs='git status'
  alias ga='git add'
  alias gc='git commit'
  alias gp='git push'
"}
```

### Zsh Configuration

```avon
let theme = "robbyrussell" in
let plugins = ["git", "docker", "kubectl"] in

@.zshrc {"
  # Zsh configuration
  export ZSH="$HOME/.oh-my-zsh"
  
  ZSH_THEME="{theme}"
  
  plugins=({join plugins " "})
  
  source $ZSH/oh-my-zsh.sh
  
  # Custom aliases
  alias k='kubectl'
  alias d='docker'
"}
```

## Git Configuration

### Basic Git Config

```avon
let name = "Alice Developer" in
let email = "alice@example.com" in
let editor = "nvim" in

@.gitconfig {"
  [user]
    name = {name}
    email = {email}
  
  [core]
    editor = {editor}
    autocrlf = input
  
  [init]
    defaultBranch = main
  
  [alias]
    st = status
    co = checkout
    br = branch
    ci = commit
    unstage = reset HEAD --
    last = log -1 HEAD
"}
```

### Multi-Environment Git Config

```avon
let work_email = "alice@company.com" in
let personal_email = "alice@personal.com" in
let use_work = "true" in

let email = if use_work == "true" then work_email else personal_email in

@.gitconfig {"
  [user]
    name = Alice Developer
    email = {email}
  
  [core]
    editor = nvim
    autocrlf = input
  
  [alias]
    st = status
    co = checkout
    br = branch
"}
```

## SSH Configuration

```avon
let ssh_key_path = "~/.ssh/id_ed25519" in
let github_host = "github.com" in

@.ssh/config {"
  Host *
    AddKeysToAgent yes
    UseKeychain yes
    IdentityFile {ssh_key_path}
  
  Host {github_host}
    HostName {github_host}
    User git
    IdentityFile {ssh_key_path}
"}
```

## Environment Files (.env)

```avon
let env = "development" in
let db_host = if env == "production" then "db.prod.example.com" else "localhost" in
let db_port = "5432" in
let debug = if env == "production" then "false" else "true" in

@.env {"
  NODE_ENV={env}
  DATABASE_HOST={db_host}
  DATABASE_PORT={db_port}
  DEBUG={debug}
  LOG_LEVEL={if env == "production" then "warn" else "debug"}
"}
```

Deploy for different environments:
```bash
avon deploy env.av --root . -env production
avon deploy env.av --root . -env development
```

## Docker Compose

```avon
let app_name = "myapp" in
let db_version = "15" in
let redis_version = "7" in

@docker-compose.yml {"
  version: '3.8'
  
  services:
    app:
      build: .
      ports:
        - "3000:3000"
      environment:
        - NODE_ENV=production
      depends_on:
        - db
        - redis
    
    db:
      image: postgres:{db_version}
      environment:
        POSTGRES_DB: {app_name}
        POSTGRES_USER: postgres
        POSTGRES_PASSWORD: postgres
      volumes:
        - db_data:/var/lib/postgresql/data
    
    redis:
      image: redis:{redis_version}
      ports:
        - "6379:6379"
  
  volumes:
    db_data:
"}
```

## Nginx Configuration

```avon
let domain = "example.com" in
let upstream = "localhost:3000" in
let ssl_cert = "/etc/ssl/certs/{domain}.crt" in
let ssl_key = "/etc/ssl/private/{domain}.key" in

@nginx.conf {{"
  server {
    listen 80;
    server_name {domain};
    return 301 https://$server_name$request_uri;
  }
  
  server {
    listen 443 ssl http2;
    server_name {domain};
    
    ssl_certificate {{ ssl_cert }};
    ssl_certificate_key {{ ssl_key }};
    
    location / {
      proxy_pass http://{{ upstream }};
      proxy_set_header Host $host;
      proxy_set_header X-Real-IP $remote_addr;
    }
  }
"}}
```

## Systemd Service Files

```avon
let service_name = "myapp" in
let user = "appuser" in
let working_dir = "/opt/myapp" in
let exec_path = "/opt/myapp/bin/myapp" in

@/etc/systemd/system/{service_name}.service {"
  [Unit]
  Description=My Application
  After=network.target
  
  [Service]
  Type=simple
  User={user}
  WorkingDirectory={working_dir}
  ExecStart={exec_path}
  Restart=always
  RestartSec=10
  
  [Install]
  WantedBy=multi-user.target
"}
```

## Sharing Configs with `--git`

**This is one of Avon's most powerful features for config management.** The `--git` flag lets you fetch and deploy configs directly from GitHub, enabling easy sharing and centralized management.

### How It Works

1. **Put your config template in a GitHub repository**
2. **Anyone can deploy it** with custom values using CLI arguments
3. **Updates are automatic** - when you update the template, everyone gets the latest version

### Format

```bash
avon deploy --git user/repo/path/to/file.av --root <destination> [arguments]
```

The format is: `username/repository/path/to/file.av`

### Example: Shared Vim Config

**Template in GitHub (`vimrc.av`):**
```avon
\username ? "developer" \theme ? "solarized" \tab_width ? "4" @.vimrc {"
  " Vim configuration for {username}
  set number
  set expandtab
  set tabstop={tab_width}
  colorscheme {theme}
"}
```

**Usage:**
```bash
# Developer Alice deploys with her preferences
avon deploy --git user/repo/vimrc.av --root ~ -username alice -theme gruvbox -tab_width 2

# Developer Bob deploys with different preferences
avon deploy --git user/repo/vimrc.av --root ~ -username bob -theme solarized -tab_width 4

# Server deployment with minimal config
avon deploy --git user/repo/vimrc.av --root ~ -username admin -theme default
```

### Example: Team Git Config

**Template in GitHub (`gitconfig.av`):**
```avon
\name ? "Developer" \email ? "dev@company.com" @.gitconfig {"
  [user]
    name = {name}
    email = {email}
  
  [core]
    editor = nvim
    autocrlf = input
  
  [alias]
    st = status
    co = checkout
"}
```

**Usage:**
```bash
# Each team member deploys with their own name and email
avon deploy --git company/dotfiles/gitconfig.av --root ~ -name "Alice Developer" -email "alice@company.com"
avon deploy --git company/dotfiles/gitconfig.av --root ~ -name "Bob Developer" -email "bob@company.com"
```

### Example: Environment-Specific Configs

**Template in GitHub (`app_config.av`):**
```avon
\env ? "dev" \user ? "developer" @config-{env}.yml {"
  environment: {env}
  user: {user}
  debug: {if env == "prod" then "false" else "true"}
  log_level: {if env == "prod" then "warn" else "debug"}
"}
```

**Usage:**
```bash
# Development machine
avon deploy --git company/configs/app_config.av --root ~/.config/myapp -env dev -user alice

# Production server
avon deploy --git company/configs/app_config.av --root /etc/myapp -env prod -user service
```

### Benefits of Using `--git`

1. **Single source of truth**: One template file in git, many customized deployments
2. **Easy updates**: Update the template once, everyone can redeploy with latest changes
3. **No copying**: No need to copy config files between machines
4. **Version control**: All config templates are versioned in git
5. **Team collaboration**: Share configs easily across teams
6. **Customization**: Each deployment can have different values via CLI arguments

### Best Practices for `--git` Templates

1. **Use default parameters**: Make templates flexible with `?` syntax
2. **Document parameters**: Add comments explaining what each parameter does
3. **Test locally first**: Test your template with `avon eval` before pushing
4. **Use meaningful defaults**: Provide sensible defaults that work for most cases
5. **Version your templates**: Use git tags or branches for different versions

## Customizing Examples

All examples use top-level `let` bindings for easy customization. For most configs you will see something like:

```avon
let user = "your_username" in
let theme = "your_theme" in
let tab_width = "4" in
```

### Method 1: Edit the File Directly

Change the default values in the `let` bindings and re-run the example:

```avon
let user = "alice" in  # Changed from "your_username"
let theme = "gruvbox" in  # Changed from "your_theme"
```

### Method 2: Make a Copy

Copy an example to your own file and customize it there:

```bash
cp examples/vim_simple.av my_vim_config.av
# Edit my_vim_config.av
avon deploy my_vim_config.av --root ~
```

### Method 3: Use Function Parameters (Best for Sharing)

Use function parameters with defaults to make templates shareable via `--git`:

```avon
let make_config = \user ? "developer" \theme ? "default" @.vimrc {"
  " Config for {user}
  colorscheme {theme}
"} in
make_config
```

Deploy locally:
```bash
avon deploy config.av --root ~ -user alice -theme solarized
```

Or share via GitHub:
```bash
avon deploy --git user/repo/config.av --root ~ -user alice -theme solarized
```

See the [TUTORIAL.md](./TUTORIAL.md) CLI section for details on passing arguments.

## Deploying Configs

### Basic Deployment

To write configuration files to disk:

```bash
avon deploy examples/vim_simple.av --root ~/.config
```

### Deploy from GitHub (Recommended for Sharing)

**This is the recommended way to share and use configs:**

```bash
# Deploy a config from GitHub with custom values
avon deploy --git user/repo/vimrc.av --root ~/.config -username alice -theme gruvbox
```

Benefits:
- No need to download or clone repositories
- Always get the latest version
- Easy to share with team members
- Customize per deployment with CLI arguments

### Overwrite Existing Files

Use `--force` to overwrite existing files:

```bash
avon deploy examples/vim_simple.av --root ~/.config --force
# Or with --git
avon deploy --git user/repo/vimrc.av --root ~/.config --force -username alice
```

### Backup Before Overwriting

Use `--backup` to create a backup of existing files:

```bash
avon deploy examples/vim_simple.av --root ~/.config --backup
# Or with --git
avon deploy --git user/repo/vimrc.av --root ~/.config --backup -username alice
```

### Deploy to System Directories

For system-wide configs, use `sudo`:

```bash
sudo avon deploy nginx.av --root /etc/nginx --force
# Or from GitHub
sudo avon deploy --git user/repo/nginx.av --root /etc/nginx --force -domain example.com
```

**Important:** Always use `--root` to specify where files should be written. This prevents accidental writes to system directories.

## Common Patterns

### Pattern 1: OS-Specific Configuration

```avon
let os_type = os in
let config_path = if os_type == "macos" then "~/Library/Application Support" else "~/.config" in

@config.yml {"
  os: {os_type}
  config_path: {config_path}
"}
```

### Pattern 2: Environment-Based Configuration

```avon
let env = env_var_or "ENVIRONMENT" "development" in
let debug = if env == "production" then "false" else "true" in
let log_level = if env == "production" then "warn" else "debug" in

@config.yml {"
  environment: {env}
  debug: {debug}
  log_level: {log_level}
"}
```

### Pattern 3: Multiple Files from One Template

```avon
let environments = ["dev", "staging", "prod"] in
let make_config = \env @config-{env}.yml {"
  environment: {env}
  debug: {if env == "prod" then "false" else "true"}
"} in

map make_config environments
```

### Pattern 4: Conditional Sections

```avon
let enable_feature = "true" in
let feature_config = if enable_feature == "true" then {"
  feature:
    enabled: true
    timeout: 30
"} else "" in

@app.yml {"
  app:
    name: myapp
{feature_config}
"}
```

## Best Practices

1. **Use `--git` for sharing**: Keep templates in GitHub and deploy with `--git` flag for easy sharing and updates
2. **Use meaningful variable names**: `user` instead of `u`, `tab_width` instead of `tw`
3. **Provide sensible defaults**: Use `?` syntax for optional parameters, especially in templates shared via `--git`
4. **Document your configs**: Add comments explaining what each section does
5. **Test before deploying**: Use `avon eval` to preview output
6. **Use `--root` always**: Prevents accidental writes to wrong locations
7. **Version control your templates**: Keep `.av` files in git, not generated configs
8. **Share templates with `--git`**: One template in git, many customized deployments - this is a key workflow pattern

## Related Documentation

- **[TUTORIAL.md](./TUTORIAL.md)** - Complete Avon tutorial
- **[STYLE_GUIDE.md](./STYLE_GUIDE.md)** - Code style and formatting
- **[FEATURES.md](./FEATURES.md)** - Complete language reference
- **[examples/](../examples/)** - 92+ real-world examples

## Next Steps

- Explore the `examples/` directory for more complex examples
- Try combining multiple configs into one generator
- Use the REPL (`avon repl`) to experiment with templates
- Read the full [TUTORIAL.md](./TUTORIAL.md) for advanced patterns
