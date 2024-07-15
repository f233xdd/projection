from collections import deque
from math import sqrt, sin, cos, radians


from approx import ApproxFloat

class BaseGeo2DComponent:
    def __init__(self, rgb: str = "#000000") -> None:
        self._rgb: str = rgb

    @property
    def rgb(self): return self._rgb


class Point(BaseGeo2DComponent):
    def __init__(self, x: float, y: float, rgb: str = "#000000") -> None:
        super().__init__(rgb)
        self._x = ApproxFloat(x)
        self._y = ApproxFloat(y)

    def __repr__(self) -> str:
        return str(self.pos)

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
        ep = ln.ep
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


class Line(BaseGeo2DComponent):
    def __init__(self, p1: Point, p2: Point, status: int = 1, rgb: str = "#000000") -> None:
        assert p1.pos != p2.pos
        super().__init__(rgb)
        self._status = status
        self._ep = [p1, p2]
        self._func_arg = None
        self.update_func()
        # (x2-x1)y+(y1-y2)x=x2*y1-x1*y2

    # def bind(self, p: Point, ep_idx: int = 0):
    #     self._endpoint[ep_idx] = p
    #     self.update_func()

    def update_func(self):
        self._func_arg = cal_line_func(*self.ep[0].pos, *self.ep[1].pos)

    def is_parallel(self, ln: "Line") -> bool:
        return is_parallel(self, ln)

    def is_superposition(self, ln: "Line") -> bool:
        return is_superposition(self, ln)

    def __str__(self) -> str:
        ed1, ed2 = self.ep
        k1, k2, b = self.func_arg
        if self.func_arg[0] != 0:
            return "{(x, y)|" + f"{k1}*y+{k2}*x={b}, {ed1.x}<=x<={ed2.x}" + "}"
        else:
            return "{(x, y)|" + f"{k1}*y+{k2}*x={b}, {ed1.y}<=y<={ed2.y}" + "}"

    @property
    def length(self) -> float:
        return cal_d(self._ep[0], self._ep[1])

    @property
    def status(self) -> int:
        return self._status

    @property
    def ep(self) -> tuple[Point, Point]:
        if self._ep[0].x < self._ep[1].x:
            return self._ep[0], self._ep[1]
        elif self._ep[0].x > self._ep[1].x:
            return self._ep[1], self._ep[0]
        else:
            if self._ep[0].y <= self._ep[1].y:
                return self._ep[0], self._ep[1]
            else:
                return self._ep[1], self._ep[0]

    @property
    def func_arg(self) -> tuple[float, float, float]:
        return self._func_arg


class Plane(BaseGeo2DComponent):
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


class Polyline(BaseGeo2DComponent):
    def __init__(self, base: Line, rgb: str = "#000000") -> None:
        super().__init__(rgb)
        self._lines = deque()
        self._ep = deque()

        self._lines.append(base)
        for p in base.ep:
            self._ep.append(p) 

    def connect(self, ln: Line):
        if self._ep[0] == ln.ep[0]:
            self._lines.appendleft(ln)
            self._ep.appendleft(ln.ep[1])
        elif self._ep[0] == ln.ep[1]:
            self._lines.appendleft(ln)
            self._ep.appendleft(ln.ep[0])
        elif self._ep[-1] == ln.ep[0]:
            self._lines.append(ln)
            self._ep.append(ln.ep[1])
        elif self._ep[-1] == ln.ep[1]:
            self._lines.append(ln)
            self._ep.append(ln.ep[0])
        else:
            raise ValueError
    
    def is_polygon(self) -> bool:
        return True if self._ep[0] == self._ep[-1] else False

    @property
    def ep(self) -> deque[Point]:
        return self._ep

    @property
    def line(self) -> deque[Line]:
        return self._lines

def cal_d(p1: Point, p2: Point) -> float:
    return sqrt((p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2)

def cal_mid_point(p1: Point, p2: Point) -> Point:
    return Point((p1.x + p2.x)/2, (p1.y + p2.y)/2)


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

    return del_repeated_point(res) if res else None


def cal_line_func(x1: float, y1: float, x2: float, y2: float) -> tuple[float, float, float] | None:
    """
    function sample:
        k1*y+ k2*x = b
    :return k1, k2, b
    """
    return x2 - x1, y1 - y2, x2 * y1 - x1 * y2 if x2 != x1 or y2 != y1 else None


def rotate(p: Point, theta: float) -> Point:
    return Point(p.x * cos(theta) - p.y * sin(theta), p.x * sin(theta) + p.y * cos(theta))


def is_parallel(ln1: Line, ln2: Line) -> bool:
    return True if ln1._func_arg[0] * ln2.func_arg[1] == ln1._func_arg[1] * ln2.func_arg[0] else False


def is_superposition(ln1: Line, ln2: Line) -> bool:
    k11, k12, b1 = ln1.func_arg
    k21, k22, b2 = ln2.func_arg
    if ln1.is_parallel(ln2):
        if k11 != 0:
            if b1 / k11 == b2 / k21:
                if ln1.ep[0].x > ln2.ep[1].x or ln1.ep[1].x < ln2.ep[0].x:
                    return False
                else:
                    return True
            else:
                return False
        else:  # ln1.func_arg[1] != 0
            if b1 / k12 == b2 / k22:
                if ln1.ep[0].x > ln2.ep[1].x or ln1.ep[1].x < ln2.ep[0].x:
                    return False
                else:
                    return True
            else:
                return False
    else:
        return False


def is_in_polygon(p: Point, pln: Polyline):
    assert pln.is_polygon()

    for l in pln.line:
        if p.is_on(l):
            return True

    d = 0
    i = 0
    while i != len(pln.ep):
        if cal_d(pln.ep[i], p) > d:
            ep1 = pln.ep[i]
            try:
                if cal_d(pln.ep[i-1], p) >= cal_d(pln.ep[i+1], p):
                    ep = cal_mid_point(ep1, pln.ep[i-1])
                else:
                    ep = cal_mid_point(ep1, pln.ep[i+1])
                break
            except IndexError:
                if cal_d(pln.ep[i-1], p) >= cal_d(pln.ep[-1], p):
                    ep = cal_mid_point(ep1, pln.ep[i-1])
                else:
                    ep = cal_mid_point(ep1, pln.ep[-1])
                break
                

    ln = Line(p, ep)
    count = 0
    repeat = 0
    d = None
    for l in pln.line:
        r = cal_line_intersection(ln, l)
        if r:
            tmp = cal_d(r, p)
            if d:
                if tmp < d:
                    d = tmp
                    repeat = 1
                elif tmp == d:
                    repeat += 1
            else:
                d = tmp
            count += 1
    if repeat == 2:
        count += -1

    if count % 2 == 0:
        return False
    else:
        return True


def del_repeated_point(points: list[Point]) -> list[Point]:
    i = 0
    while i < len(points):
        j = i + 1
        while j < len(points):
            if points[i].pos == points[j].pos:
                points.pop(j)
                j += -1
            j += 1
        i += 1
    return points


if __name__ == "__main__":
    print("===============Test Case===============")
    # pn = Plane(Point(0, 0), Point(1, 2), Point(2, 1))
    # print(pn.border_func_args)
    # assert Point(1, 1).is_on(pn) is True
    # ln = Line(Point(1, 1), Point(0, 3))
    # ln2 = Line(Point(10, 10), Point(0, 30))
    # print(cal_plane_intersection(pn, ln).pop().pos)  # (0.75, 1.5)
    # assert ln.is_parallel(ln2) is True
    p = [Point(-1.3873, 3.86814),
         Point(-1.20899, 0.80864),
         Point(0.17469, 1.97528),
         Point(2.56222, -1.76881),
         Point(5.73656, 3.65741),
         Point(6.68614, -0.79209),
         Point(3.97304, -3.04397),
         Point(1.80255, -5.40437),
         Point(1.34133, -3.15249),
         Point(-0.23227, -1.4975),
         Point(-1.10047, -5.29584),
         Point(-0.74776, 0.02184),
         Point(-3.75931, -4.23773),
         Point(-5.59326, -3.58546)
         ]

    p2 = p.pop(0)
    polyline = Polyline(base=Line(p2, p[0]))
    for i in range(len(p) - 1):
        polyline.connect(Line(p[i], p[i + 1]))
    polyline.connect(Line(p2, p[-1]))

    p1 = Point(-0.73633, -2.32572)
    print(is_in_polygon(p1, polyline))
