import os
import platform
from datetime import timedelta

import pytest
from hypothesis import (HealthCheck,
                        settings)

is_pypy = platform.python_implementation() == 'PyPy'
on_ci = bool(os.getenv('CI', False))
max_examples = (-(-settings.default.max_examples // 5)
                if is_pypy and on_ci
                else settings.default.max_examples)
settings.register_profile('default',
                          max_examples=max_examples,
                          suppress_health_check=[HealthCheck.too_slow])

if on_ci:
    @pytest.hookimpl(tryfirst=True)
    def pytest_runtest_call(item: pytest.Item) -> None:
        set_deadline = settings(deadline=((timedelta(hours=1)
                                           / (max_examples
                                              * len(item.session.items)))))
        item.obj = set_deadline(item.obj)


@pytest.hookimpl(trylast=True)
def pytest_sessionfinish(session: pytest.Session,
                         exitstatus: pytest.ExitCode) -> None:
    if exitstatus == pytest.ExitCode.NO_TESTS_COLLECTED:
        session.exitstatus = pytest.ExitCode.OK
