# Title
Generation of end-to-end tests can be disabled with a feature flag
# Status
Accepted
# Context
The command 'cargo publish' failed due to the build.rs script modifying source code. This is by default not allowed.
The modification of source code only happens because of the generation of the test-code to trigger the
end-to-end tests defined by the jsons in tests/jsons.

# Decision
Add a feature flag for the skipping of generation of tests: 'skip-test-generation'

# Consequences
- cargo publish must be provided with skip-test-generation feature flag. This ensures no source code change happens on publish, allowing the publish to succeed
- we can supply the skip-test-generation feature flag if for any other reason we want to skip the generation
- all other processes should be unaffected and should not require the feature flag

# Alternatives considered
- using the --no-verify flag with cargo publish. This would skip the check on changed source code. We do not want this, since it also skips building the contents.
- only generate the test code on test run, and never write test code to disk. This would never generate the code to trigger tests, so running individual tests would be harder
- having a feature flag to generate the tests (instead of one to skip generation). This would be more error-prone because most runs would require this flag. In the current implementation, the flag should only be supplied when performing cargo publish (and there is a clear error when we forget this)



# Metadata
#### Relevant issues/links
- Issue that triggered discussion: https://github.com/tijlleenders/ZinZen-scheduler/issues/306
- PR with proposed solution: https://github.com/tijlleenders/ZinZen-scheduler/pull/389
- further discussion, documentation and slight change of plans: https://github.com/tijlleenders/ZinZen-scheduler/pull/390
#### Proposed on
2023-09-14
#### Accepted on
2023-09-15