from math import sin, cos, sqrt, radians

from geo2d import Point, Line, Plane


class ProjectivePoint(Point):
    def __init__(self, x, y, tag: str = "A", rgb: str = "#000000") -> None:
        super().__init__(x, y, rgb)
        self._tag = tag  # TODO

    @property
    def tag(self): return self._tag


class ProjectiveLine(Line):
    def __init__(self, p1: Point, p2: Point, status=1, rgb: str = "#000000") -> None:
        super().__init__(p1, p2, status, rgb)
        self.parts = [Line(self._endpoint[0], self._endpoint[1], 1, rgb=rgb)]

    def combine(self, ln: Line) -> None:
        if not self.is_superposition(ln) or not ln.status == 0:
            return
        if self.func_arg[0] != 0:
            v = 'x'
        else:
            v = 'y'

        i = len(self.parts)
        while True:
            ln0 = self.parts.pop(0)
            if getattr(ln0.endpoint[0], v) <= getattr(ln.endpoint[0], v) <= getattr(ln0.endpoint[1], v):
                if ln0.status == 0 or getattr(ln.endpoint[0], v) == getattr(self.endpoint[0], v):
                    ep = ln0.endpoint[0]
                else:
                    self.parts.append(Line(ln0.endpoint[0], ln.endpoint[0], 1))
                    ep = ln.endpoint[0]
                i += -1
                break
            self.parts.append(ln0)
            i += -1

        while True:
            if getattr(ln0.endpoint[1], v) >= getattr(ln.endpoint[1], v):
                if ln0.status == 0 or getattr(ln.endpoint[1], v) == getattr(self.endpoint[1], v):
                    self.parts.append(Line(ep, ln0.endpoint[1], 0))
                else:
                    self.parts.append(Line(ep, ln.endpoint[1], 0))
                    self.parts.append(Line(ln.endpoint[1], ln0.endpoint[1], 1))
                break
            ln0 = self.parts.pop(0)
            i += -1

        for __ in range(i):
            self.parts.append(self.parts.pop(0))


class ProjectivePlane(Plane):
    def __init__(self, p1: Point, p2: Point, p3: Point, rgb: str = "#000000") -> None:
        super().__init__(p1, p2, p3, rgb)
        self.inner = []  # TODO


def multi_project(p: tuple[float, float, float], s: tuple[float, float, float]):
    x_p, y_p, z_p = p
    x_s, y_s, z_s = s

    k = x_p ** 2 + y_p ** 2 + z_p ** 2
    x_s1 = (k * x_p + (y_p ** 2 + z_p ** 2) * x_s - (y_p * y_s + z_p * z_s) * x_p) / k
    y_s1 = y_p / x_p * x_s1 + y_s - x_s * y_p / x_p
    z_s1 = z_p / x_p * x_s1 + z_s - x_s * z_p / x_p
    
    # TODO


def single_project(s: tuple[float, float, float], r0: float, theta_xoy: float, theta_z: float):
    theta_xoy = radians(theta_xoy)
    theta_z = radians(theta_z)
    x_s, y_s, z_s = s

    x_h = r0 * cos(theta_z) * cos(theta_xoy)
    y_h = r0 * cos(theta_z) * sin(theta_xoy)
    z_h = r0 * sin(theta_z)

    k_s = x_h * x_s + y_h * y_s + z_h * z_s
    if k_s == 0:
        return None
    x_s1 = r0 ** 2 * x_s / k_s
    y_s1 = r0 ** 2 * y_s / k_s
    z_s1 = r0 ** 2 * z_s / k_s

    d = sqrt((x_h-x_s1)**2 + (y_h-y_s1)**2 + (z_h-z_s1)**2)
    if (x_h > 0 and y_h > 0) or (x_h < 0 and y_h < 0):
        x = d * cal_cos((x_h/2, x_h**2/(2*y_h)+y_h, z_h),
                        (x_s1, y_s1, z_s1), (x_h, y_h, z_h))
    elif (x_h > 0 and y_h < 0) or (x_h < 0 and y_h > 0):
        x = d * cal_cos((y_h**2/(2*x_h)+x_h, y_h/2, z_h),
                        (x_s1, y_s1, z_s1), (x_h, y_h, z_h))
    elif y_h == 0 and x_h != 0:
        if x_h > 0:
            x = d * cal_cos((x_h, 1, z_h), (x_s1, y_s1, z_s1), (x_h, y_h, z_h))
        else:  # x_h < 0
            x = d * cal_cos((x_h, -1, z_h), (x_s1, y_s1, z_s1),
                            (x_h, y_h, z_h))
    elif y_h != 0 and x_h == 0:
        if y_h > 0:
            x = d * cal_cos((-1, y_h, z_h), (x_s1, y_s1, z_s1),
                            (x_h, y_h, z_h))
        else:  # y_h < 0
            x = d * cal_cos((1, y_h, z_h), (x_s1, y_s1, z_s1), (x_h, y_h, z_h))
    else:  # x_h == 0 and y_h == 0
        raise RuntimeError

    if z_h > 0:
        y = -d * cal_cos((x_h+z_h**2*x_h/(x_h**2+y_h**2), y_h+z_h**2*y_h/(x_h**2+y_h**2), 0), 
                         (x_s1, y_s1, z_s1), 
                         (x_h, y_h, z_h))
    elif z_h < 0:
        y = d * cal_cos((x_h+z_h**2*x_h/(x_h**2+y_h**2), y_h + z_h**2*y_h/(x_h**2+y_h**2), 0), 
                        (x_s1, y_s1, z_s1), 
                        (x_h, y_h, z_h))
    else:  # z_h == 0
        y = z_s1

    return x, y
    
    
def cal_cos(P1: tuple[float, float, float],
                  P2: tuple[float, float, float],
                  O: tuple[float, float, float]):
    x1, y1, z1 = P1
    x2, y2, z2 = P2
    x0, y0, z0 = O
    OP1_2 = (x0 - x1) ** 2 + (y0 - y1) ** 2 + (z0 - z1) ** 2
    OP2_2 = (x0 - x2) ** 2 + (y0 - y2) ** 2 + (z0 - z2) ** 2
    P1P2_2 = (x2 - x1) ** 2 + (y2 - y1) ** 2 + (z2 - z1) ** 2
    try:
        return (OP1_2 + OP2_2 - P1P2_2) / (2 * sqrt(OP1_2) * sqrt(OP2_2))
    except ZeroDivisionError:
        return None


if __name__ == "__main__":
    print("============Test case============")
    DS, HD = single_project((4.63049, -2.9153, 2), 3, -59.4455, 51.8055)
    print("Single point perspective:")
    print(f"Axis-X: {DS}")
    print(f"Axis-Y: {HD}\n")

    # DS, PD = multi_project((5, 5, 3.26), (4.92001, 0.82469, 5.36765))
    # print("Multi point perspective:")
    # print(f"Axis-X: {DS}")
    # print(f"Axis-Y: {PD}\n")

    cos_theta = cal_cos((2, 2, 4), (-20, 6, 47), (0, 0, 0))
    print("cos(<(2, 2, 4), (-20, 6, 47)>)= ", cos_theta)

    l1 = ProjectiveLine(Point(0, 0), Point(12, 0))
    l = []
    for i, j in [(2, 3), (1, 3), (5, 4), (10, 8)]:
        l.append(Line(Point(i, 0), Point(j, 0), 0))

    for l0 in l:
        l1.combine(l0)
    
    for _ in l1.parts:
        print(_, _.status)
