import cal
import tkinter as tk
from tkinter import ttk


s = [(0,0,0), (0,1,0), (1,0,0), (1,1,0), (0,0,1), (0,1,1), (1,0,1), (1,1,1)]
p = (3,3,3)
l = [cal.multi_project(p, s1) for s1 in s]


root = tk.Tk()
w = tk.Canvas(root, bg='white')

w.grid(column=0, row=0, sticky=tk.NSEW)

def paint_point(x, y):
        w.create_oval(x-1, y-1, x+1, y+1, fill = "black")
        
for s1 in l:
    paint_point(s1)

tk.mainloop()
