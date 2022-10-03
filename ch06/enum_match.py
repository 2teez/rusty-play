
from enum import Enum


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


if __name__ == '__main__':
    main()
