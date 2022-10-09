from python_data_model.vectors import Vector


def test_vectors():
    v1 = Vector(2, 4)
    v2 = Vector(2, 1)
    assert v1 + v2 == Vector(4, 5)

    v = Vector(3, 4)
    assert abs(v) == 5.0

    print("v * 3:", v * 3)
    assert v * 3 == Vector(9, 12)
