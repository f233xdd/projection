from time import perf_counter


class Timer:
    def __init__(self, e: int = 3) -> None:
        self._t1 = 0
        self._t2 = 0
        self._t3 = 0
        self._break = 0
        self._is_started = False
        self._is_paused = False
        self._dt = 0
        self._e = e
    def __r(self, x): return round(x, self._e)

    def start(self):
        if self._is_paused:
            self._break += perf_counter() - self._t3
            self._is_paused = False
        if not self._is_started:
            self._t1 = perf_counter()
            self._is_started = True

    def pause(self):
        if not self._is_paused:
            self._t3 = perf_counter()
            self._is_paused = True

    def tick(self):
        if self._is_started:
            self._t2 = perf_counter()
            if self._is_paused:
                dt = self.__r(self._t3 - self._t1 - self._break)
                self._t3 = self._t2
            else:
                dt = self.__r(self._t2 - self._t1 - self._break)
            self._t1 = self._t2
            self._break = 0
            self._dt = dt
            return dt

    def reset(self):
        self._t1 = 0
        self._t2 = 0
        self._t3 = 0
        self._break = 0
        self._is_started = False
        self._is_paused = False
        self._dt = 0
    @property
    def is_started(self): return self._is_started
    @property
    def is_paused(self): return self._is_paused
    @property
    def saved(self): return self._dt


if __name__ == "__main__":
    from time import sleep as s
    res = []
    t = Timer()

    t.start()
    s(1)
    res.append(t.tick())
    s(1)
    t.pause()
    s(1)
    res.append(t.tick())
    s(1)
    t.start()
    s(1)
    res.append(t.tick())
    s(1)
    res.append(t.tick())
    s(1)
    t.pause()
    s(1)
    res.append(t.tick())
    s(1)
    res.append(t.tick())
    s(1)
    t.start()
    s(1)
    t.reset()
    print(res)
