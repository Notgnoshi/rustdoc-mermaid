# Rust Mermaid

Using Mermaid.js with rustdoc

## Why not aquamarine?

I actually encourage folks to try [Aquamarine](https://github.com/mersinvald/aquamarine) first! I
think it's good enough for most use-cases.

I came up against the following limitations though:

* Compile time - Aquamarine's compile time is not abysmal, but in an already large project, I was
  sensitive to anything that made it worse
* Markdown files - Aquamarine doesn't support Mermaid blocks in markdown files included with
  `include_str!`.

  It _does_ support `include_mmd!`, but that is _only_ for separate diagrams, not long-form
  documentation in markdown files that includes diagrams.

## How?

`rustdoc` supports the `--html-in-header` and `--html-after-content` flags for injecting your own
HTML, so inject

```html
<script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
```

and

```html
<script>
mermaid.init({ startOnLoad: true, theme: "dark" }, "pre.language-mermaid > code");
</script>
```

## Workspaces

I found some sharp edges with Cargo workspaces (who'd have thunk it?):

* `cargo test`, `cargo tarpaulin`, and `cargo doc` all seemed to behave differently with respect to
  the CWD that `rustdoc` is invoked with.

  Workarounds:
  * Pre Rust 1.72, you could use `-Z doctest-in-workspace`
    (<https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#doctest-in-workspace>) to
    workaround this.
  * Running the doctests in the workspace root is now stabilized in Rust 1.72+
    (<https://doc.rust-lang.org/cargo/commands/cargo-test.html#working-directory-of-tests>)
* `cargo doc` will try to inject your HTML for each of your dependencies, and interprets the paths
  you pass it in the context of each crate it tries to build the docs for.

  Workarounds:
  * `cargo doc --no-deps` resolves this by not compiling the documentation for your dependencies

I tried real hard to find a way to use environment variables to set the path to the HTML fragments
to inject, but ultimately failed. My workaround is to add a `rustdoc` shim that auto-injects these
fragments with an absolute path

```toml
[build]
rustdoc = "./docs/rustdoc-workspace-shim.sh"
```

```sh
#!/bin/bash
REPO_ROOT="$(git rev-parse --show-toplevel)"

# Use absolute paths to the injected HTML to resolve differences in relative paths when rustdoc is
# invoked from _both_ the WORKSPACE _and_ the CRATE directories for each crate in the workspace, as
# well as crate dependencies when not using --no-deps
rustdoc \
    --html-in-header="$REPO_ROOT/docs/header.html" \
    --html-after-content="$REPO_ROOT/docs/after-content.html" \
    "$@"
```
