import geo2d


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

    def __is_on_line(self, ln: "Line") -> bool:
        k11, k12, b1 = ln._func_arg[0]
        k21, k22, b2 = ln._func_arg[1]
        ep = ln._endpoint
        if geo2d.approximate(self.y * k11 + self.x * k12, b1) and \
            geo2d.approximate(self.z * k21 + self.x * k22 == b2):
            if k11 != 0 and k21 != 0:
                return ln.endpoint[0].x <= self.x <= ln.endpoint[0].x
            elif k11 == 0:
                return ln.endpoint[0].y <= self.y <= ln.endpoint[0].y
            else:  # k21 == 0
                return ln.endpoint[0].z <= self.z <= ln.endpoint[0].z
        else:
            return False

    def __is_on_plane(self, pn: "Plane") -> bool:
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
            
        self._func_arg = (geo2d.cal_line_func(p1.x, p1.y, p2.x, p2.y),
                          geo2d.cal_line_func(p1.x, p1.z, p2.x, p2.z))
        # | (x2-x1)x+(y1-y2)y=x2*y1-x1*y2
        # | (x2-x1)x+(z1-z2)z=x2*z1-x1*z2

    def is_on(self, pn: "Plane") -> bool:
        pass  # TODO

    def is_parallel(self, ln: "Line") -> bool:
        pass  # TODO

    def is_superposition(self, ln: "Line") -> bool:
        pass  # TODO
    
    @property
    def endpoint(self):
        if self._endpoint[0].x < self._endpoint[1].x:
            return self._endpoint[0], self._endpoint[1]
        elif self._endpoint[0].x > self._endpoint[1].x:
            return self._endpoint[1], self._endpoint[0]
        else:
            if self._endpoint[0].y < self._endpoint[1].y:
                return self._endpoint[0], self._endpoint[1]
            elif self._endpoint[0].y > self._endpoint[1].y:
                return self._endpoint[1], self._endpoint[0]
            else:
                if self._endpoint[0].z <= self._endpoint[1].z:
                    return self._endpoint[0], self._endpoint[1]
                else:
                    return self._endpoint[1], self._endpoint[0]
                
    @property
    def length(self) -> float:
        pass  # TODO


class Plane(Base3DGeometricComponent):
    def __init__(self, p1: Point, p2: Point, p3: Point, rgb: str = "#000000") -> None:
        assert p1.pos != p2.pos and p1.pos != p3.pos
        super().__init__(rgb)
        self._endpoint = (p1, p2, p3)
        self._func_arg = cal_plane_func(*p1.pos, *p2.pos, *p3.pos)


def cal_d(p1: Point, p2: Point) -> float:
    pass  # TODO


def cal_line_intersection(ln1: Line, ln2: Line) -> Point | None:
    pass  # TODO


def cal_plane_intersection(pl: Plane, ln: Line) -> tuple[Point] | None:
    pass  # TODO


def cal_sight_intersection(ln1: Line, ln2: Line) -> tuple[Point] | None:
    pass  # TODO


def cal_line_func(x1: float, y1: float, z1: float,
                  x2: float, y2: float, z2: float):
    return (geo2d.cal_line_func(x1, y1, x2, y2),
            geo2d.cal_line_func(x1, z1, x2, z2)) if (x1, y1, z1) != (x2, y2, z2) else None


def cal_plane_func(x1: float, y1: float, z1: float,
                   x2: float, y2: float, z2: float,
                   x3: float, y3: float, z3: float,):
    k1 = (x1-x2)*(y2-y3)-(y1-y2)*(x2-x3)
    k2 = (y1-y2)*(z2-z3)-(z1-z2)*(y2-y3)
    k3 = (z1-z2)*(x2-x3)-(x1-x2)*(z2-z3)
    b = x1 * k2 + y1 * k3 + z1 * k1
    return k1, k2, k3, b if k1 != 0 and k2 != 0 and k3 != 0 else None
    