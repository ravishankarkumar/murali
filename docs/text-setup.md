# Text Setup

This document describes how to set up Murali's text backends.

The LaTeX and Typst paths are both wired into the renderer now.
LaTeX depends on external tools, while Typst uses an embedded in-process backend.

## Text Backends

Murali currently has three text paths:

- regular label text
- LaTeX text
- Typst text

### Regular Label Text

This is Murali's built-in text path for labels, UI-like captions, titles, and simple scene annotations.

It does not require external tools.

Relevant code:

- [font.rs](/Users/ravishankar/personal-work/murali/src/resource/text/font.rs)
- [layout.rs](/Users/ravishankar/personal-work/murali/src/resource/text/layout.rs)
- [atlas.rs](/Users/ravishankar/personal-work/murali/src/resource/text/atlas.rs)
- [mesh.rs](/Users/ravishankar/personal-work/murali/src/resource/text/mesh.rs)

### LaTeX Text

Murali renders LaTeX by:

1. generating a temporary `.tex` file
2. compiling it to `.dvi` with `latex`
3. converting `.dvi` to `.svg` with `dvisvgm`
4. rasterizing the SVG
5. uploading the result as a textured quad

Relevant code:

- [backend.rs](/Users/ravishankar/personal-work/murali/src/resource/latex_resource/backend.rs)
- [raster.rs](/Users/ravishankar/personal-work/murali/src/resource/latex_resource/raster.rs)
- [sync.rs](/Users/ravishankar/personal-work/murali/src/backend/sync.rs)

### Typst Text

Murali also has an embedded Typst rendering path.

Relevant code:

- [compiler.rs](/Users/ravishankar/personal-work/murali/src/resource/typst_resource/compiler.rs)
- [raster.rs](/Users/ravishankar/personal-work/murali/src/resource/typst_resource/raster.rs)
- [sync.rs](/Users/ravishankar/personal-work/murali/src/backend/sync.rs)

## LaTeX Setup

### Required tools

Murali's current LaTeX path needs these tools on `PATH`:

- `latex`
- `dvisvgm`

Optional but useful:

- `ffmpeg`

Use Murali's built-in doctor command to check availability:

```bash
cargo run -- doctor
```

### Recommended macOS setup

If you are using Homebrew, a working setup is typically based on:

```bash
brew install texlive dvisvgm
```

If Homebrew TeX continues to be unreliable, `mactex-no-gui` is a reasonable fallback for a more complete TeX environment.

### If `dvisvgm` is missing

Install it with:

```bash
brew install dvisvgm
```

Then re-run:

```bash
cargo run -- doctor
```

### If LaTeX fails with missing TeX support files

Symptoms may include:

- `tex.pro not found`
- `color.pro not found`
- `default map files could not be found`
- `TeXDict` PostScript errors

First try refreshing TeX file databases and maps:

```bash
mktexlsr
updmap-sys --syncwithtrees
updmap-sys
```

### If LaTeX fails with `latex.fmt` missing

Symptoms:

- `I can't find the format file latex.fmt`

Rebuild TeX formats:

```bash
fmtutil-sys --all
```

Then verify:

```bash
kpsewhich latex.fmt
```

If that prints a path, the format exists.

## Typst Setup

### Required tools

Murali's current Typst path does not require external Typst binaries on `PATH`.

It renders Typst in-process through Rust dependencies and uses:

- `typst`
- `typst-as-lib`
- `typst-svg`
- embedded fonts provided through the Typst stack

That means Typst is usually easier to get working than LaTeX on a new machine.

### Font behavior

Murali's embedded Typst backend now uses Typst's font search path with:

- system fonts enabled
- embedded Typst fonts enabled

This matters because math rendering needs a math-capable font family.
The embedded path gives Murali access to bundled fonts such as the New Computer Modern math fonts, which makes Typst math much more reliable across machines.

### If Typst fails with `current font does not support math`

This error usually means the backend did not have access to a math-capable font family.

Murali's current backend should already enable embedded Typst fonts, so if this error appears again, likely causes are:

- a dependency feature mismatch in the build
- an older cached build before the embedded-font fix
- a regression in Typst backend setup

First try:

```bash
cargo check
cargo run --example typst_showcase
```

If the error persists after a clean rebuild, inspect:

- [Cargo.toml](/Users/ravishankar/personal-work/murali/Cargo.toml)
- [compiler.rs](/Users/ravishankar/personal-work/murali/src/resource/typst_resource/compiler.rs)

### If Typst content compiles but looks too small

The current Typst pipeline normalizes mixed text + math blocks at raster time, but the exact visual balance is still under active tuning.

The main places to adjust are:

- [raster.rs](/Users/ravishankar/personal-work/murali/src/resource/typst_resource/raster.rs)
- [typst_showcase.rs](/Users/ravishankar/personal-work/murali/examples/typst_showcase.rs)

## Validation

Use this command to validate the environment:

```bash
cargo run -- doctor
```

Use these commands to validate rendering:

```bash
cargo run --example latex_showcase
cargo run --example typst_showcase
```

Reference scenes:

- [latex_showcase.rs](/Users/ravishankar/personal-work/murali/examples/latex_showcase.rs)
- [typst_showcase.rs](/Users/ravishankar/personal-work/murali/examples/typst_showcase.rs)

Current expectations:

- formulas and rich text blocks should render
- placement should be roughly correct
- color/tint should apply in both LaTeX and Typst paths
- visual polish such as size normalization is still under active improvement

## Known Gaps

Current LaTeX gaps:

- color/tint behavior still needs polish
- visual scale is not yet consistent across expressions
- bounds and centering for tall expressions can still be improved

Current Typst gaps:

- mixed text + math scaling still needs tuning
- bounds and layout ergonomics are still approximate at the frontend object level
- the rendering path still needs more visual QA scenes beyond the current showcase
