import random
import string
import sys


def generate_random_string(low: int, high: int):
    assert low <= high
    length = random.randint(low, high)
    random_string = "".join(random.choices(string.ascii_letters, k=length))
    return random_string


if __name__ == "__main__":
    USAGE = (
        f"Usage: {sys.argv[0]} <OUTPUT_FILE> <LENGTH_MIN> <LENGTH_MAX> <NUM_OF_STRINGS>"
    )

    if len(sys.argv) != 5:
        print(USAGE, file=sys.stderr)
        exit(1)

    output_file = sys.argv[1]

    try:
        low = int(sys.argv[2])
        high = int(sys.argv[3])
        num = int(sys.argv[4])
    except ValueError:
        print("Error: please use valid numbers.")
        print(USAGE)
        exit(1)

    with open(output_file, "w") as file:
        for i in range(num):
            text1 = generate_random_string(low, high)
            text2 = generate_random_string(low, high)
            print(f"{text1} {text2}", file=file)
