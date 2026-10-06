import importlib.util
import tempfile
import unittest
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

spec = importlib.util.spec_from_file_location('research_tools', Path(__file__).resolve().parents[1] / 'mcp/raw_research_mcp.py')
tools = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tools)

class SubmissionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.original = tools.RESEARCH_DIR
        tools.RESEARCH_DIR = Path(self.tmp.name) / 'research'
    def tearDown(self):
        tools.RESEARCH_DIR = self.original
        self.tmp.cleanup()
    def submit(self, title, text='fixture'):
        return tools.submit_raw_research(title, 'Synthetic author', text, ['test'])
    def test_collision_preserves_original(self):
        first = self.submit('Same title!', 'original')
        path = Path(first['file_path']); before = path.read_bytes()
        second = self.submit('Same title?', 'replacement')
        self.assertEqual(second['status'], 'CONFLICT')
        self.assertEqual(path.read_bytes(), before)
    def test_distinct_submissions(self):
        self.assertEqual(self.submit('First')['status'], 'SUCCESS')
        self.assertEqual(self.submit('Second')['status'], 'SUCCESS')
        self.assertEqual(len(tools.list_raw_research()['papers']), 2)
    def test_empty_slug_rejected(self):
        self.assertEqual(self.submit('!?')['status'], 'INVALID_TITLE')
        self.assertFalse((tools.RESEARCH_DIR / '.md').exists())
    def test_concurrent_collision_has_one_winner(self):
        with ThreadPoolExecutor(max_workers=2) as executor:
            results = list(executor.map(lambda i: self.submit('Concurrent', str(i)), range(2)))
        self.assertEqual(sorted(r['status'] for r in results), ['CONFLICT', 'SUCCESS'])
        winner = next(i for i, r in enumerate(results) if r['status'] == 'SUCCESS')
        self.assertTrue((tools.RESEARCH_DIR/'concurrent.md').read_text().endswith(str(winner)))
    def test_existing_symlink_not_followed(self):
        tools.RESEARCH_DIR.mkdir(); target = Path(self.tmp.name)/'original.md'
        target.write_text('preserve'); (tools.RESEARCH_DIR/'linked.md').symlink_to(target)
        self.assertEqual(self.submit('Linked')['status'], 'CONFLICT')
        self.assertEqual(target.read_text(), 'preserve')

if __name__ == '__main__': unittest.main()
