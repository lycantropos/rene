from hypothesis import strategies as st

from rene.exact import Box as _Box
from tests.exact_tests import strategies as _strategies

strings = st.text(st.characters(blacklist_categories=('Cc', 'Cs')))
string_key_dictionaries = st.dictionaries(strings, st.from_type(object))
non_zero_integers = _strategies.non_zero_integers
boxes_limits = _strategies.boxes_limits
boxes_like = _strategies.scalars_strategies.flatmap(
    lambda scalars: st.builds(_Box, scalars, scalars, scalars, scalars)
)
boxes = _strategies.boxes
