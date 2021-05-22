def string_with_arrows(text: str, pos_start, pos_end):
    result = text.splitlines()
    if pos_start.ln == pos_end.ln:
        result = result[pos_end.ln] + '\n'
        result += ' ' * pos_start.col + '^' * (pos_end.col - pos_start.col)
        return result

    out = result[pos_start.ln] + '\n'
    out += ' ' * pos_start.col + '^' * (len(out) - pos_start.col) + '\n'

    for i in range(pos_start.ln + 1, pos_end.ln):
        out += result[i] + '\n' + '^' * len(result[i]) + '\n'

    out += result[pos_end.ln] + '\n' + '^' * pos_end.col

    return out
