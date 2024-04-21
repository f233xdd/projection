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

    def combine(self, p1, p2) -> None:
        pass  # TODO


class ProjectivePlane(Plane):
    def __init__(self, p1: Point, p2: Point, p3: Point, rgb: str = "#000000") -> None:
        super().__init__(p1, p2, p3, rgb)
        self.inner = []  # TODO


def multi_project(p: tuple[float, float, float], s: tuple[float, float, float]):
    x_p, y_p, z_p = p
    x_s, y_s, z_s = s

    x0 = (x_p ** 2 + y_p ** 2 + z_p ** 2) / x_p

    k = x_p ** 2 + y_p ** 2 + z_p ** 2
    x_s1 = (k * x_p + (y_p ** 2 + z_p ** 2) * x_s - (y_p * y_s + z_p * z_s) * x_p) / k
    y_s1 = y_p / x_p * x_s1 + y_s - x_s * y_p / x_p
    z_s1 = z_p / x_p * x_s1 + z_s - x_s * z_p / x_p

    x_m = (x_p ** 2 * x0) / (x_p ** 2 + y_p ** 2)
    y_m = (x_p * y_p * x0) / (x_p ** 2 + y_p ** 2)

    x_d = (z_s1 - x_m * z_p / (x_m - x_p)) * ((x_p - x_m) / z_p)
    y_d = ((y_m - y_p) / (x_m - x_p)) * x_d + (x_p * y_m - x_m * y_p) / (x_p - x_m)
    z_d = z_s1

    x = sqrt((x_s1 - x_d) ** 2 + (y_s1 - y_d) ** 2)
    y = sqrt((x_p - x_d) ** 2 + (y_p - y_d) ** 2 + (z_p - z_d) ** 2)

    s1 = (x_s1, y_s1, z_s1)
    theta_mps1 = cal_cos_angle(s1, (x_m, y_m, 0), p)
    if x_p != 0 or y_p != 0:
        if x_p * y_p > 0:
            theta_gps1 = cal_cos_angle(s1, (x_p / 2, x_p ** 2 / (2 * y_p) + y_p, z_p), p)
        elif x_p * y_p < 0:
            theta_gps1 = cal_cos_angle(s1, (y_p ** 2 / (2 * x_p) + x_p, y_p / 2, z_p), p)
        elif x_p == 0:
            if y_p > 0:
                theta_gps1 = cal_cos_angle(s1, (-1, y_p, z_p), p)
            else:  # y_p < 0
                theta_gps1 = cal_cos_angle(s1, (1, y_p, z_p), p)
        else:  # y_p == 0
            if x_p > 0:
                theta_gps1 = cal_cos_angle(s1, (x_p, 1, z_p), p)
            else:  # x_p < 0
                theta_gps1 = cal_cos_angle(s1, (x_p, -1, z_p), p)

        if theta_mps1 > 0:
            y = - y
        if theta_gps1 < 0:
            x = - x
    else:
        if x_s1 < 0:
            x = - x
        if y_s1 < 0:
            y = - y

    return round(x, 4), round(y, 4)


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

    x0 = (x_h ** 2 + y_h ** 2 + z_h ** 2) / x_h
    x_m = x_h ** 2 * x0 / (x_h ** 2 + y_h ** 2)
    y_m = x_h * y_h * x0 / (x_h ** 2 + y_h ** 2)

    x_d = (z_s1 - x_m * z_h / (x_m - x_h)) * ((x_h - x_m) / z_h)
    y_d = ((y_m - y_h) / (x_m - x_h)) * x_d + (x_h * y_m - x_m * y_h) / (x_h - x_m)
    z_d = z_s1

    x = sqrt((x_s1 - x_d) ** 2 + (y_s1 - y_d) ** 2)
    y = sqrt((x_h - x_d) ** 2 + (y_h - y_d) ** 2 + (z_h - z_d) ** 2)

    s1 = (x_s1, y_s1, z_s1)
    h = (x_h, y_h, z_h)
    theta_mps1 = cal_cos_angle(s1, (x_m, y_m, 0), h)

    if x_h != 0 or y_h != 0:
        if x_h * y_h > 0:
            theta_gps1 = cal_cos_angle(s1, (x_h / 2, x_h ** 2 / (2 * y_h) + y_h, z_h), h)
        elif x_h * y_h < 0:
            theta_gps1 = cal_cos_angle(s1, (y_h ** 2 / (2 * x_h) + x_h, y_h / 2, z_h), h)
        elif x_h == 0:
            if y_h > 0:
                theta_gps1 = cal_cos_angle(s1, (-1, y_h, z_h), h)
            else:  # y_p < 0
                theta_gps1 = cal_cos_angle(s1, (1, y_h, z_h), h)
        else:  # y_p == 0
            if x_h > 0:
                theta_gps1 = cal_cos_angle(s1, (x_h, 1, z_h), h)
            else:  # x_p < 0
                theta_gps1 = cal_cos_angle(s1, (x_h, -1, z_h), h)
        if theta_mps1 > 0:
            y = - y
        if theta_gps1 < 0:
            x = - x
    else:
        if x_s1 < 0:
            x = - x
        if y_s1 < 0:
            y = - y

    return round(x, 4), round(y, 4)


def cal_cos_angle(P1: tuple[float, float, float],
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
        return 2


if __name__ == "__main__":
    print("============Test case============")
    DS, HD = single_project((4.63049, -2.9153, 2), 3, -59.4455, 51.8055)
    print("Single point perspective:")
    print(f"Axis-X: {DS}")
    print(f"Axis-Y: {HD}\n")

    DS, PD = multi_project((5, 5, 3.26), (4.92001, 0.82469, 5.36765))
    print("Multi point perspective:")
    print(f"Axis-X: {DS}")
    print(f"Axis-Y: {PD}\n")

    cos_theta = cal_cos_angle((2, 2, 4), (-20, 6, 47), (0, 0, 0))
    print("cos(<(2, 2, 4), (-20, 6, 47)>)= ", cos_theta)
