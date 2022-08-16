# Contributing to Infobip Community

✨✨ First off, thanks for taking the time to contribute! ✨✨

This project adheres to the Contributor Covenant [code of conduct](CODE_OF_CONDUCT.md). By participating,
you are expected to uphold this code. Please report unacceptable behavior to DevRel@infobip.com.

The following is a set of guidelines for contributing to Infobip's SDKs or any other projects
hosted in the [Infobip Organization](https://github.com/infobip-community) on GitHub.
These are mostly guidelines, not rules. Use your best judgment,
and feel free to propose changes to this document in a pull request.

## 🚩 Issues
How to report a bug?

Bugs are tracked as [GitHub issues](https://docs.github.com/en/issues/tracking-your-work-with-issues/about-issues). After you've determined which repository your bug is related to,
create an issue on that repository and provide the following information by filling in comment section.

Explain the problem and include additional details to help maintainers reproduce the problem:
* **Use a clear and descriptive title** for the issue to identify the problem.
* **Describe the exact steps which reproduce the problem** in as many details as possible.
* **Provide specific examples to demonstrate the steps**.
* **Describe the behavior you observed after following the steps**
* **Explain which behavior you expected to see instead and why.**
* **Can you reliably reproduce the issue?** If not, provide details about how often the problem happens and under which conditions it normally happens.

## ℹ️ Asking for General Help

The [Infobip website](https://www.infobip.com/docs/api) has a list of resources for getting programming help and more.
For any question contributors are available at [DevRel@infobip.com](mailto:DevRel@infobip.com).
Please use the issue tracker for bugs only!

## ⬇️ Pull request

### 🍴 Step 1: Fork
Fork the project on GitHub and clone your fork locally.
Example for Python SDK repository:
```bash
git clone https://github.com/<your username>/infobip-api-go-sdk.git
cd infobip-api-go-sdk
git remote add upstream https://github.com/infobip-community/infobip-api-go-sdk.git
git fetch upstream
```
### 🛠️ Step 2: Build
Please run all the tests in the repository, all tests should pass.
You can manually test by calling the included script from the root of the project:
```bash
bash scripts/run_tests.sh
```

### 🌱 Step 3: Branch
To keep your development environment organized, create local branches to hold your work.
These should be branched directly off of the main branch.

```bash
git checkout -b my-branch -t upstream/main
```

### 💻 Step 4: Code
Please follow code structure and naming structure that is already in the repository.
Please make sure that all tests and linters pass for all changes. You can check for that in the [GitHub Actions.](https://github.com/infobip-community/infobip-api-rust-sdk/actions)
We mostly adhere to the [Rust API Guidelines Checklist](https://rust-lang.github.io/api-guidelines/checklist.html) for style and best practices.
We use `rustfmt` to keep the code well-formatted.

### ✅ Step 5: Commit
It is recommended to keep your changes grouped logically within individual commits.
Many contributors find it easier to review changes that are split across multiple commits.
There is no limit to the number of commits in a pull request.

```bash
git add my/changed/files
git commit
```

Note that multiple commits get squashed when they are landed.
A good commit message should describe what changed and why.
Commit message should:

* Contain a short description of the change (preferably 50 characters or less, and no more than 72 characters)
* First letter should be capital and rest entirely in lowercase with the exception of proper nouns, acronyms,
  and the words that refer to code, like function/variable names

#### ⚠️ Breaking Changes

When a commit has a breaking change first line of commit text should be BREAKING CHANGE.
Please take a look at [Cargo's guide on Semantic Versioning](https://doc.rust-lang.org/cargo/reference/semver.html) for more information.

### 📌 Step 6: Rebase
Once you have committed your changes, it is a good idea to use git rebase (not git merge) to synchronize your work with the main repository.
```bash
git fetch upstream
git rebase upstream/main
```

### 🧪 Step 7: Tests
Bug fixes and features should always come with tests. Look at other tests to see how they should be structured.
Before submitting your changes in a pull request, always run the full test suite
```bash
cargo test
cargo fmt --check
cargo clippy
```
Make sure that no check reports issues and that all tests pass. Please do not submit patches that fail either check.

### 🚀 Step 8: Push
Once your commits are ready to go -- with passing tests and linting -- begin the process of opening a pull request by pushing your working branch to your fork on GitHub.
```bash
git push origin my-branch
```

### 📬 Step 9: Opening the Pull Request
From within GitHub, open new pull request. Add repository admins as reviewers.
Your PR may be delayed in being merged as maintainers seek more information or clarify ambiguities.

### 🤼 Step 10: Discuss and update
You will probably get feedback or requests for changes to your pull request.
This is a big part of the submission process so don't be discouraged!
Some contributors may sign off on the pull request right away.
Others may have detailed comments or feedback.
This is a necessary part of the process in order to evaluate whether the changes are correct and necessary.

To make changes to an existing pull request, make the changes to your local branch,
add a new commit with those changes, and push those to your fork. GitHub will automatically update the pull request.

```bash
git add my/changed/files
git commit
git push origin my-branch
```

Feel free to post a comment in the pull request to ping reviewers if you are awaiting an answer on something.

### 🌍 Step 11: Landing

In order to land, a pull request needs to be reviewed and approved by at least one repository Owner and pass CI.
After that, if there are no objections from other contributors, the pull request can be merged.

🎉🎊 Congratulations and thanks for your contribution! 🎊🎉

Every pull request is tested on the Continuous Integration (CI) system to confirm that it works.
Ideally, the pull request will pass ("be green") on all of CI's tests.
This means that all tests pass and there are no linting errors.
However, it is not uncommon for the CI infrastructure itself to fail on specific platforms or for so-called "flaky" tests to fail ("be red").
Each CI failure must be manually inspected to determine the cause.

## 📜 Code of Conduct

This project and everyone participating in it is governed by the [Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code.
Please report unacceptable behavior to [DevRel@infobip.com](mailto:DevRel@infobip.com).

## 📚 Further Reading

For more in-depth guides on developing SDKs, see
[Readme](README.md) and [Infobip's website](https://www.infobip.com/docs/api).
