from ahrs_fusion import NxpFusion


def test_instantiate():
    _n = NxpFusion.new(52)


def test_update():
    nxp = NxpFusion.new(150)
    nxp.update(1., 2., 3., 4., 5., 6., 0., 0., 0.)


def test_reset_nxp():
    nxp = NxpFusion.new(150.)
    nxp1 = NxpFusion.new(150.)

    assert nxp == nxp1

    nxp.update(1., 2., 3., 4., 5., 6., 0., 0., 0.)
    assert nxp != nxp1

    nxp.reset()
    nxp1.reset()

    assert nxp == nxp1


def test_set_and_get_quaternion():
    nxp = NxpFusion.new(150)
    nxp.set_quaternion([0.5, 1., 2., 3.])
    assert nxp.quaternion == [0.5, 1., 2., 3.]
