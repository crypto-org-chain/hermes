# Hermes Guide

Hermes is the name of the binary that comes packaged with
[IBC Relayer CLI](https://crates.io/crates/ibc-relayer-cli) crate.

This directory comprises a comprehensive guide to Hermes.
In order to build and view this guide you need to install [`mdBook`](https://github.com/rust-lang/mdBook).
mdBook is a utility to create modern online books from Markdown files.

This guide should be permanently deployed at its latest stable version at
[hermes.informal.systems](https://hermes.informal.systems).

Current version: `v1.13.2`.

The version of this guide is aligned with the [versioning of the ibc crates](../README.md).

## Local deployment

This section describes how to deploy this guide locally on your system.

### Pre-requisites

The guide needs mdBook 0.4 and plug-ins built for it: `mdbook-template`, [`mdbook-mermaid`](https://mermaid-js.github.io/mermaid/#/) for diagrams, and `mdbook-toc` for tables of contents. The versions CI uses are pinned in [`.github/workflows/guide.yml`](../.github/workflows/guide.yml). Install them using [`cargo`](https://doc.rust-lang.org/cargo/):

```bash
cargo install --locked mdbook@0.4.52 mdbook-mermaid@0.16.2 mdbook-toc@0.14.2
```

`mdbook-template` 1.1.0 no longer compiles with `--locked` on current Rust, so take its binary from the [v1.1.0 release](https://github.com/sgoudham/mdbook-template/releases/tag/v1.1.0) or install it without `--locked`.

### Building and viewing the guide locally

In order to build and view the guide on your local machine, please follow these instructions.

#### Change to the `guide` dir

This assumes your current dir is the `ibc-rs` repository

```bash
cd guide
```

#### Build the guide

To build and view the guide in your browser, run the `mdbook` command below:

```bash
mdbook serve
```

#### View the guide

This will host the guide in your local machine. Open your browser and navigate to:

```bash
http://localhost:3000
```

## Adding or editing new content to the guide

Please check the [mdBook documentation](https://rust-lang.github.io/mdBook/index.html) for additional information on how to add new content to the guide.

Basically if you want to add new content to the guide, just add an entry to the `SUMMARY.md` Markdown file which is the TOC page. Then create a page for the entry you've added to the `SUMMARY.md` page. If you don't create the page, but save the `SUMMARY.md` file and build again, `mdBook` will create the page automatically for you.

#### Local development
If you are adding content using your favorite IDE and have a terminal opened running `mdbook serve`, it provides a convenient watch functionality, so any changes detected on local files will trigger another build and if you refresh the guide on your browser they will be shown there.

#### Submit your changes
Once you finish adding the new content just commit your changes (`git commit`) and push them to the repository (`git push`).
