import pathlib

input_file = pathlib.Path("input")

def to_number(line: str) -> int | None:
    if line.startswith("one"):
        return 1
    elif line.startswith("two"):
        return 2
    elif line.startswith("three"):
        return 3
    elif line.startswith("four"):
        return 4
    elif line.startswith("five"):
        return 5
    elif line.startswith("six"):
        return 6
    elif line.startswith("seven"):
        return 7
    elif line.startswith("eight"):
        return 8
    elif line.startswith("nine"):
        return 9

total = 0
for line in input_file.open():
    first_digit = None
    last_digit = None
    for i, c in enumerate(line):
        if c.isdigit():
            if first_digit is None:
                first_digit = int(c)
            last_digit = int(c)
        elif (val := to_number(line[i:])) is not None:
            if first_digit is None:
                first_digit = val
            last_digit = val
            
    if first_digit and last_digit:
        combined_digits = first_digit * 10 + last_digit
        total += combined_digits

print(total)
