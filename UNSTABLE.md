# Helix Unstable

The **helix** release cycle has been quite stalled for a while now, but contributors are actively working on bug fixes and feature implementation. The intent of this fork is to bring together a set of mature pull requests against the **helix** master branch.

## Philosophy

The most important merged feature is the *steel plugin system*; it is kept in sync with the helix master branch and referenced as *steel-master*. This allows building on *steel-master* and focusing on improvements and fixes not achievable by a steel plugin.

Any subsequent PRs are cloned into this repo, rebased onto the *steel-master* branch, and necessary conflict resolutions are applied. The final integration is done with sequential regular merges onto the *unstable-steel* branch.

The set of selected features is obviously opinionated, but a lot of these choices are shared with [other helix forks](#Other-Helix-Variants). Features are also picked for their minimal impact on the helix codebase to ease future syncs with the helix upstream master branch.

> **Note**:
> I'm not a helix maintainer nor affiliated in any way with the helix team; all credit goes to the helix authors. I'm just a user trying to keep my beloved editor alive. I'm also not an everyday Rust developer; I try my best to make minimal and safe edits. The *unstable-steel* branch hasn't gone through the whole CI pipeline, so it's currently in a "works on my machine" state. This could be fixed in future versions.
  

## Installation

There are no prebuilt releases available from this repo; **helix** should be built from sources. Ensure you have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

Quick install:

```sh
export PREFIX=~/.local
export HELIX_DEFAULT_RUNTIME=$PREFIX/lib/helix/runtime
cargo install --git https://github.com/gravures/helix helix-term --root $PREFIX --locked
```

This will build and install **helix** with the *steel plugin system* in your chosen prefix.
For detailed build instructions see [Building from source](book/src/building-from-source).

## Features

### Steel Plugin System

To use the plugin system you will also need the **steel** interpreter and dylib to run plugins, **forge** (the steel package manager) to install plugins, and the **steel language server** for plugin authoring.

```sh
cargo install --git https://github.com/mattwparas/steel.git steel-interpreter \
  steel-language-server cargo-steel-lib --root $PREFIX --locked --force
cargo install --git https://github.com/mattwparas/steel.git steel-forge \
  --root $PREFIX --locked --force
forge pkg refresh
```

Now you can follow the instructions in [Setting up configurations for helix](STEEL.md#setting-up-configurations-for-helix).

**Useful links**:

- [The steel language documentation](steel-docs.md)
- [Matthew Paras's helix steel libraries](https://github.com/mattwparas/helix-config)
- [awesome helix](https://github.com/npupko/awesome-helix#plugins)
- [helix editor plugins](https://helixplugins.xyz/)

### Treesitter Python Injection

Small feature of my own that adds *rst* syntax highlighting in Python docstrings.

### MDX Language Support

Adds *mdx* as a proper language definition with language servers set to ["mdx-language-server", "rumdl", "astro-ls"]. As a consequence *mdx* was removed from markdown language file-types.
Also adds treesitter queries for *mdx*.

```bash
npm install -g @mdx-js/language-server  
````

### Welcome Screen

When you launch helix with `hx`, you are now greeted with a pleasant welcome screen.

```toml
[editor]
welcome-screen = false
```

### Breadcrumb Navigation

A toggleable breadcrumb bar that shows where you are in your code (treesitter location).

```toml
[editor.breadcrumb]
enable = true
# full: helix-term > src > commands > typed.rs > TypeableCommand > name
# file: typed.rs > TypeableCommand > name
# none: TypeableCommand > name
path = "full|file|none"
```

### Clickable Buffer Line

Simply switch buffers by clicking on their names.

### Vertical Buffer

On narrow terminals, the picker and preview automatically stack vertically.

### Resizable Window Splits

**Window Resizing**:

- `Alt+w h` or `Alt+w left` - Shrink window width
- `Alt+w l` or `Alt+w right` - Grow window width  
- `Alt+w j` or `Alt+w down` - Shrink window height
- `Alt+w k` or `Alt+w up` - Grow window height

**Focus Mode**:

- `Alt+w f` - Toggle focus mode (expands current window)

**Sticky Mode**: activate sticky mode with `Alt+W` (Alt + Shift + w),
then use single keys for repeated resizing:

- `h` or `left` - Shrink width
- `l` or `right` - Grow width  
- `j` or `down` - Shrink height
- `k` or `up` - Grow height
- `f` - Toggle focus mode

Sticky mode stays active until you press a key that's not part of the window resize commands.

These *static commands* are added: *shrink_buffer_width*, *shrink_buffer_height*, *grow_buffer_width*, *grow_buffer_height*, and *toggle_focus_window*; so you can remap key bindings.

### Soft Wraps Indicators

These indicators are now displayed in the gutter.

### Icons Support

Icons are available to all pickers, the statusline, bufferline, and the completion list.

```toml
[icons.kind]
# show icons in autocompletion, symbols pickers
enable = true

[icons.fs]
# show icon in buffers-line and with status-line file-type and file-pickers
enable = true

[icons.vcs]
# visible in the changed-file picker
enable = true
branch = "⌬"

[icons.diagnostic]
warn = "▲"
```

> **Note on divergances from [PR#12369](https://github.com/helix-editor/helix/pull/12369):**
>
> - These config keys still work: `[editor.statusline] separator`,`[editor.whitespace] characters` (space, nbsp, nnbsp, tab, tabpad, newline) and `[editor.soft-wrap] wrap-indicator`. Existing configs need no migration
> - At this time `wrap-indicators` are only handled with the legacy key since beeing [displayed in the gutter](#Soft-Wraps-Indicators)
> - Precedence when both are set: **user-set `[icons.*]` > non-default legacy key > icon default**
> - Steel keeps writing the legacy keys, so `set-option!` / `get-config-option-value` work as before for whitespace, statusline separator and wrap-indicator — but the new `[icons.*]` keys can only be set in `config.toml`, not from steel
> - Steel engine is untouched, so PR descriptions of internals don't match this fork

## Roadmap

### Merged

- [x] Steel plugin system: [PR#8675](https://github.com/helix-editor/helix/pull/8675)
- [x] features/treesitter-python-injection
- [x] features/language-mdx
- [x] Welcome screen: [PR#15173](https://github.com/helix-editor/helix/pull/15173)
- [x] breadcrumb navigation bar: [PR#15573](https://github.com/helix-editor/helix/pull/15573)
- [x] clickable buffer line: [PR#12173](https://github.com/helix-editor/helix/pull/12173)
- [x] vertical-buffer: [PR#13430](https://github.com/helix-editor/helix/pull/13430)
- [x] resizable windows/focus mode: [PR#8546](https://github.com/helix-editor/helix/pull/8546)
- [x] basics-icons: [PR#12369](https://github.com/helix-editor/helix/pull/12369)
- [x] render soft-wrap indicators in the gutter: [PR#14825](https://github.com/helix-editor/helix/pull/14825)
- [x] scrollable file preview: [PR#15854](https://github.com/helix-editor/helix/pull/15854)
- [x] DAP: fix issue with debugpy corrupting terminal: [PR#15593](https://github.com/helix-editor/helix/pull/15593)

### Planned

- [ ] features/franky-theme
- [ ] set-option commands in keybinding sequences: [PR#15860](https://github.com/helix-editor/helix/pull/15860)

### Survey

- Multi-character auto-pairs support: [PR#15105](https://github.com/helix-editor/helix/pull/15105)
- match in next and previous char pairs: [PR#11695](https://github.com/helix-editor/helix/pull/11695)
- Code folding: [PR#14593](https://github.com/helix-editor/helix/pull/14593)

### Delegated

- [ ] move selected lines up and down: [PR#14121](https://github.com/helix-editor/helix/pull/14121)
  * PR is stalled with unresolved problematic behaviors when multiple cursors exist
  * adds two static commands already defined: move_line_up, move_line_down
- [ ] Inline Git Blame: [PR#13133](https://github.com/helix-editor/helix/pull/13133)
  * Maybe this could be implemented as a *steel* plugin

## Other Helix Variants

- [helix-unstable](https://github.com/Helix-Unstable/helix-unstable): exact same philosophy, currently a different, smaller feature set, and no steel integration yet
- [gj1118 fork](https://github.com/gj1118/helix): similar philosophy, more features, with a *lua* based plugin system
- [mitos editor](https://github.com/mitos-editor/mitos): a real fork of helix, has its own set of features and no plugin system to my knowledge
