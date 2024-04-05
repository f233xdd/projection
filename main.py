import cal
import tkinter as tk
from tkinter import ttk


s = [(0,0,0), (0,1,0), (1,0,0), (1,1,0), (0,0,1), (0,1,1), (1,0,1), (1,1,1)]
p = (-4, -3, 7)
l = [cal.multi_project(p, s1) for s1 in s]
link = [(0, 1), (1, 7)]

root = tk.Tk()
w = tk.Canvas(root, bg='white', width=500, height=500)

w.grid(column=0, row=0, sticky=tk.NSEW)

def paint_point(x, y):
    w.create_oval(x-3, y-3, x+3, y+3, fill = "black")

l = [(round(x*50)+100, round(y*50)+100) for x, y in l]
print(l)
for s1 in l:
    paint_point(*s1)

for i, j in link:
    w.create_line(l[i], l[j])

tk.mainloop()
