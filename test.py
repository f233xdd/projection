from cal import *
from debug import *


def f():
    for __ in range(10):
        multi_project((-400, 600, 100), (348, -218, 389))


runtime(f, times=1000)
