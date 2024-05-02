from time import perf_counter

class Ticker:
    def __init__(self) -> None:
        self._t1 = 0
        self._t2 = 0
        self._t3 = 0
        self._break = 0
        self._is_started = False
        self._is_paused = False
    def start(self):
        if self._is_paused:
            self._break += perf_counter() - self._t3
            self._is_paused = False
        elif not self._is_started:
            self._t1 = perf_counter()
    def pause(self):
        if not self._is_paused:
            self._t3 = perf_counter()
            self._is_paused = True
    def tick(self):
        if self._is_started:
            self._t2 = perf_counter()
            if self._is_paused:
                dt = self._t3 - self._t1 - self._break
                self._t3 = self._t2
            else:
                dt = self._t2 - self._t1 - self._break
            self._t1 = self._t2
            self._break = 0
            return dt
    def reset(self):
        self._t1 = 0
        self._t2 = 0
        self._t3 = 0
        self._break = 0
        self._is_started = False
        self._is_paused = False
    @property
    def is_started(self): return self._is_started
    @property
    def is_paused(self): return self._is_paused
