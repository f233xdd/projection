from project import single_project


s = [(3, 3, 3), (3, 4, 3), (4, 3, 3), (4, 4, 3),
     (3, 3, 4), (3, 4, 4), (4, 3, 4), (4, 4, 4)]
l = [single_project(s1, 3, 45, 0) for s1 in s]
print(l)