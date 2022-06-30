from typing import TypeVar

from typing_extensions import Protocol

_Key = TypeVar('_Key',
               contravariant=True)
_Value = TypeVar('_Value',
                 covariant=True)


class Map(Protocol[_Key, _Value]):
    def __getitem__(self, key: _Key) -> _Value:
        pass
