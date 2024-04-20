import tkinter as tk

root = tk.Tk()
canvas = tk.Canvas(root, width=400, height=400)
canvas.pack()

# 绘制实线

# 绘制虚线
canvas.create_line(60, 100, 60, 200, dash=(2, 4))

root.mainloop()
