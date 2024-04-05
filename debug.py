import multiprocessing
import queue
import threading
import time


class CountTimeProcess(multiprocessing.Process):
    _sign_time = time.perf_counter

    def __init__(self, target, name, args, kwargs):
        super().__init__(group=None, target=target, name=name, args=args, kwargs=kwargs)
        self.delta_t = None
        self.queue = multiprocessing.Queue()

    @classmethod
    def set_counter(cls, func):
        cls._sign_time = func

    def run(self) -> None:
        t1 = self._sign_time()
        super().run()
        t2 = self._sign_time()
        self.delta_t = t2 - t1
        self.queue.put(self.delta_t)

    def get_data(self):
        return self.queue.get()


class CountTimeThread(threading.Thread):
    _sign_time = time.perf_counter

    def __init__(self, target, name, args, kwargs):
        super().__init__(group=None, target=target, name=name, args=args, kwargs=kwargs)
        self.delta_t = None
        self.queue = queue.Queue()

    @classmethod
    def set_counter(cls, func):
        cls._sign_time = func

    def run(self) -> None:
        t1 = self._sign_time()
        super().run()
        t2 = self._sign_time()
        self.delta_t = t2 - t1
        self.queue.put(self.delta_t)

    def get_data(self):
        return self.queue.get()


def runtime(func, *args, times=8, mode="Thread", count_sleep=True, **kwargs):
    if mode == "Thread":
        run = CountTimeThread
    elif mode == "Process":
        run = CountTimeProcess
    else:
        raise AttributeError

    if not count_sleep:
        CountTimeProcess.set_counter(time.process_time)

    threads = [run(target=func, name=f"Thread-{i}", args=args, kwargs=kwargs) for i in range(times)]
    for thd in threads:
        thd.daemon = True
        thd.start()
    for thd in threads:
        thd.join()

    total = 0
    for thread in threads:
        total += thread.get_data()

    average = total / times
    print(f"Average run time: {average}")
