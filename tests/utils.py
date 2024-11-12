from functools import partial, singledispatch
from typing import Any, Callable, Iterable, List, Sequence, Tuple, TypeVar

from hypothesis import strategies as _st

from rene import Location, MIN_CONTOUR_VERTICES_COUNT, Orientation, exact
from rene._utils import deduplicate, locate_point_in_point_point_point_circle

_BoxT = TypeVar('_BoxT', bound=exact.Box)
_ContourT = TypeVar('_ContourT', bound=exact.Contour)
_EmptyT = TypeVar('_EmptyT', bound=exact.Empty)
_MultipolygonT = TypeVar('_MultipolygonT', bound=exact.Multipolygon)
_MultisegmentT = TypeVar('_MultisegmentT', bound=exact.Multisegment)
_PointT = TypeVar('_PointT', bound=exact.Point)
_PolygonT = TypeVar('_PolygonT', bound=exact.Polygon)
_SegmentT = TypeVar('_SegmentT', bound=exact.Segment)
_T1 = TypeVar('_T1')
_T2 = TypeVar('_T2')


def apply(function: Callable[..., _T2], args: Iterable[_T1], /) -> _T2:
    return function(*args)


def are_sequences_equivalent(
    first: Sequence[Any], second: Sequence[Any], /
) -> bool:
    return len(first) == len(second) and all(
        first[index] == second[index] for index in range(len(first))
    )


def equivalence(left: bool, right: bool, /) -> bool:
    return left is right


def implication(antecedent: bool, consequent: bool, /) -> bool:
    return not antecedent or consequent


def is_contour_triangular(contour: _ContourT, /) -> bool:
    return len(contour.vertices) == MIN_CONTOUR_VERTICES_COUNT


def is_multisegment_inside_box(
    multisegment: _MultisegmentT, box: _BoxT, /
) -> bool:
    return all(
        is_segment_inside_box(segment, box)
        for segment in multisegment.segments
    )


def is_point_inside_box(point: _PointT, box: _BoxT, /) -> bool:
    return (
        box.min_x <= point.x <= box.max_x and box.min_y <= point.y <= box.max_y
    )


def is_point_inside_circumcircle(
    point: _PointT,
    first_vertex: _PointT,
    second_vertex: _PointT,
    third_vertex: _PointT,
    /,
) -> bool:
    return (
        locate_point_in_point_point_point_circle(
            point, first_vertex, second_vertex, third_vertex
        )
        is Location.INTERIOR
    )


def is_segment_inside_box(segment: _SegmentT, box: _BoxT, /) -> bool:
    return is_point_inside_box(segment.start, box) and is_point_inside_box(
        segment.end, box
    )


def pack(function: Callable[..., _T2], /) -> Callable[[Iterable[_T1]], _T2]:
    return partial(apply, function)


def reverse_box_coordinates(box: _BoxT, /) -> _BoxT:
    return type(box)(box.min_y, box.max_y, box.min_x, box.max_x)


@singledispatch
def reverse_compound_coordinates(compound: Any, /) -> Any:
    raise TypeError(f'Unsupported type: {type(compound)!r}.')


@reverse_compound_coordinates.register(exact.Empty)
def _(compound: exact.Empty, /) -> exact.Empty:
    return compound


@reverse_compound_coordinates.register(exact.Contour)
def _(compound: exact.Contour, /) -> exact.Contour:
    return reverse_contour_coordinates(compound)


@reverse_compound_coordinates.register(exact.Multipolygon)
def _(compound: exact.Multipolygon, /) -> exact.Multipolygon:
    return reverse_multipolygon_coordinates(compound)


@reverse_compound_coordinates.register(exact.Multisegment)
def _(compound: exact.Multisegment, /) -> exact.Multisegment:
    return reverse_multisegment_coordinates(compound)


@reverse_compound_coordinates.register(exact.Polygon)
def _(compound: exact.Polygon, /) -> exact.Polygon:
    return reverse_polygon_coordinates(compound)


@reverse_compound_coordinates.register(exact.Segment)
def _(compound: exact.Segment, /) -> exact.Segment:
    return reverse_segment_coordinates(compound)


def reverse_contour_vertices(contour: _ContourT, /) -> _ContourT:
    vertices = contour.vertices
    return type(contour)(reverse_sequence(vertices))


_T = TypeVar('_T')


def reverse_contour_coordinates(contour: _ContourT, /) -> _ContourT:
    reversed_vertices = [
        reverse_point_coordinates(vertex) for vertex in contour.vertices
    ]
    result = type(contour)(reversed_vertices)
    if result.orientation is not contour.orientation:
        result = type(contour)(
            [*reversed_vertices[:1], *reversed_vertices[:0:-1]]
        )
    return result


def reverse_multipolygon_polygons(
    multipolygon: _MultipolygonT, /
) -> _MultipolygonT:
    return type(multipolygon)(reverse_sequence(multipolygon.polygons))


def reverse_multipolygon_coordinates(
    multipolygon: _MultipolygonT, /
) -> _MultipolygonT:
    return type(multipolygon)(
        [
            reverse_polygon_coordinates(polygon)
            for polygon in multipolygon.polygons
        ]
    )


def reverse_multisegment(multisegment: _MultisegmentT, /) -> _MultisegmentT:
    return type(multisegment)(reverse_sequence(multisegment.segments))


def reverse_multisegment_coordinates(
    multisegment: _MultisegmentT, /
) -> _MultisegmentT:
    return type(multisegment)(
        [
            reverse_segment_coordinates(segment)
            for segment in multisegment.segments
        ]
    )


def reverse_point_coordinates(point: _PointT, /) -> _PointT:
    return type(point)(point.y, point.x)


def reverse_polygon_coordinates(polygon: _PolygonT, /) -> _PolygonT:
    return type(polygon)(
        reverse_contour_coordinates(polygon.border),
        [reverse_contour_coordinates(hole) for hole in polygon.holes],
    )


def reverse_polygon_holes(polygon: _PolygonT, /) -> _PolygonT:
    return type(polygon)(polygon.border, reverse_sequence(polygon.holes))


def reverse_segment_coordinates(segment: _SegmentT, /) -> _SegmentT:
    return type(segment)(
        reverse_point_coordinates(segment.start),
        reverse_point_coordinates(segment.end),
    )


def reverse_segment_endpoints(segment: _SegmentT, /) -> _SegmentT:
    return type(segment)(segment.end, segment.start)


def reverse_sequence(value: Sequence[_T], /) -> Sequence[_T]:
    return list(reversed(value))


def rotate_contour(contour: _ContourT, offset: int, /) -> _ContourT:
    return type(contour)(rotate_sequence(contour.vertices, offset))


def rotate_each_polygon_hole(polygon: _PolygonT, offset: int, /) -> _PolygonT:
    return type(polygon)(
        polygon.border,
        [rotate_contour(hole, offset) for hole in polygon.holes],
    )


def rotate_multipolygon(
    multipolygon: _MultipolygonT, offset: int, /
) -> _MultipolygonT:
    return type(multipolygon)(rotate_sequence(multipolygon.polygons, offset))


def rotate_multisegment(
    multisegment: _MultisegmentT, offset: int, /
) -> _MultisegmentT:
    return type(multisegment)(rotate_sequence(multisegment.segments, offset))


def rotate_polygon_border(polygon: _PolygonT, offset: int, /) -> _PolygonT:
    return type(polygon)(rotate_contour(polygon.border, offset), polygon.holes)


def rotate_polygon_holes(polygon: _PolygonT, offset: int, /) -> _PolygonT:
    return type(polygon)(
        polygon.border, rotate_sequence(polygon.holes, offset)
    )


def rotate_sequence(sequence: Sequence[_T1], offset: int, /) -> List[_T1]:
    if not sequence:
        return []
    offset %= len(sequence)
    return [
        *[sequence[index] for index in range(offset, len(sequence))],
        *[sequence[index] for index in range(0, offset)],
    ]


_Orienteer = Callable[[_PointT, _PointT, _PointT], Orientation]


def to_convex_hull(
    points: Sequence[_PointT], orienteer: _Orienteer[_PointT], /
) -> List[_PointT]:
    points = deduplicate(sorted(points))
    lower, upper = (
        _to_sub_hull(points, orienteer),
        _to_sub_hull(reversed(points), orienteer),
    )
    return lower[:-1] + upper[:-1] or points


to_distinct = dict.fromkeys


def to_max_convex_hull(
    points: Sequence[_PointT], orienteer: _Orienteer[_PointT], /
) -> List[_PointT]:
    points = deduplicate(sorted(points))
    lower, upper = (
        _to_max_sub_hull(points, orienteer),
        _to_max_sub_hull(reversed(points), orienteer),
    )
    return lower[:-1] + upper[:-1] or points


def to_pairs(
    values: _st.SearchStrategy[_T], /
) -> _st.SearchStrategy[Tuple[_T, _T]]:
    return _st.tuples(values, values)


def to_triplets(
    values: _st.SearchStrategy[_T], /
) -> _st.SearchStrategy[Tuple[_T, _T, _T]]:
    return _st.tuples(values, values, values)


def _to_max_sub_hull(
    points: Iterable[_PointT], orienteer: _Orienteer[_PointT], /
) -> List[_PointT]:
    result: List[_PointT] = []
    for point in points:
        while len(result) >= 2:
            if (
                orienteer(result[-2], result[-1], point)
                is Orientation.CLOCKWISE
            ):
                del result[-1]
            else:
                break
        result.append(point)
    return result


def _to_sub_hull(
    points: Iterable[_PointT], orienteer: _Orienteer[_PointT], /
) -> List[_PointT]:
    result: List[_PointT] = []
    for point in points:
        while len(result) >= 2:
            if (
                orienteer(result[-2], result[-1], point)
                is not Orientation.COUNTERCLOCKWISE
            ):
                del result[-1]
            else:
                break
        result.append(point)
    return result
