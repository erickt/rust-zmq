# Contributing to rust-zmq

Thank you for your interest in contributing to rust-zmq!

## Bug reports

Please describe what you did, what you expected, and what happened
instead. Please use https://gist.github.com/ if your examples run
long.

## Feature requests

If you find a missing feature, such as functionality provided by the
underlying `libzmq` library, but not available via the Rust API
provided by the `zmq` crate, or a suggestion to improve the existing
API to make it more ergonomic, please file an issue before starting to
work on a pull request, especially when the feature requires API
changes or adds to the existing API in non-trivial ways.

This gives the maintainers a chance to provide guidance on how the
feature might be best implemented, or point out additional constraints
that need to be considered when tackling the feature.

## Pull requests

rust-zmq uses the "fork and pull" model [described
here](https://help.github.com/en/articles/about-collaborative-development-models). It
is highly recommended that you create a dedicated branch in your
repository for each pull request you submit, instead of submitting
using your `master` branch. This will make it easier on you when you
end up having multiple pull requests outstanding, and will let you
avoid rewriting history on your fork's `master` branch (see below).

### Version history

The rust-zmq project aims to keep the version history useful and
reasonably simple. Thus, when preparing and updating your pull
request, you should make liberal use of git's history rewriting
capabilities, such as amending and squashing commits. Try to observe
the following guidelines:

- Unless you are working on a complex feature that can be implemented
  in multiple, independent changes, the pull request should contain a
  single commit. Do not submit "fixup" commits, instead amend your
  commit (`git commit --amend`) or use interactive rebase (`git rebase
  -i`) to prepare commits that can be considered on their own, instead
  of requiring a reviewer to take later fixup commits into account.

- Each commit message should give a rough description of what was
  changed, and why. See ["How to Write a Commit Message"] for details
  on what a good commit message looks like, and why it matters.

- Do not merge the target branch into your pull request branch if you
  need to update your pull request with changes made in the target
  branch. Instead, use `git rebase`.

["How to Write a Commit Message"]: https://chris.beams.io/posts/git-commit/

### Tests

If you add a feature, or fix an issue, think about how it could be
tested, and add to the testsuite, if possible. This will make it
easier for reviewers to see that your code actually does what it is
supposed to do, and will prevent future regressions.
