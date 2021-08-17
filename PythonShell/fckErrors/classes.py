from textwrap import wrap
from re import match

from . import wrap_length
from .explanations import *


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


def highlight(e: str) -> str:
    split = e.splitlines()
    match_ = False
    for line_num, line in enumerate(split[1:]):
        match_ = match(r'( )*\^+', line)
        if match_:
            caret_line = line_num
            break
        else:
            raise Exception('Error and warning examples must include highlighted section where the error or warning '
                            'originates from')
    if not match_:
        return e
    match_ = (match_.regs[1][0] + 1, match_.regs[0][1])
    split[caret_line] = split[caret_line][:match_[0]] + '\033[35m' + \
                        split[caret_line][match_[0]:match_[1]] + '\033[0m' + \
                        split[caret_line][match_[1]:]

    return '\n'.join(split)


def get_explain(eow_code: int, error: bool = True):
    name = list(error_explain.keys())[eow_code]
    explain = list((error_explain if error else wrn_explain).values())[eow_code]
    eow_code += 1
    out = f'\033[4;34m{"Error" if error else "Warning"} number {eow_code} ' \
          f'({"E" if error else "W"}{str(eow_code).rjust(3, "0")}: {name})\n\033[0m'
    out += "\n".join(wrap(explain[0], wrap_length))
    out += f'\n\n\033[34mExample:\033[0m\n{highlight(explain[1])}\n'
    out += "\n".join(wrap(explain[2], wrap_length))
    return out


def wrap_(text: str):
    split = text.splitlines()
    return '\n'.join(['\n'.join(wrap(i, wrap_length)) for i in split])


class Base:
    def __init__(self, error_name, details, pos_start, pos_end, context, title, **kwargs):
        self.pos_start = pos_start
        self.pos_end = pos_end
        self.context = context
        self.error_name = error_name
        self.details = details
        self.title = title
        self.__dict__.update(kwargs)

    def generate_traceback(self):
        result = ''
        pos = self.pos_start
        ctx = self.context

        while ctx:
            result = f'  Line {pos.ln + 1}, in {ctx.display_name}\n' + result
            pos = ctx.parent_entry_pos
            ctx = ctx.parent

        return f'Traceback (most recent call):\n{result[:-1]}\n' \
               f'{string_with_arrows(self.context.ftxt, self.pos_start, self.pos_end)}'

    def __str__(self, header: bool = True):
        names, format_str, explain, col = (error_names, error_format, error_explain, 1) \
            if self.title == "Error" else (wrn_names, wrn_format, wrn_explain, 3)

        number = str(list(explain.keys()).index(self.error_name) + 1).rjust(3, "0")
        title = f'\n{self.title}:\033[35m {names[self.error_name]} ({self.title[0]}{number})\033[0m\n'
        body = wrap_(format_str[self.error_name].format(traceback=self.generate_traceback(),
                                                        **dict(self.__dict__)))
        longest = max([len(i) for i in
                       f'{self.title}: {names[self.error_name]} ({self.title[0]}{number})\n{body}'.split('\n')])
        bounding = f'\033[3{col}m' + "*" * longest * header

        return bounding + title + (f'{body}\n' if body else '') + \
               f'\nUse \'fck -{self.title[0].lower()} {self.title[0]}{number}\' for more details\n' + \
               bounding + '\033[0m'


class Error(Base):
    def __init__(self, error_name, details, pos_start, pos_end, context, **kwargs):
        super().__init__(error_name, details, pos_start, pos_end, context, "Error", **kwargs)


class Warning(Base):
    def __init__(self, error_name, pos_start, pos_end, context, **kwargs):
        super().__init__(error_name, "", pos_start, pos_end, context, "Warning", **kwargs)
        print(str(self))


__all__ = ["Error", "Warning"]
