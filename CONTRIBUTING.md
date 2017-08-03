# Contributing to `scrapmetal`

Hi! We'd love to have your contributions! If you want help or mentorship, reach
out to us in a GitHub issue, or stop by [#rust on irc.mozilla.org](irc://irc.mozilla.org#rust) and ping
`fitzgen`.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [Code of Conduct](#code-of-conduct)
- [Filing an Issue](#filing-an-issue)
- [Looking to Start Contributing to `scrapmetal`?](#looking-to-start-contributing-to-scrapmetal)
- [Building](#building)
- [Testing](#testing)
- [Automatic code formatting](#automatic-code-formatting)
- [Pull Requests and Code Reviews](#pull-requests-and-code-reviews)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Code of Conduct

We abide by the [Rust Code of Conduct][coc] and ask that you do as well.

[coc]: https://www.rust-lang.org/en-US/conduct.html

## Filing an Issue

Think you've found a bug? File an issue! To help us understand and reproduce the
issue, provide us with:

* A test case that can be used to reproduce the bug
* The steps to reproduce the bug with the test case
* The expected behavior
* The actual actual (buggy) behavior

## Looking to Start Contributing to `scrapmetal`?

* [Issues labeled "easy"](https://github.com/fitzgen/scrapmetal/issues?q=is%3Aopen+is%3Aissue+label%3Aeasy)

## Building

Make sure that `rustup` is using nightly Rust, since `scrapmetal` depends on
specialization.

```
$ cd scrapmetal/
$ rustup override set nightly
$ cargo build
```

## Testing

Once you've already told `rustup` to use nightly Rust with `scrapmetal`, all you
need to do is:

```
$ cargo test
```

## Automatic code formatting

We use [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) to enforce a consistent code style across the whole
`scrapmetal` code base.

You can install the latest version of `rustfmt` with this command:

```
$ cargo install -f rustfmt-nightly
```

Ensure that `~/.cargo/bin` is on your path.

Once that is taken care of, you can (re)format all code by running this command:

```
$ cargo fmt
```

The code style is described in the `rustfmt.toml` file in top level of the repo.

## Pull Requests and Code Reviews

Ensure that each commit stands alone, and passes tests. This enables better `git
bisect`ing when needed. If your commits do not stand on their own, then rebase
them on top of the latest master and squash them into a single commit.

All pull requests undergo code review before merging.

Unsure who to ask for review? Ask any of:

* `@fitzgen`
* TODO: need more maintainers *hint hint*

More resources:

* [A Beginner's Guide to Rebasing and Squashing](https://github.com/servo/servo/wiki/Beginner's-guide-to-rebasing-and-squashing)
