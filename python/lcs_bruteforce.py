def brute_force_longest_common_subsequence(text1: str, text2: str) -> str | None:
    text1_subsequences = generate_subsequences(text1)
    text2_subsequences = generate_subsequences(text2)

    longest_subsequence = ""

    for subsequence1 in text1_subsequences:
        for subsequence2 in text2_subsequences:
            if subsequence1 == subsequence2 and len(subsequence1) > len(
                longest_subsequence
            ):
                longest_subsequence = subsequence1

    return longest_subsequence if len(longest_subsequence) else None


def generate_subsequences(text: str) -> list[str]:
    def inner_generate_subsequences(text: str, pos: int, current: str) -> list[str]:
        if pos == len(text):
            return [current]

        # option 1: skip current character
        without_curr_char = inner_generate_subsequences(text, pos + 1, current)

        # option 2: include current character
        with_curr_char = inner_generate_subsequences(text, pos + 1, current + text[pos])

        return without_curr_char + with_curr_char

    return inner_generate_subsequences(text, 0, "")

