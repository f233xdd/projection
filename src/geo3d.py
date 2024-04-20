class Base3DGeometricComponent:
    def __init__(self, rgb: str = "#000000") -> None:
        self._rgb = rgb
        self._func_arg = None


class Point(Base3DGeometricComponent):
    def __init__(self, x, y, z, rgb: str = "#000000") -> None:
        super().__init__(rgb)
        self._x = x
        self._y = y
        self._z = z

    @property
    def x(self): return self._x

    @property
    def y(self): return self._y

    @property
    def z(self): return self._z

    @property
    def pos(self): return self._x, self._y, self._z

    def is_on(self, cpt: "Plane|Line") -> bool:
        pass

    def __is_on_line(self, ln) -> bool:
        pass

    def __is_on_plane(self, pn) -> bool:
        pass


class Line(Base3DGeometricComponent):
    def __init__(self, p1: Point, p2: Point, status=1, rgb: str = "#000000") -> None:
        assert p1.pos != p2.pos
        super().__init__(rgb)
        self._status = status
        if p1.x <= p2.x:
            self._endpoint = (p1, p2)
        else:
            self._endpoint = (p2, p1)

    def is_on(self, pn) -> bool:
        pass

    def is_parallel(self, ln) -> bool:
        pass

    def is_superposition(self, ln) -> bool:
        pass

    @property
    def length(self) -> float:
        pass


class Plane(Base3DGeometricComponent):
    def __init__(self, p1: Point, p2: Point, p3: Point, rgb: str = "#000000") -> None:
        assert p1.pos != p2.pos and p1.pos != p3.pos
        super().__init__(rgb)
        self._endpoint = (p1, p2, p3)


def cal_d(p1, p2) -> float:
    pass


def cal_line_intersection(ln1, ln2) -> Point | None:
    pass


def cal_plane_intersection(pl, ln) -> tuple[Point] | None:
    pass


def cal_sight_intersection(ln1, ln2) -> tuple[Point] | None:
    pass
