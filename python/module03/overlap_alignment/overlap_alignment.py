def overlap_alignment(match, mismatch, indel, v, w):
    n, m = len(v), len(w)
    # Initialize DP table
    dp = [[0] * (m + 1) for _ in range(n + 1)]
    backtrack = [[None] * (m + 1) for _ in range(n + 1)]

    # Fill DP table
    for i in range(1, n + 1):
        dp[i][0] = -i * indel
        backtrack[i][0] = "up"
    for j in range(1, m + 1):
        dp[0][j] = -j * indel
        backtrack[0][j] = "left"

    for i in range(1, n + 1):
        for j in range(1, m + 1):
            score_sub = dp[i-1][j-1] + (match if v[i-1] == w[j-1] else -mismatch)
            score_del = dp[i-1][j] - indel
            score_ins = dp[i][j-1] - indel
            dp[i][j] = max(score_sub, score_del, score_ins)

            if dp[i][j] == score_sub:
                backtrack[i][j] = "diag"
            elif dp[i][j] == score_del:
                backtrack[i][j] = "up"
            else:
                backtrack[i][j] = "left"

    # Find best overlap: suffix of v (last row) vs prefix of w (last col)
    max_score = float("-inf")
    max_pos = (0, 0)

    # last row
    for j in range(m + 1):
        if dp[n][j] > max_score:
            max_score = dp[n][j]
            max_pos = (n, j)

    # last column
    for i in range(n + 1):
        if dp[i][m] > max_score:
            max_score = dp[i][m]
            max_pos = (i, m)

    # Traceback
    i, j = max_pos
    v_aln, w_aln = "", ""
    while i > 0 and j > 0:
        if backtrack[i][j] == "diag":
            v_aln = v[i-1] + v_aln
            w_aln = w[j-1] + w_aln
            i -= 1
            j -= 1
        elif backtrack[i][j] == "up":
            v_aln = v[i-1] + v_aln
            w_aln = "-" + w_aln
            i -= 1
        elif backtrack[i][j] == "left":
            v_aln = "-" + v_aln
            w_aln = w[j-1] + w_aln
            j -= 1
        else:
            break

    return max_score, v_aln, w_aln


# --- Test with Sample Input ---
match, mismatch, indel = 1, 1, 2
v = "GAGA"
w = "GAT"

score, v_aln, w_aln = overlap_alignment(match, mismatch, indel, v, w)
print(score)
print(v_aln)
print(w_aln)