"""Scientific guardrails for the offline representation measurements."""
from pathlib import Path
import sys
import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'scripts'))
from banana1_representation_study import score, fit_probe, single_neurons, bootstrap


def test_rare_positive_task_uses_balanced_accuracy():
    y=np.r_[np.zeros(99),1.]
    assert score(y,np.zeros(100),True)==.5
    assert score(y,y,True)==1.


def test_probe_scaler_and_fit_do_not_learn_from_test_rows():
    rng=np.random.default_rng(3);X=rng.normal(size=(120,8));y=X[:,0]*.8-X[:,1]*.2
    split=np.repeat([0,1,2],[60,30,30])
    _,a=fit_probe(X,y,split,False)
    X2=X.copy();X2[split==2]+=10000
    y2=y.copy();y2[split==2]=-10000
    _,b=fit_probe(X2,y2,split,False)
    for k in ['mean','scale','coef']:
        np.testing.assert_array_equal(a[k],b[k])
    assert a['alpha']==b['alpha']
    assert a['intercept']==b['intercept']


def test_single_coordinate_selection_is_independent_of_test_labels():
    rng=np.random.default_rng(19);X=rng.normal(size=(300,12));y=X[:,4]+rng.normal(0,.05,300)
    split=np.repeat([0,1,2],[180,60,60])
    _,j,fit,rank,_=single_neurons(X,y,split,False)
    shuffled=y.copy();shuffled[split==2]=rng.permutation(shuffled[split==2])*100
    _,j2,fit2,rank2,_=single_neurons(X,shuffled,split,False)
    assert j==j2==4
    assert fit==fit2
    np.testing.assert_array_equal(rank,rank2)


def test_bootstrap_resamples_games_not_individual_observations():
    y=np.array([0.,1.,2.,3.,4.]);p=np.array([.5,.9,2.7,2.9,4.3]);groups=np.arange(5)
    a=bootstrap(y,p,groups,False,repeats=100)
    # Repeating every observation 10 times adds no independent evidence.
    b=bootstrap(np.repeat(y,10),np.repeat(p,10),np.repeat(groups,10),False,repeats=100)
    np.testing.assert_allclose(a,b,atol=1e-12)
