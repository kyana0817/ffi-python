class Simple:
    a: int
    b: int

class ManualSimple:
    """
    A simple struct with manual properties.
    """
    @property
    def a(self) -> int: ...
    """
    A property representing the first integer.
    """

    @property
    def b(self) -> int: ...


def sum_as_string (a: int, b: int) -> str: ...
def simple_struct() -> Simple: ...
def manula_simple_struct() -> ManualSimple: ...