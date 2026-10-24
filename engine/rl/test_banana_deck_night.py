"""The overnight deck schedule names every supported specialist except Hog."""
import datetime as dt
import unittest
from zoneinfo import ZoneInfo

from .banana_deck_night import edmonton_morning, model_name, training_decks


class DeckNight(unittest.TestCase):
    def test_names_follow_the_banana_specialist_pattern(self):
        self.assertEqual(model_name("log-bait"), "banana-log-bait-1")
        self.assertEqual(model_name("custom-mortar"), "banana-custom-mortar-1")

    def test_hog_is_skipped_because_banana_hog_1_already_exists(self):
        decks = training_decks()
        self.assertNotIn("hog-mirror", decks)
        self.assertEqual(len(decks), 10)
        self.assertEqual(decks[0], "log-bait")
        self.assertIn("custom-mortar", decks)

    def test_six_am_is_the_next_edmonton_morning(self):
        zone = ZoneInfo("America/Edmonton")
        evening = dt.datetime(2026, 10, 22, 18, 27, tzinfo=zone)
        self.assertEqual(edmonton_morning(evening), dt.datetime(2026, 10, 23, 6, 0, tzinfo=zone))
        morning = dt.datetime(2026, 10, 23, 5, 59, tzinfo=zone)
        self.assertEqual(edmonton_morning(morning), dt.datetime(2026, 10, 23, 6, 0, tzinfo=zone))


if __name__ == "__main__":
    unittest.main()
