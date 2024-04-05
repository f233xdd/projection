import cal
import tkinter as tk
from tkinter import ttk

s = [(0, 1, 0), (1, 0, 0), (0, 0, 1)]
# (4, -3, 7)
p = (4, -3, 7)
l = [cal.single_project(s1, 2, 0, 45) for s1 in s]
# link = [(0, 1), (0, 2), (0, 3), (1, 1), (1, 3), (3, 3)]

root = tk.Tk()
w = tk.Canvas(root, bg='white', width=500, height=500)

w.grid(column=0, row=0, sticky=tk.NSEW)


def paint_point(x, y):
    w.create_oval(x - 3, y - 3, x + 3, y + 3, fill="black")


print(l)
l = [(round(x * 10) + 250, round(y * 10) + 250) for x, y in l]
print(l)
for s1 in l:
    paint_point(*s1)

# for i, j in link:
#     w.create_line(l[i], l[j])

tk.mainloop()
