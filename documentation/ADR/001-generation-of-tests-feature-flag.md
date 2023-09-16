# Title
Generation of end-to-end tests happens with a feature flag
# Status
Accepted
# Context
The command 'cargo publish' failed due to the build.rs script modifying source code. This is by default not allowed.
The modification of source code only happens because of the generation of the test-code to trigger the
end-to-end tests defined by the jsons in tests/jsons.

# Decision
Add a feature flag for the generation of tests: 'generate-tests'

# Consequences
- cargo publish can be used because by default the test generation does not run, hence no source code change happens on publish
- to run end-to-end tests we need to supply the 'generate-tests' feature flag: if we do not do this the code to trigger tests is not run
- this feature flag does not need to be supplied when the content of tests changes
- this feature flag needs to be supplied when tests move around/change name/ are added/ are removed...
- this feature flag must be added to any pipeline runs

# Alternatives considered
- using the --no-verify flag with cargo publish. This would skip the check on changed source code. We do not want this, since it also skips building the contents.
- only generate the test code on test run, and never write test code to disk. This would never generate the code to trigger tests, so running individual tests would be harder



# Metadata
#### Relevant issues/links
- Issue that triggered discussion: https://github.com/tijlleenders/ZinZen-scheduler/issues/306
- PR with proposed solution: https://github.com/tijlleenders/ZinZen-scheduler/pull/389
#### Proposed on
2023-09-14
#### Accepted on
2023-09-15