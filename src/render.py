import tkinter as tk
import threading

class CanvasHandler:
    def __init__(self, size: tuple[int, int] = None, k: float = 1) -> None:
        self._root = tk.Tk()
        if size is None:
            size = (800, 500)  # default size
        self._canvas = tk.Canvas(self._root, bg='white',
                                 width=size[0], height=size[1])
        self._canvas.grid(column=0, row=0, sticky=tk.NSEW)

    def create_point(self, p) -> None:
        pass

    def create_line(self, ln) -> None:
        pass

    def create_plane(self, pn) -> None:
        pass

    def show(self) -> None:
        threading.Thread(target=self._root.mainloop, name="mainloop_thread", daemon=True).start()

class Renderer:
    def __init__(self) -> None:
        pass

    def render(self) -> None:
        pass
