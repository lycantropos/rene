import os
import platform
import time
import typing as t
from datetime import timedelta

import pytest
from hypothesis import HealthCheck, settings

is_pypy = platform.python_implementation() == 'PyPy'
on_ci = bool(os.getenv('CI', False))
max_examples = 2 if on_ci else settings.default.max_examples
settings.register_profile(
    'default',
    deadline=None,
    max_examples=max_examples,
    suppress_health_check=[HealthCheck.too_slow],
)

# FIXME:
#  workaround until https://github.com/pytest-dev/pluggy/issues/191 is fixed
hookimpl = t.cast(t.Callable[..., t.Callable[..., None]], pytest.hookimpl)

if on_ci:
    time_left = timedelta(hours=1)

    @hookimpl(tryfirst=True)
    def pytest_runtest_call(item: pytest.Function) -> None:
        set_deadline = settings(deadline=time_left / max_examples)
        item.obj = set_deadline(item.obj)

    @pytest.fixture(scope='function', autouse=True)
    def time_function_call() -> t.Iterator[None]:
        start = time.monotonic()
        try:
            yield
        finally:
            duration = timedelta(seconds=time.monotonic() - start)
            global time_left
            time_left = max(duration, time_left) - duration


@hookimpl(trylast=True)
def pytest_sessionfinish(
    session: pytest.Session, exitstatus: pytest.ExitCode
) -> None:
    if exitstatus == pytest.ExitCode.NO_TESTS_COLLECTED:
        session.exitstatus = pytest.ExitCode.OK
