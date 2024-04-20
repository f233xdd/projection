import project
import tkinter as tk
from tkinter import ttk

s = [(3, 3, 3), (3, 4, 3), (4, 3, 3), (4, 4, 3),
     (3, 3, 4), (3, 4, 4), (4, 3, 4), (4, 4, 4)]
# (4, -3, 7)
p = (-3, -10, -7)
l = [project.multi_project(p, s1) for s1 in s]
link = [(0, 4), (1, 5), (2, 6), (3, 7), (0, 1), (0, 2), (1, 3), (2, 3), (4, 5), (4, 6), (5, 7), (6, 7)]

root = tk.Tk()
w = tk.Canvas(root, bg="#FFFFFF", width=800, height=500)

w.grid(column=0, row=0, sticky=tk.NSEW)


def paint_point(x, y):
    w.create_oval(x - 3, y - 3, x + 3, y + 3, fill="black")


print(l)
d = []
for i in range(len(l)):
    if l[i] == None:
        d.append(i)

for j in range(len(d)):
    del l[d[j]-j]

l = [(round(x * 50) + 200, round(y * 50) + 400) for x, y in l]
print(l)
for s1 in l:
    paint_point(*s1)

for i, j in link:
    w.create_line(l[i], l[j])

root.mainloop()
