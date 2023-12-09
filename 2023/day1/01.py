import pathlib

input_file = pathlib.Path("input")

total = 0
for line in input_file.open():
    first_digit = None
    last_digit = None
    for c in line:
        if c.isdigit():
            if first_digit is None:
                first_digit = int(c)
            last_digit = int(c)
    if first_digit and last_digit:
        combined_digits = first_digit * 10 + last_digit
        total += combined_digits

print(total)
