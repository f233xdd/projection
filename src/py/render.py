import tkinter as tk
import threading

from geo2d import Point, Line, Plane


class CanvasHandler:
    def __init__(self, size: tuple[int, int] = None, k: float = 1) -> None:
        self._root = tk.Tk()
        if size is None:
            self._size = (800, 500)  # default size
        self._k = k  # normally, k >= 1
        self._canvas = tk.Canvas(self._root, bg='white',
                                 width=self._size[0], height=self._size[1])
        self._canvas.grid(column=0, row=0, sticky=tk.NSEW)

    def __transform(self, pos: tuple[float, float], enable_k: bool = True) -> tuple[float, float]:
        return (pos[0] * self._k + self._size[0] / 2, - pos[1] * self._k + self._size[1] / 2) \
            if enable_k else (pos[0] + self._size[0] / 2, - pos[1] + self._size[1] / 2)

    def create_point(self, p: Point, enable_k: bool = True) -> None:
        x, y = self.__transform(p.pos, enable_k=enable_k)
        self._canvas.create_oval(x - 2, y - 2, x + 2, y + 2, fill=p.rgb)

    def create_line(self, ln: Line, enable_k: bool = True) -> None:
        if ln.status == 1:
            self._canvas.create_line(*self.__transform(ln.ep[0].pos, enable_k),
                                     *self.__transform(ln.ep[1].pos, enable_k))
        else:
            self._canvas.create_line(*self.__transform(ln.ep[0].pos, enable_k),
                                     *self.__transform(ln.ep[1].pos, enable_k), dash=(2, 4))

    def create_plane(self, pn: Plane, enable_k: bool = True) -> None:
        pass  # TODO

    def show(self) -> None:
        self._root.mainloop()

    def create_sight_bead(self) -> None:
        self.create_line(Line(Point(-10, 0), Point(10, 0), 0), enable_k=False)
        self.create_line(Line(Point(0, -10), Point(0, 10), 0), enable_k=False)


class Renderer:
    def __init__(self) -> None:
        pass  # TODO

    def render(self) -> None:
        pass  # TODO
