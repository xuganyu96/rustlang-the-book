/** Integration tests are placed in the tests/ directory instead of the src/
 * directory. They are treated like external code, and each integration test
 * file needs to bring in the tested module separately
 *
 * Integration tests are run alongside unit tests if calling "cargo test",
 * but can be also run separately using
 * "cargo test --test <name_of_file>"
 */
use automated_tests;

#[test]
fn integration_test() {
    assert_eq!(automated_tests::add(2, 2), 4);
}

