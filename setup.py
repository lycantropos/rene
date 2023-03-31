import platform

from setuptools import (find_packages,
                        setup)

project_base_url = 'https://github.com/lycantropos/rene/'
parameters = dict(packages=find_packages(exclude=('tests', 'tests.*')),
                  url=project_base_url,
                  download_url=project_base_url + 'archive/master.zip')
if platform.python_implementation() == 'CPython':
    from setuptools_rust import RustExtension

    parameters.update(rust_extensions=[RustExtension('rene._cexact'),
                                       RustExtension('rene._crene')],
                      zip_safe=False)
setup(**parameters)
