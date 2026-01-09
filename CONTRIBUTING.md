# Contributing

Thank you for improving this project! We value small, precise changes that solve real problems.
We value **incremental, detail‑first contributions** over big rewrites or abstractions.

## 1. Philosophy

> "Nobody should start to undertake a large project. You start with a small _trivial_ project, and you should never expect it to get large. If you do, you'll just overdesign and generally think it is more important than it likely is at that stage. Or worse, you might be scared away by the sheer size of the work you envision. So start small, and think about the details. Don't think about some big picture and fancy design. If it doesn't solve some fairly immediate need, it's almost certainly over-designed. And don't expect people to jump in and help you. That's not how these things work. You need to get something half-way _useful_ first, and then others will say "hey, that _almost_ works for me", and they'll get involved in the project." — [Linus Torvalds](https://web.archive.org/web/20050404020308/http://www.linuxtimes.net/modules.php?name=News&file=article&sid=145)

## 2. Code & Commit Conventions

- **Branch Naming**
  Follow [Conventional Branch](https://conventional-branch.github.io/) specification:
  - `ci/<description>` for CI/CD changes
  - `docs/<description>` for documentation
  - `refactor/<description>` for refactoring
  - `test/<description>` for tests

  Example: `feat/15-players-crud-api`, `ci/separate-workflow-jobs`

- **Conventional Commits**
  Follow <https://www.conventionalcommits.org/en/v1.0.0/>:
  - `feat: …` for new features
  - `fix: …` for bug fixes
  - `chore: …` for maintenance
  - `ci: …` for CI/CD changes
  - `docs: …` for documentation

- **Commit Message Format**
  - **Header** (first line): Max 80 characters
  - **Body** (optional): Max 80 characters per line, blank line after header
  - **Footer** (optional): Reference issues (e.g., `Closes #15`)

  Example:

  ```text
  feat: add player CRUD endpoints (#15)

  - Implement GET, POST, PUT, DELETE routes for player management
  - Add squad number uniqueness validation
  - Configure server to run on port 9000

  Closes #15
  ```

- **Logical Commits**
  Group changes by purpose. It's okay to have multiple commits in a PR, but if they're mere checkpoints, squash them into a single logical commit.

- **Lint & Tests**
  Run `cargo fmt` and `cargo clippy` before committing. Ensure all tests pass with `cargo test`.

## 3. Pull Request Workflow

- **One logical change per PR.**
- **Rebase or squash** before opening to keep history concise.
- **Title & Description**
  - Title uses Conventional Commits style.
  - Description explains _what_ and _why_—keep context minimal.

## 4. Issue Reporting

- Search existing issues first.
- Provide a minimal reproducible example and clear steps.

## 5. Automation & Checks

We enforce quality via CI on every push and PR:

- **Commitlint** for commit‑message style
- **Linters/Formatters** (rustfmt, clippy)
- **Unit tests**

Failures must be fixed before review.

## 6. Code of Conduct & Support

- Please see `CODE_OF_CONDUCT.md` for behavioral expectations and reporting.
- For quick questions or discussions, open an issue with the `discussion` label or mention a maintainer.

Thanks again for helping keep this project small, simple, and impactful!
