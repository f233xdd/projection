from math import sqrt


class Base2DGeometricComponent:
    def __init__(self, rgb: str = "#000000") -> None:
        self._rgb: str = rgb

    @property
    def rgb(self): return self._rgb


class Point(Base2DGeometricComponent):
    def __init__(self, x: float, y: float, rgb: str = "#000000") -> None:
        super().__init__(rgb)
        self._x = x
        self._y = y

    @property
    def x(self):
        return self._x

    @property
    def y(self):
        return self._y

    @property
    def pos(self) -> tuple[float, float]:
        return self._x, self._y

    def is_on(self, cpt: "Line|Plane") -> bool:
        if isinstance(cpt, Line):
            return self.__is_on_line(cpt)
        else:
            return self.__is_on_plane(cpt)

    def __is_on_line(self, ln: "Line") -> bool:
        k1, k2, b = ln.func_arg
        ep = ln.endpoint
        if k1 * self.y + k2 * self.x == b:
            if ep[0].x <= self.x <= ep[1].x and (ep[0].y - self.y) * (ep[1].y - self.y) <= 0:
                return True
        return False

    def __is_on_plane(self, pn: "Plane") -> bool:
        args = pn.border_func_args
        for k1, k2, b, r in args:
            if (k1 * self.y + k2 * self.x) * r < b * r:
                return False
        return True


class Line(Base2DGeometricComponent):
    def __init__(self, p1: Point, p2: Point, status: int = 1, rgb: str = "#000000") -> None:
        assert p1.pos != p2.pos
        super().__init__(rgb)
        self._status = status
        self._endpoint = [p1, p2]
        self._func_arg = None
        self.update_func()
        # (x2-x1)y+(y1-y2)x=x2*y1-x1*y2

    def bind(self, p: Point, ep_idx: int = 0):
        self._endpoint[ep_idx] = p
        self.update_func()
    
    def update_func(self):
        self._func_arg = cal_line_func(*self.endpoint[0].pos, *self.endpoint[1].pos)

    def is_parallel(self, ln: "Line") -> bool:
        return is_parallel(self, ln)

    def is_superposition(self, ln: "Line") -> bool:
        return is_superposition(self, ln)

    @property
    def length(self) -> float:
        return sqrt((self._endpoint[0].x - self._endpoint[1].x) ** 2 +
                    (self._endpoint[0].y - self._endpoint[1].y) ** 2)

    @property
    def status(self) -> int:
        return self._status

    @property
    def endpoint(self) -> tuple[Point, Point]:
        if self._endpoint[0].x < self._endpoint[1].x:
            return self._endpoint[0], self._endpoint[1]
        elif self._endpoint[0].x > self._endpoint[1].x:
            return self._endpoint[1], self._endpoint[0]
        else:
            if self._endpoint[0].y <= self._endpoint[1].y:
                return self._endpoint[0], self._endpoint[1]
            else:
                return self._endpoint[1], self._endpoint[0]

    @property
    def func_arg(self) -> tuple[float, float, float]:
        self.update_func()
        return self._func_arg


class Plane(Base2DGeometricComponent):
    def __init__(self, p1: Point, p2: Point, p3: Point, rgb: str = "#000000") -> None:
        assert p1.pos != p2.pos and p1.pos != p3.pos and p2.pos != p3.pos
        super().__init__(rgb)
        self._endpoint = (p1, p2, p3)
        self._border_line = [[Line(p2, p3)],
                             [Line(p1, p3)],
                             [Line(p1, p2)]]
        assert not self._border_line[0][0].is_parallel(self._border_line[1][0])

        for i in range(3):
            k1, k2, b = self._border_line[i][0].func_arg
            ep = self._endpoint[i]
            if k1 * ep.y + k2 * ep.x > b:
                self._border_line[i].append(1)
            else:  # k1 * ep.y + k2 * ep.x < b
                self._border_line[i].append(-1)

    @property
    def border_func_args(self) -> \
        tuple[tuple[float, float, float, int],
            tuple[float, float, float, int],
            tuple[float, float, float, int]]:
        return (*self._border_line[0][0].func_arg, self._border_line[0][1]), \
            (*self._border_line[1][0].func_arg, self._border_line[1][1]), \
            (*self._border_line[2][0].func_arg, self._border_line[2][1])

    @property
    def border_lines(self) -> tuple[Line, Line, Line]:
        return self._border_line[0][0], self._border_line[1][0], self._border_line[2][0]


def cal_d(p1: Point, p2: Point) -> float:
    return sqrt((p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2)


def cal_line_intersection(ln1: Line, ln2: Line) -> Point | None:
    if ln1.is_parallel(ln2):
        return None
    k11, k12, b1 = ln1.func_arg
    k21, k22, b2 = ln2.func_arg
    p = Point((k21 * b1 - k11 * b2) / (k12 * k21 - k11 * k22),
              (k22 * b1 - k12 * b2) / (k11 * k22 - k12 * k21))

    return p if p.is_on(ln1) and p.is_on(ln2) else None


def cal_plane_intersection(pn: Plane, ln: Line) -> list[Point] | None:
    res = []
    for border in pn.border_lines:
        r = cal_line_intersection(ln, border)
        if r:
            res.append(r)

    if len(res) == 2:  # max_len: 2
        if res[0].pos == res[1].pos:
            del res[1]

    return res if res else None


def cal_line_func(x1: float, y1: float, x2: float, y2: float) -> tuple[float, float, float] | None:
    """
    function sample:
        k1*y+ k2*x = b
    :return k1, k2, b
    """
    return x2 - x1, y1 - y2, x2 * y1 - x1 * y2 if x2 != x1 or y2 != y1 else None


def is_parallel(ln1: Line, ln2: Line) -> bool:
    return True if ln1._func_arg[0] * ln2.func_arg[1] == ln1._func_arg[1] * ln2.func_arg[0] else False


def is_superposition(ln1 : Line, ln2: Line) -> bool:
    if ln1.is_parallel(ln2):
        if ln1._func_arg[0] != 0:
            if ln1._func_arg[2] / ln1._func_arg[0] == ln2._func_arg[2] / ln2.func_arg[0]:
                if ln1.endpoint[0].x > ln2.endpoint[1].x or ln1.endpoint[1].x < ln2.endpoint[0].x:
                    return False
                else:
                    return True
            else:
                return False
        else:  # ln1.func_arg[1] != 0
            if ln1._func_arg[2] / ln1._func_arg[1] == ln2._func_arg[2] / ln2.func_arg[1]:
                if ln1.endpoint[0].x > ln2.endpoint[1].x or ln1.endpoint[1].x < ln2.endpoint[0].x:
                    return False
                else:
                    return True
            else:
                return False
    else:
        return False


if __name__ == "__main__":
    print("===============Test Case===============")
    pn = Plane(Point(0, 0), Point(1, 2), Point(2, 1))
    print(pn.border_func_args)
    p = Point(1, 1)
    print(p.is_on(pn))
    ln = Line(Point(1, 1), Point(0, 3))
    ln2 = Line(Point(10, 10), Point(0, 30))
    print(cal_plane_intersection(pn, ln).pop().pos)  # (0.75, 1.5)
    print(ln.is_parallel(ln2))

