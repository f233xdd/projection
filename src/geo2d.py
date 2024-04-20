class Base2DGeometricComponent:
    def __init__(self, rgb: str = "#000000") -> None:
        self._rgb = rgb
        self._func_arg = None


class Point(Base2DGeometricComponent):
    def __init__(self, x, y, rgb: str = "#000000") -> None:
        super().__init__(rgb)
        self._x = x
        self._y = y

    @property
    def x(self): return self._x
    @property
    def y(self): return self._y

    def is_on(self, cpt) -> bool:
        pass

    def __is_on_line(self, ln) -> bool:
        pass

    def __is_on_plane(self, pn) -> bool:
        pass


class Line(Base2DGeometricComponent):
    def __init__(self, p1: Point, p2: Point, status=1, rgb: str = "#000000") -> None:
        super().__init__(rgb)
        self._status = status
        if p1.x <= p2.x:
            self._endpoint = (p1, p2)
        else:
            self._endpoint = (p2, p1)

    def is_parallel(self, ln) -> bool:
        pass

    def is_superposition(self, ln) -> bool:
        pass

    @property
    def length(self) -> float:
        pass


class Plane(Base2DGeometricComponent):
    def __init__(self, p1: Point, p2: Point, p3: Point, rgb: str = "#000000") -> None:
        super().__init__(rgb)
        self._endpoint = (p1, p2, p3)


def cal_d(p1, p2) -> float:
    pass

def cal_line_intersection(ln1, ln2) -> Point | None:
    pass

def cal_plane_intersection(pl, ln) -> tuple[Point] | None:
    pass
