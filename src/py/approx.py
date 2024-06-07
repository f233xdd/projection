ACCURACY = 5


def round(x):
    return round(x, ACCURACY)


def approximate(x, y):
    return True if abs(x - y) <= 10 ** (-ACCURACY + 1) else False


class ApproxFloat(float):
    def __approximate(self, v1: float, v2: float) -> bool:
        v = abs(v1 - v2)
        if isinstance(v, ApproxFloat):
            return True if super(ApproxFloat, v).__le__(10 ** (-ACCURACY + 1)) else False
        else:
            return True if v <= 10 ** (-ACCURACY + 1) else False
    
    def __eq__(self, value: float) -> bool:
        return self.__approximate(self, value)

    def __le__(self, value: float) -> bool:
        return super().__lt__(value) if self != value else True

    def __ge__(self, value: float) -> bool:
        return super().__gt__(value) if self != value else True

    def __ne__(self, value: float) -> bool:
        return False if self == value else True

    def __add__(self, value: float) -> float:
        return ApproxFloat(super().__add__(value))
    
    def __radd__(self, value: float) -> float:
        return ApproxFloat(super().__radd__(value))
    
    def __sub__(self, value: float) -> float:
        return ApproxFloat(super().__sub__(value))
    
    def __rsub__(self, value: float) -> float:
        return ApproxFloat(super().__rsub__(value))

    def __mul__(self, value: float) -> float:
        return ApproxFloat(super().__mul__(value))
    
    def __rmul__(self, value: float) -> float:
        return ApproxFloat(super().__rmul__(value))

    def __truediv__(self, value: float) -> float:
        return ApproxFloat(super().__truediv__(value))
    
    def __rtruediv__(self, value: float) -> float:
        return ApproxFloat(super().__rtruediv__(value))
    
    def __floordiv__(self, value: float) -> float:
        return ApproxFloat(super().__floordiv__(value))
    
    def __rfloordiv__(self, value: float) -> float:
        return ApproxFloat(super().__rfloordiv__(value))
    
    def __mod__(self, value: float) -> float:
        return ApproxFloat(super().__mod__(value))
    
    def __rmod__(self, value: float) -> float:
        return ApproxFloat(super().__rmod__(value))
    
    def __divmod__(self, value: float) -> tuple[float, float]:
        return ApproxFloat(super().__divmod__(value))
    
    def __rdivmod__(self, value: float) -> tuple[float, float]:
        return ApproxFloat(super().__rdivmod__(value))
    
    def __pow__(self, value: int, mod: None = None) -> float:
        return ApproxFloat(super().__pow__(value, mod))
    
    def __rpow__(self, value: float, mod: None = None, /) -> float:
        return ApproxFloat(super().__rpow__(value, mod))

    def __abs__(self) -> float:
        return ApproxFloat(super().__abs__())

