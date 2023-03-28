import platform
from pathlib import Path

from setuptools import (find_packages,
                        setup)

project_base_url = 'https://github.com/lycantropos/rene/'


def read_file(path_string: str) -> str:
    return Path(path_string).read_text(encoding='utf-8')


parameters = dict(packages=find_packages(exclude=('tests', 'tests.*')),
                  url=project_base_url,
                  download_url=project_base_url + 'archive/master.zip')
if platform.python_implementation() == 'CPython':
    from setuptools_rust import RustExtension

    parameters.update(rust_extensions=[RustExtension('rene._cexact'),
                                       RustExtension('rene._crene')],
                      include_package_data=True,
                      zip_safe=False)
setup(**parameters)
