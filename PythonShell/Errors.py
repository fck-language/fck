from Arrows import string_with_arrows
from random import randint
from ErrorParser import *
from textwrap import wrap

err_warn = get_err_warns()


class Error:
    def __init__(self, pos_start, pos_end, error_name, details=""):
        self.pos_start = pos_start
        self.pos_end = pos_end
        self.error_name = error_name
        self.details = details

    def as_string(self):
        result = "\n".join(wrap(f'{self.error_name}: {self.details}\n'))
        result += f'File {self.pos_start.fn}, line {self.pos_start.ln + 1}'
        result += '\n' + string_with_arrows(self.pos_start.ftxt, self.pos_start, self.pos_end)
        return result


class IllegalCharError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, 'Illegal Character', details)


class ExpectedCharError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, 'Expected Character', details)


class InvalidSyntaxError(Error):
    def __init__(self, pos_start, pos_end, details=''):
        super().__init__(pos_start, pos_end, 'Invalid Syntax', details)


class IllegalOperationError(Error):
    def __init__(self, pos_start, pos_end):
        super().__init__(pos_start, pos_end, 'Illegal operation. Basically, no can use operation here')


class IllegalValueError(Error):
    def __init__(self, pos_start, pos_end, details=None):
        super().__init__(pos_start, pos_end, 'Illegal value' + f':\n{details}' if details else 'Illegal value')


class RTError(Error):
    def __init__(self, pos_start, pos_end, details, context):
        super().__init__(pos_start, pos_end, 'Runtime Error', details)
        self.context = context

    def as_string(self):
        result = self.generate_traceback()
        result += f'{self.error_name}: {self.details}'
        result += '\n' + string_with_arrows(self.pos_start.ftxt, self.pos_start, self.pos_end)
        return result

    def generate_traceback(self):
        result = ''
        pos = self.pos_start
        ctx = self.context

        while ctx:
            result = f'  File {pos.fn}, line {str(pos.ln + 1)}, in {ctx.display_name}\n' + result
            pos = ctx.parent_entry_pos
            ctx = ctx.parent

        return 'Traceback (most recent call last):\n' + result


class NonBreakError:
    def __init__(self, pos_start, pos_end, context, error_name):
        self.pos_start = pos_start
        self.pos_end = pos_end
        self.context = context
        self.error_name = err_warn_names[error_name]
        self.value = err_warn[error_name]

    def generate_traceback(self):
        result = ''
        pos = self.pos_start
        ctx = self.context

        while ctx:
            result = f'  File {pos.fn}, line {str(pos.ln + 1)}, in {ctx.display_name}\n' + result
            pos = ctx.parent_entry_pos
            ctx = ctx.parent

        return f'Traceback (most recent call):\n{result[:-1]}\n' \
               f'{string_with_arrows(self.pos_start.ftxt, self.pos_start, self.pos_end)}'

    def print_method(self):
        value = self.value[randint(0, len(self.value) - 1)]
        out = "\n".join(wrap(f'{self.error_name}: {value}')) + f"\n{self.generate_traceback()}"
        longest = max([len(i) for i in out.split("\n")])
        print("*" * longest + "\nWarning:\n" + out + "\n" + "*" * longest)
