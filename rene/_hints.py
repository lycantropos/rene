import typing as _t

import typing_extensions as _te

_Key = _t.TypeVar('_Key',
                  contravariant=True)
_Value = _t.TypeVar('_Value',
                    covariant=True)


class Map(_te.Protocol[_Key, _Value]):
    def __getitem__(self, key: _Key) -> _Value:
        pass
