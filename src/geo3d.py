class Base3DGeometricComponent:
    def __init__(self, rgb: str = "#000000") -> None:
        self._rgb = rgb
        self._func_arg = None
    
    @property
    def rgb(self): return self._rgb


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
        pass  # TODO

    def __is_on_line(self, ln) -> bool:
        pass  # TODO

    def __is_on_plane(self, pn) -> bool:
        pass  # TODO


class Line(Base3DGeometricComponent):
    def __init__(self, p1: Point, p2: Point, status=1, rgb: str = "#000000") -> None:
        assert p1.pos != p2.pos
        super().__init__(rgb)
        self._status = status
        if p1.x <= p2.x:
            self._endpoint = (p1, p2)
        else:
            self._endpoint = (p2, p1)
            
        self._func_arg = ([p2.x - p1.x, p1.y - p2.y, p2.x * p1.y - p1.x * p2.y],
                          [p2.x - p1.x, p1.z - p2.z, p2.x * p1.z - p1.x * p2.z])
        # | (x2-x1)x+(y1-y2)y=x2*y1-x1*y2
        # | (x2-x1)x+(z1-z2)z=x2*z1-x1*z2

    def is_on(self, pn: "Plane") -> bool:
        pass  # TODO

    def is_parallel(self, ln: "Line") -> bool:
        pass  # TODO

    def is_superposition(self, ln: "Line") -> bool:
        pass  # TODO

    @property
    def length(self) -> float:
        pass  # TODO


class Plane(Base3DGeometricComponent):
    def __init__(self, p1: Point, p2: Point, p3: Point, rgb: str = "#000000") -> None:
        assert p1.pos != p2.pos and p1.pos != p3.pos
        super().__init__(rgb)
        self._endpoint = (p1, p2, p3)
        # TODO


def cal_d(self, p1: Point, p2: Point) -> float:
    pass  # TODO


def cal_line_intersection(self, ln1: Line, ln2: Line) -> Point | None:
    pass  # TODO


def cal_plane_intersection(self, pl: Plane, ln: Line) -> tuple[Point] | None:
    pass  # TODO


def cal_sight_intersection(self, ln1: Line, ln2: Line) -> tuple[Point] | None:
    pass  # TODO
