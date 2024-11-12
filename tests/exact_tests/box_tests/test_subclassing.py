import types
import typing as t

import pytest
from hypothesis import given

from rene.exact import Box

from . import strategies


def test_static() -> None:
    with pytest.raises(TypeError):

        class Subclass(Box):
            pass


@given(strategies.strings, strategies.string_key_dictionaries)
def test_dynamic(name: str, members: t.Dict[str, t.Any]) -> None:
    with pytest.raises(TypeError):
        type(name, (Box,), members)
    with pytest.raises(TypeError):
        types.new_class(name, (Box,), members)
