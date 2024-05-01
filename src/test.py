from geo2d import *
from project import *


p = [Point(-1.3873, 3.86814),
     Point(-1.20899, 0.80864),
     Point(0.17469, 1.97528),
     Point(2.56222, -1.76881),
     Point(5.73656, 3.65741),
     Point(6.68614, -0.79209),
     Point(3.97304, -3.04397),
     Point(1.80255, -5.40437),
     Point(1.34133, -3.15249),
     Point(-0.23227, -1.4975),
     Point(-1.10047, -5.29584),
     Point(-0.74776, 0.02184),
     Point(-3.75931, -4.23773),
     Point(-5.59326, -3.58546)
     ]

l = []
for i in range(len(p)-1):
    l.append(Line(p[i], p[i+1]))
l.append(Line(p[0], p[-1]))

p1 = Point(-5.77109, -2.78984)
print(is_in_polygon(p1, l))
