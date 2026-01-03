# rene

[![Github Actions](https://github.com/lycantropos/rene/workflows/CI/badge.svg)](https://github.com/lycantropos/rene/actions/workflows/ci.yml "Github Actions")
[![Codecov](https://codecov.io/gh/lycantropos/rene/branch/master/graph/badge.svg)](https://codecov.io/gh/lycantropos/rene "Codecov")
[![License](https://img.shields.io/github/license/lycantropos/rene.svg)](https://github.com/lycantropos/rene/blob/master/LICENSE "License")
[![PyPI](https://badge.fury.io/py/rene.svg)](https://badge.fury.io/py/rene "PyPI")
[![crates.io](https://img.shields.io/crates/v/rene.svg)](https://crates.io/crates/rene "crates.io")

In what follows `python` is an alias for `python3.10` or `pypy3.10`
or any later version (`python3.11`, `pypy3.11` and so on).

## Installation

### Prerequisites

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
python -m pip install -e '.'
```

## Usage

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
>>> from rene.enums import Location
>>> from rene.exact import Trapezoidation
>>> trapezoidation = Trapezoidation.from_polygon(square)
>>> all(vertex in trapezoidation for vertex in square.border.vertices)
True
>>> all(trapezoidation.locate(vertex) is Location.BOUNDARY
...     for vertex in square.border.vertices)
True

```

## Development

### Bumping version

#### Prerequisites

Install [bump-my-version](https://github.com/callowayproject/bump-my-version#installation).

#### Release

Choose which version number category to bump following [semver
specification](http://semver.org/).

Test bumping version

```bash
bump-my-version bump --dry-run --verbose $CATEGORY
```

where `$CATEGORY` is the target version number category name, possible
values are `patch`/`minor`/`major`.

Bump version

```bash
bump-my-version bump --verbose $CATEGORY
```

This will set version to `major.minor.patch`.

### Running tests

#### Plain

Install with dependencies

```bash
python -m pip install -e '.[tests]'
```

Run

```bash
pytest
```

#### `Docker` container

Run

- with `CPython`

  ```bash
  docker-compose --file docker-compose.cpython.yml up
  ```

- with `PyPy`

  ```bash
  docker-compose --file docker-compose.pypy.yml up
  ```

#### `Bash` script

Run

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

#### `PowerShell` script

Run

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
