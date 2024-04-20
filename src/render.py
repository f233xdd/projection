import tkinter as tk


class CanvasHandler:
    def __init__(self, size: tuple[int, int] = None, k: float=1) -> None:
        self._root = tk.Tk()
        if size is None:
            size = (800, 500)  # default size
        self._canvas = tk.Canvas(self._root, bg='white', 
                                 width=size[0], height=size[1])
        self._canvas.grid(column=0, row=0, sticky=tk.NSEW)
    
    def create_point(self, p):
        pass
    
    def create_line(self, ln):
        pass
    
    def create_plane(self, pn):
        pass
    
    def show(self):
        self._root.mainloop()


class Renderer:
    
    def __init__(self) -> None:
        pass
    
    def render(self):
        pass