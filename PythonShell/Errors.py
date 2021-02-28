from Arrows import *


class Context:
    def __init__(self, display_name, parent=None, parent_entry_pos=None):
        self.display_name = display_name
        self.parent = parent
        self.parent_entry_pos = parent_entry_pos
        self.symbol_table = None


class SymbolTable:
    def __init__(self, parent=None):
        self.symbols = {}
        self.parent = parent

    def get(self, name):
        value = self.symbols.get(name, None)
        if value is None and self.parent:
            return self.parent.get(name)
        return value

    def set(self, name, value):
        self.symbols[name] = value

    def remove(self, name):
        del self.symbols[name]


class Position:
    def __init__(self, idx, ln, col, fn, ftxt):
        self.idx = idx
        self.ln = ln
        self.col = col
        self.fn = fn
        self.ftxt = ftxt

    def advance(self, current_char=None):
        self.idx += 1
        self.col += 1

        if current_char == '\n':
            self.ln += 1
            self.col = 0

        return self

    def copy(self):
        return Position(self.idx, self.ln, self.col, self.fn, self.ftxt)


class Error:
    def __init__(self, pos_start: Position, pos_end: Position, error_name, details):
        self.error_name = error_name
        self.details = details
        self.pos_start = pos_start
        self.pos_end = pos_end

    def as_string(self) -> str:
        return f'{self.error_name}: {self.details}\n' \
               f'File {self.pos_start.fn}, line {self.pos_start.ln + 1}\n' \
               f'{string_with_arrows(self.pos_start.ftxt, self.pos_start, self.pos_end)}'


class IllegalCharError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, 'Illegal character (Bad bad character)', details)


class InvalidSyntaxError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, 'Invalid syntax (You no use right)', details)


class ExpectedCharError(Error):
    def __init__(self, pos_start, pos_end, details):
        super().__init__(pos_start, pos_end, 'Expected character, did you forgets?', details)


class IllegalOperationError(Error):
    def __init__(self, pos_start, pos_end):
        super().__init__(pos_start, pos_end, 'Illegal operation. Basically, no can use operation here', '')


class RTError(Error):
    def __init__(self, pos_start, pos_end, details, context):
        super().__init__(pos_start, pos_end, 'Runtime error (oh dear...)', details)
        self.context = context

    def as_string(self):
        return f'{self.generate_traceback()}' \
               f'{self.error_name}: {self.details}\n' \
               f'{string_with_arrows(self.pos_start.ftxt, self.pos_start, self.pos_end)}'

    def generate_traceback(self):
        out = ""
        pos = self.pos_start
        ctx = self.context

        while ctx:
            out += f'   File {pos.fn}, line {str(pos.ln + 1)}, in {ctx.display_name}\n'
            pos = ctx.parent_entry_pos
            ctx = ctx.parent

        return f'Traceback from where it done fucked up last time I checked:\n{out}'
