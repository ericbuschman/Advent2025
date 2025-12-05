import sys


def load_file_content(file_name):
    try:
        with open(file_name, "r") as file:
            content = file.read()
        return content
    except FileNotFoundError:
        print(f"Error: file '{file_name}' not found.")
        return None
    except Exception as e:
        print(f"Error: {e}")
        return None


def joltage_a(line):
    line = list(line)
    largest = list()
    largest.append(line[0])
    largest.append(line[1])
    for i in range(1, len(line)):
        if int(line[i]) > int(largest[0]) and i < len(line) - 1:
            largest[0] = line[i]
            largest[1] = line[i + 1]
        elif int(line[i]) > int(largest[1]):
            largest[1] = line[i]
    return largest


if __name__ == "__main__":
    contents = """987654321111111
811111111111119
234234234234278
818181911112111"""

    if len(sys.argv) > 1:
        file_to_load = sys.argv[1]
        contents = load_file_content(file_to_load)

    sum = 0
    if contents:
        for line in contents.splitlines():
            res = joltage_a(line)
            res = "".join(res)
            print(res)
            sum += int(res)
        print(sum)
