import typing as t

import typing_extensions as te

_Key = t.TypeVar('_Key',
                  contravariant=True)
_Value = t.TypeVar('_Value',
                    covariant=True)


class Map(te.Protocol[_Key, _Value]):
    def __getitem__(self, key: _Key, /) -> _Value:
        pass
