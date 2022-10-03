
from collections import namedtuple
from enum import Enum
import io

persons = namedtuple('persons', ['name', 'age'])


class Direction(Enum):
    NORTH = "North",
    SOUTH = "South",
    EAST = "East",
    WEST = "West",


def main() -> None:
    value = Direction.SOUTH
    match value:
        case (Direction.NORTH):
            value = "North"
        case (Direction.SOUTH):
            value = "South"
        case (Direction.East):
            value = "East"
        case (Direction.West):
            value = "West"
    print(f'Direction is: {value}')

    for n in range(-2, 6):
        match n:
            case 0: print(n, "is zero")
            case 1: print(n, "is one")
            case _ if n < 0: print(n, "is negative")
            case default: print(n, "is plural")

    ppl = [persons("timothy", 23), persons("tolu", 18)]
    print(ppl)

    print(", ".join([str(p.age) for p in ppl]))


if __name__ == '__main__':
    main()
