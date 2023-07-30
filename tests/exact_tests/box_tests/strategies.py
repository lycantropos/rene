from hypothesis import strategies as _st

from rene.exact import Box as _Box
from tests.exact_tests import strategies as _strategies

strings = _st.text()
string_key_dictionaries = _st.dictionaries(strings, _st.from_type(object))
non_zero_integers = _strategies.non_zero_integers
boxes_limits = _strategies.boxes_limits
boxes_like = _strategies.scalars_strategies.flatmap(
        lambda scalars: _st.builds(_Box, scalars, scalars, scalars, scalars)
)
boxes = _strategies.boxes
