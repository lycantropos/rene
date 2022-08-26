from reprit.base import generate_repr


class Empty:
    __module__ = 'rene.exact'
    __slots__ = ()

    def __new__(cls):
        return super().__new__(cls)

    def __eq__(self, other):
        return (True
                if isinstance(other, Empty)
                else NotImplemented)

    def __hash__(self):
        return 0

    __repr__ = generate_repr(__new__,
                             with_module_name=True)

    def __str__(self):
        return f'{type(self).__qualname__}()'
