# xcodex

`xcodex` is a local fork of OpenAI Codex CLI for testing Tabby/xterm.js
terminal compatibility.

The current focus is preserving native terminal scrollback when Codex runs in
inline mode, such as:

```toml
[tui]
alternate_screen = "never"
```

or:

```sh
xcodex --no-alt-screen
```

## What Changed

In inline mode, finalized chat history is written using a scrollback-safe
newline path instead of `DECSTBM` scroll regions and reverse-index control
sequences. This keeps earlier conversation output reachable in xterm.js-based
terminals such as Tabby.

## Build

From the repository root:

```sh
(cd codex-rs && \
  OPENSSL_DIR=/usr \
  OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu \
  OPENSSL_INCLUDE_DIR=/usr/include \
  cargo build -p codex-cli --bin codex)
```

Install the local build as `xcodex`:

```sh
mkdir -p ~/.local/bin
cp codex-rs/target/debug/codex ~/.local/bin/xcodex
chmod +x ~/.local/bin/xcodex
```

## Run

```sh
xcodex --no-alt-screen
```

If `~/.codex/config.toml` already sets `alternate_screen = "never"`, run:

```sh
xcodex
```

## Upstream

This fork is based on OpenAI's Codex CLI:

https://github.com/openai/codex

Licensed under the Apache-2.0 License.
