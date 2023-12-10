from pathlib import Path
import re

REGEX = r"^[.\d]+$"


with Path("input").open() as file:
    lines = file.readlines()

    sum = 0
    for i, line in enumerate(lines):
        if i == 0:
            prev_line = line
            next_line = lines[i + 1]
        elif i == len(lines) - 1:
            next_line = line
            prev_line = lines[i - 1]
        else:
            prev_line = lines[i - 1]
            next_line = lines[i + 1]

        indices = [m.span() for m in re.finditer(r"\d+", line)]
        for start, end in indices:
            if start > 0:
                start_wrap = start - 1
            else:
                start_wrap = start

            surround_symbol_checks = [bool(re.match(REGEX, curr_line[start_wrap : end + 1])) for curr_line in [prev_line, line, next_line]]
            if not all(surround_symbol_checks):
                part_num = "".join(line[start:end])
                sum += int(part_num)

print(sum)
