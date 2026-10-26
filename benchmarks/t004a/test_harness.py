"""Measurement integrity: a reused label must never reuse old raw evidence."""
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch
import harness


class RunIdentityTests(unittest.TestCase):
    def test_existing_label_rejected_before_process_launch(self):
        with tempfile.TemporaryDirectory() as directory:
            raw = Path(directory)
            (raw / 'repeat-frames.jsonl').write_text('old evidence')
            with patch.object(harness, 'RAW', raw), patch.object(harness, 'launch') as launch:
                with self.assertRaisesRegex(ValueError, 'Label already has raw artifacts'):
                    harness.sample_session('tauri', 1000, 'repeat', 1)
                launch.assert_not_called()
