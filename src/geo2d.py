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
    def x(self): return self._x

    @property
    def y(self): return self._y

    @property
    def pos(self) -> tuple[float, float]: return self._x, self._y

    def is_on(self, cpt: "Line|Plane") -> bool:
        if isinstance(cpt, Line):
            return self.__is_on_line(cpt)
        else:
            return self.__is_on_plane(cpt)
            

    def __is_on_line(self, ln: "Line") -> bool:
        k1, k2, b = ln.func_arg
        if k1 * self.y + k2 * self.x == b:
            return True
        else:
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
        if p1.x <= p2.x:
            self._endpoint = (p1, p2)
        else:
            self._endpoint = (p2, p1)

        self._func_arg = (p2.x-p1.x, p1.y-p2.y, p2.x*p1.y-p1.x*p2.y)
        # (x2-x1)y+(y1-y2)x=x2*y1-x1*y2

    def is_parallel(self, ln: "Line") -> bool:
        return True if self._func_arg[0] * ln.func_arg[1] == self._func_arg[1] * ln.func_arg[0] else False

    def is_superposition(self, ln: "Line") -> bool:
        if self.is_parallel(ln):
            if self._func_arg[0] != 0:
                if self._func_arg[2] / self._func_arg[0] == ln._func_arg[2] / ln.func_arg[0]:
                    if  self.endpoint[0].x > ln.endpoint[1].x or self.endpoint[1].x < ln.endpoint[0].x:
                        return False
                    else:
                        return True
                else:
                    return False
            else:  # self.func_arg[1] != 0
                if self._func_arg[2] / self._func_arg[1] == ln._func_arg[2] / ln.func_arg[1]:
                    if self.endpoint[0].x > ln.endpoint[1].x or self.endpoint[1].x < ln.endpoint[0].x:
                        return False
                    else:
                        return True
                else:
                    return False

    @property
    def length(self) -> float:
        return sqrt((self._endpoint[0].x - self._endpoint[1].x)**2 + 
                    (self._endpoint[0].y - self._endpoint[1].y)**2)
    
    @property
    def status(self) -> int: return self._status
    
    @property
    def endpoint(self) -> tuple[Point, Point]: return self._endpoint
    
    @property
    def func_arg(self) -> tuple[float, float, float]: return self._func_arg
    


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
                self._border_line[i][1] = 1
            else: # k1 * ep.y + k2 * ep.x < b
                self._border_line[i][1] = -1
    
    @property
    def border_func_args(self) -> tuple[tuple[float, float, float, int], 
                                       tuple[float, float, float, int], 
                                       tuple[float, float, float, int]]:
        return (*self._border_line[0][0].func_arg, self._border_line[0][1]), \
                (*self._border_line[1][0].func_arg, self._border_line[1][1]), \
                (*self._border_line[2][0].func_arg, self._border_line[2][1])
    
    @property
    def border_lines(self) -> tuple[Line, Line, Line]: return self._border_line


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

 
def cal_plane_intersection(pl: Plane, ln: Line) -> list[Point] | None:
    res: list[Point] = []
    for border in pl.border_lines:
        r = cal_line_intersection(ln, border)
        if r:
            res.append(r)
    
    if len(res) == 2:  # max_len: 2
        if res[0].pos == res[1].pos:
            del res[1]

    return res if res else None