"""Attribution must fail closed when a shell launches the app under its owner."""

import unittest

from ownership import classify


def fact(pid, responsible, ppid=1, resource=10, jetsam=11):
    return {
        "pid": pid,
        "ppid": ppid,
        "name": "fixture",
        "responsible_pid": responsible,
        "resource_coalition": resource,
        "jetsam_coalition": jetsam,
    }


class OwnershipTests(unittest.TestCase):
    def test_self_responsible_app_owns_helpers_but_not_other_app(self):
        rows = [fact(100, 100), fact(101, 100), fact(200, 200)]
        result = classify(100, rows)
        self.assertEqual(result["pids"], [100, 101])
        self.assertTrue(result["self_responsible"])

    def test_shared_shell_coalition_never_proves_app_ownership(self):
        rows = [fact(100, 50), fact(101, 50), fact(50, 50)]
        result = classify(100, rows)
        self.assertEqual(result["pids"], [100])
        self.assertFalse(result["self_responsible"])

    def test_direct_descendant_uses_parent_chain_when_responsibility_inherited(self):
        rows = [fact(100, 50), fact(101, 50, ppid=100), fact(102, 50, ppid=101)]
        self.assertEqual(classify(100, rows)["pids"], [100, 101, 102])

    def test_missing_or_failed_responsibility_is_not_zero_equals_zero(self):
        rows = [fact(100, None), fact(101, None)]
        self.assertEqual(classify(100, rows)["pids"], [100])

    def test_missing_main_fails_closed(self):
        self.assertEqual(classify(100, [fact(101, 100)])["pids"], [])


if __name__ == "__main__":
    unittest.main()
