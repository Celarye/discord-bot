# Contributing Guidelines <!-- omit in toc -->

Before checking the contributing guideline, please read through our [Code of Conduct][code-of-conduct].

We encourage and value all types of contributions. Refer to the [Table of Contents](#table-of-contents) for guidance on how to start contributing to this project. Following the guidelines will make the experience smoother for everyone involved, and the community looks forward to your contributions!

## Table of Contents <!-- omit in toc -->

- [Development Contributions](#development-contributions)
    - [Project Conventions](#project-conventions)
    - [Development and Build Process](#development-and-build-process)
        - [Commands](#commands)
        - [.env File Structure](#env-file-structure)
    - [Open Tasks](#open-tasks)
- [Documentation Contributions](#documentation-contributions)
- [Support Contributions](#support-contributions)
    - [Answering Questions](#answering-questions)
    - [Validating Bug Reports](#validating-bug-reports)
    - [Contributing to Enhancement Suggestions](#contributing-to-enhancement-suggestions)

## Development Contributions

### Project Conventions

- **Layout:** The layout is based on [these conventions][layout-conventions]. Please create an issue before making any changes to it.

- **Branching:** The project follows [these branching conventions][branching-conventions]. Ensure branches are named and managed accordingly.

- **Versioning:** The project uses [Semantic Versioning 2.0.0][semver-2.0.0].

- **Commit Messages:** Ensure your commit messages are clear and descriptive.

- **Pull Requests:** Each pull request must include a detailed description of the changes and relevant links to other issues or pull requests, rather than just using the commit message as the title.

### Development and Build Process

#### Commands

Check the [`Makefile`](MakeFile) for available commands and [`Cargo.toml`](Cargo.toml) for configurable features.

#### .env File Structure

The default [`Makefile`](Makefile) command uses the `dotenv` feature, allowing you to configure environment variables during development with a `.env` file:

```env
TOKEN=
REPOSITORY=
BRANCH=
COMMIT_MESSAGE=
COMMIT_AUTHOR_NAME=
COMMIT_AUTHOR_EMAIL=
LOG_LEVEL=
```

### Open Tasks

You can work on bug fixes or approved enhancement suggestions that haven't been picked up yet. Check in with us via our [Discord server][discord-server] if you want to work on those.

## Documentation Contributions

We welcome contributions that improve, update, or correct the documentation.

## Support Contributions

### Answering Questions

Assist by answering questions from other community members. Questions can be asked in our [Discord server][discord-server] or in the [Q&A Discussions][q-a-discussions]. Your insights are always appreciated!

### Validating Bug Reports

Help ensure bugs can be addressed effectively by attempting to replicate reported issues and confirming their existence. Bug reports can be made in our [Discord server][discord-server] or in the [issue tracker][issue-tracker-bugs].

### Contributing to Enhancement Suggestions

Enhancement suggestions often need feedback and refinement. Contribute by reviewing and discussing these suggestions, offering your perspective, and proposing improvements. Enhancement suggestions can be made in our [Discord server][discord-server] or in the [issue tracker][issue-tracker-enhancements]. Your input helps shape the future of the project.

[code-of-conduct]: https://github.com/inkdex/registry-manager?tab=coc-ov-file#readme
[layout-conventions]: https://doc.rust-lang.org/cargo/guide/project-layout.html
[branching-conventions]: https://stackoverflow.com/a/6065944
[semver-2.0.0]: https://semver.org/spec/v2.0.0.html
[discord-server]: https://discord.gg/inkdex
[q-a-discussions]: https://github.com/inkdex/registry-manager/discussions/categories/q-a
[issue-tracker-bugs]: https://github.com/inkdex/registry-manager/issues?q=is%3Aissue+is%3Aopen+label%3Abug
[issue-tracker-enhancements]: https://github.com/inkdex/registry-manager/issues?q=label%3Aenhancement
