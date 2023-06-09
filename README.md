rene
====

[![](https://github.com/lycantropos/rene/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/lycantropos/rene/actions/workflows/ci.yml "Github Actions")
[![](https://codecov.io/gh/lycantropos/rene/branch/master/graph/badge.svg)](https://codecov.io/gh/lycantropos/rene "Codecov")
[![](https://img.shields.io/github/license/lycantropos/rene.svg)](https://github.com/lycantropos/rene/blob/master/LICENSE "License")
[![](https://badge.fury.io/py/rene.svg)](https://badge.fury.io/py/rene "PyPI")
[![](https://img.shields.io/crates/v/rene.svg)](https://crates.io/crates/rene "crates.io")

In what follows `python` is an alias for `python3.8` or `pypy3.8`
or any later version (`python3.9`, `pypy3.9` and so on).

Installation
------------

Install the latest `pip` & `setuptools` packages versions
```bash
python -m pip install --upgrade pip setuptools
```

### User

Download and install the latest stable version from `PyPI` repository
```bash
python -m pip install --upgrade rene
```

### Developer

Download the latest version from `GitHub` repository
```bash
git clone https://github.com/lycantropos/rene.git
cd rene
```

Install
```bash
python setup.py install
```

Usage
-----

```python
>>> from rene.exact import (Contour,
...                         Empty,
...                         Point,
...                         Polygon)
>>> square = Polygon(Contour([Point(0, 0), Point(4, 0), Point(4, 4),
...                           Point(0, 4)]),
...                  [])
>>> square == square
True
>>> square & square == square
True
>>> square | square == square
True
>>> square - square == Empty()
True
>>> square ^ square == Empty()
True
>>> len(square.border.vertices) == 4
True
>>> len(square.holes) == 0
True
>>> from rene.exact import ConstrainedDelaunayTriangulation
>>> (ConstrainedDelaunayTriangulation.from_polygon(square).triangles
...  == [Contour([Point(0, 0), Point(4, 0), Point(0, 4)]),
...      Contour([Point(0, 4), Point(4, 0), Point(4, 4)])])
True
>>> from rene import Location
>>> from rene.exact import Trapezoidation
>>> trapezoidation = Trapezoidation.from_polygon(square)
>>> all(vertex in trapezoidation for vertex in square.border.vertices)
True
>>> all(trapezoidation.locate(vertex) is Location.BOUNDARY
...     for vertex in square.border.vertices)
True

```

Development
-----------

### Bumping version

#### Preparation

Install
[bump2version](https://github.com/c4urself/bump2version#installation).

#### Pre-release

Choose which version number category to bump following [semver
specification](http://semver.org/).

Test bumping version
```bash
bump2version --dry-run --verbose $CATEGORY
```

where `$CATEGORY` is the target version number category name, possible
values are `patch`/`minor`/`major`.

Bump version
```bash
bump2version --verbose $CATEGORY
```

This will set version to `major.minor.patch-alpha`. 

#### Release

Test bumping version
```bash
bump2version --dry-run --verbose release
```

Bump version
```bash
bump2version --verbose release
```

This will set version to `major.minor.patch`.

### Running tests

Install dependencies
```bash
python -m pip install -r requirements-tests.txt
```

Plain
```bash
pytest
```

Inside `Docker` container:
- with `CPython`
  ```bash
  docker-compose --file docker-compose.cpython.yml up
  ```
- with `PyPy`
  ```bash
  docker-compose --file docker-compose.pypy.yml up
  ```

`Bash` script:
- with `CPython`
  ```bash
  ./run-tests.sh
  ```
  or
  ```bash
  ./run-tests.sh cpython
  ```

- with `PyPy`
  ```bash
  ./run-tests.sh pypy
  ```

`PowerShell` script:
- with `CPython`
  ```powershell
  .\run-tests.ps1
  ```
  or
  ```powershell
  .\run-tests.ps1 cpython
  ```
- with `PyPy`
  ```powershell
  .\run-tests.ps1 pypy
  ```
