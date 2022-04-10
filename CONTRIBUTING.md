# Contributing to SteadiFi

A big welcome and thank you for considering contributing to the SteadiFi project!

These guidelines are to be followed by all maintainers, developers, and community working on SteadiFi. Reading and following these guidelines will help us make the contribution process easy and effective for everyone involved.

## Quicklinks

* [Getting Started](#getting-started)
    * [Issues](#issues)
    * [Pull Requests](#pull-requests)
* [Getting Help](#getting-help)

## Getting Started

Contributions are made to this repo via Issues and Pull Requests (PRs). A few general guidelines that cover both:

- To report security vulnerabilities, please send us an email. Please do post Issues or PRs related to security vulnerabilities (specifics are a work in progress).
- Search for existing Issues and PRs before creating your own.
- We work hard to makes sure issues are handled in a timely manner but, depending on the impact, it could take a while to investigate the root cause. A friendly ping in the comment thread to the submitter or a contributor can help draw attention if your issue is blocking.

### Issues

Issues should be used to report problems with the library, request a new feature, or to discuss potential changes before a PR is created.

If you find an Issue that addresses the problem you're having, please add your own reproduction information to the existing issue rather than creating a new one. Adding a [reaction](https://github.blog/2016-03-10-add-reactions-to-pull-requests-issues-and-comments/) can also help be indicating to our maintainers that a particular problem is affecting more than just the reporter.

### Pull Requests

PRs are always welcome and can be a quick way to get your fix or improvement slated for the next release. In general, PRs should:

- Only fix/add the functionality in question **OR** address wide-spread whitespace/style issues, not both.
- Add unit or integration tests for fixed or changed functionality (if a test suite already exists).
- Address a single concern in the least number of changed lines as possible.
- Include documentation in the repository.

For changes that address core functionality or would require breaking changes (e.g. a major release), it's best to open an Issue to discuss your proposal first. This is not required but can save time creating and reviewing changes.

In general, we follow the ["fork-and-pull" Git workflow](https://github.com/susam/gitpr)

1. Fork the repository to your own Github account
2. Clone the project to your machine
3. Create a branch locally with a succinct but descriptive name
4. Commit changes to the branch
5. Following any formatting and testing guidelines specific to this repository
6. Push changes to your fork

We prefer to keep our Git history linear, therefore we use a Rebase, Squash, and Merge workflow for merging. For this reason, we ask you to first rebase your branch on the most up-to-date version of the main branch available upstream:
1. Commit the work of your current branch
2. Checkout the main branch and update it to the latest version upstream.
3. `git checkout` your local feature branch that you wish to merge.
4. Execute `git rebase main` in order to apply your changes on top of the latest main branch and resolve any conflicts.
5. Make sure the updated code still works.
6. Push the changes back to your branch. This might require you do force push using `git push --force`.
7. Open a PR in our repository.
8. All commits will be squashed to a single commit and merged to main.

## Getting Help

Work in progress
