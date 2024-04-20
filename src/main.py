import project
import tkinter as tk
from tkinter import ttk

from render import CanvasHandler
from geo2d import Point

s = [(3, 3, 3), (3, 4, 3), (4, 3, 3), (4, 4, 3),
     (3, 3, 4), (3, 4, 4), (4, 3, 4), (4, 4, 4)]
# (4, -3, 7)
p = (-34, -3, 7)
l = [project.multi_project(p, s1) for s1 in s]
link = [(0, 4), (1, 5), (2, 6), (3, 7), (0, 1), (0, 2), (1, 3), (2, 3), (4, 5), (4, 6), (5, 7), (6, 7)]

c = CanvasHandler(k=50)

print(l)
d = []
for i in range(len(l)):
    if l[i] is None:
        d.append(i)

for j in range(len(d)):
    del l[d[j] - j]

l = [Point(*p) for p in l]
for p in l:
    c.create_point(p)

c.create_sight_bead()
c.show()
