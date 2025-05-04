def longestCommonSubsequence(text1: str, text2: str) -> tuple[int, str] | None:
    if not text1 or not text2:
        return None

    m, n = len(text1), len(text2)
    dp_table = [[0] * (n + 1)] * (m + 1)  # dp table filled with zeros

    for i in range(1, m + 1):
        for j in range(1, n + 1):
            if text1[i - 1] == text2[j - 1]:
                dp_table[i][j] = dp_table[i - 1][j - 1] + 1
            else:
                dp_table[i][j] = max(dp_table[i - 1][j], dp_table[i][j - 1])

    index = dp_table[m][n]
    if index == 0:
        return None

    # reconstruct the subsequence using backtracking
    lcs = [""] * (index + 1)

    i = m
    j = n
    while i > 0 and j > 0:
        if text1[i - 1] == text2[j - 1]:
            lcs[index - 1] = text1[i - 1]
            i -= 1
            j -= 1
            index -= 1
        elif dp_table[i - 1][j] > dp_table[i][j - 1]:
            i -= 1
        else:
            j -= 1

    return dp_table[m][n], "".join(lcs)

