import json_hydrate


def test_hydrate():
    item = {"foo": "bar"}
    base = {"baz": "boz"}
    item = json_hydrate.hydrate(item, base)
    assert item == {"foo": "bar", "baz": "boz"}
