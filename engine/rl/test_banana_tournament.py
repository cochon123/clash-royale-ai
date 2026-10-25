import unittest

from .banana_tournament import make_jobs,standings


class TournamentTest(unittest.TestCase):
    def test_all_pairs_decks_seeds_and_seats(self):
        jobs=make_jobs(['a','b','c'],['hog','bait'],[1,2])
        self.assertEqual(len(jobs),24)
        self.assertEqual(len({j['id'] for j in jobs}),24)
        for a,b in [('a','b'),('a','c'),('b','c')]:
            for deck in ['hog','bait']:
                for seed in [1,2]:
                    self.assertEqual({j['seat'] for j in jobs if j['a']==a and j['b']==b and j['deck']==deck and j['seed']==seed},{0,1})

    def test_void_never_awards_points(self):
        rows=[dict(a='a',b='b',result=r) for r in ['W','D','VOID']]
        scores={r['model']:r for r in standings(rows,['a','b'])}
        self.assertEqual(scores['a']['games'],2)
        self.assertEqual(scores['a']['wins'],1)
        self.assertEqual(scores['b']['losses'],1)
        self.assertEqual(scores['a']['points'],1.5)
        self.assertEqual(scores['b']['points'],.5)
        self.assertEqual(scores['b']['voids'],1)
        self.assertEqual(scores['a']['score'],.75)


if __name__=='__main__':unittest.main()
