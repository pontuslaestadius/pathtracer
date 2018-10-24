# Contributing

Contrubitions are not fully automated and require manual oversign to be integrated.

## Way of working

This repository initially worked on a code-first bases but has transitioned to a heavier TDD approach. This means that added functionality should come with additional tests. And any issue that is found should be attempted to be identified with a failing test.

### Github

Changes are commited as pull requests, evaulated by Travis and manually merged in by the maintainer.

### Test suites

Tests are managed differently depending on if it's cargo's built in testing used with the #[test] annotation or higher level testing.

#### src/tests

Handles exensive testing of components and integration and system testing.
Any single function tests should be in the same file as the function.

#### /tests

Holds rustup component testing such as lint, style checking and pedantic whitespace checks.
These are run by the test framework script in /scripts.

### Issue tracking

All discovered issues are to be labeled with 'FIXME' in the source code. Issues require a issue number and disallows untracked FIXME's. So if you discover an issue, open a issue ticket followed by a pull request for the added FIXME if it has an appropriate source code position.

# Have fun!

This is merly a side project and without a pre-defined goal. Do not expect specific features to be implemented.

