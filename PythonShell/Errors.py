from Arrows import string_with_arrows
from ErrorParser import *
from Bases import wrap_length

from textwrap import wrap
from random import randint

err_warn = get_err_warns()


class Error:
    def __init__(self, pos_start, pos_end, error_name, details=""):
        self.pos_start = pos_start
        self.pos_end = pos_end
        self.error_name = error_name
        self.details = details

    def as_string(self):
        return '\n'.join(['\n'.join(wrap(f'{self.error_name}: {self.details}' if self.details else self.error_name)),
                          '\n'.join(wrap(f'File {self.pos_start.fn}, line {self.pos_start.ln + 1}')),
                          f'{string_with_arrows(self.pos_start.ftxt, self.pos_start, self.pos_end)}'])


class IllegalCharError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, 'Illegal character', details)


class ExpectedCharError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, 'Expected character', details)


class ExpectedExprError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, 'Expected expression', details)



class InvalidSyntaxError(Error):
    def __init__(self, pos_start, pos_end, details=''):
        super().__init__(pos_start, pos_end, 'Invalid syntax', details)


class IllegalOperationError(Error):
    def __init__(self, pos_start, pos_end):
        super().__init__(pos_start, pos_end, 'Illegal operation. Basically, no can use operation here')


class IllegalValueError(Error):
    def __init__(self, pos_start, pos_end, details=None):
        super().__init__(pos_start, pos_end, 'Illegal value', details)


class IllegalVariableAssignment(Error):
    def __init__(self, pos_start, pos_end, details=None):
        super().__init__(pos_start, pos_end, 'Illegal variable value', details)


class ArgumentError(Error):
    def __init__(self, pos_start, pos_end, details, arg_explain):
        super().__init__(pos_start, pos_end, details)
        self.arg_explain = arg_explain

    def as_string(self):
        return '\n'.join(['\n'.join(wrap(f'{self.error_name}: {self.details}')),
                          self.arg_explain,
                          '\n'.join(wrap(f'File {self.pos_start.fn}, line {self.pos_start.ln + 1}')),
                          f'{string_with_arrows(self.pos_start.ftxt, self.pos_start, self.pos_end)}'])


class TooArgumentError(ArgumentError):
    def __init__(self, pos_start, pos_end, details, arg_explain):
        super().__init__(pos_start, pos_end, f'Argument error: {details}', arg_explain)


class AttributeTypeError(ArgumentError):
    def __init__(self, pos_start, pos_end, details, arg_explain):
        super().__init__(pos_start, pos_end, f'Argument type error: {details}', arg_explain)


class IllegalAttributeValue(ArgumentError):
    def __init__(self, pos_start, pos_end, details, arg_explain):
        super().__init__(pos_start, pos_end, f'Illegal argument value: {details}', arg_explain)


class UnknownAttributeError(Error):
    def __init__(self, pos_start, pos_end, attribute, trace):
        super().__init__(pos_start, pos_end, f'Attribute \'{attribute}\' does not exist for \'{trace}\'')


class RTError(Error):
    def __init__(self, pos_start, pos_end, details, context):
        super().__init__(pos_start, pos_end, 'Runtime error', details)
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
        print("*" * longest + "\nWarning:\n" + out + "\n" + "*" * wrap_length)
