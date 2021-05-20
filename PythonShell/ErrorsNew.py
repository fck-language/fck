from Arrows import string_with_arrows
from textwrap import wrap
from Bases import wrap_length
from random import randint

ET_IllegalChar = "IllegalChar"
ET_ExpectedChar = "ExpectedChar"
ET_ExpectedExpr = "ExpectedExpr"
ET_InvalidSyntax = "InvalidSyntax"
ET_IllegalOperation = "IllegalOperation"
ET_IllegalValue = "IllegalValue"
ET_IllegalVariableAssignment = "IllegalVariableAssignment"
ET_ExpectedIdentifier = "ExpectedIdentifier"
ET_ExpectedTypeIdentifier = "ExpectedTypeIdentifier"
ET_ExpectedKeyword = "ExpectedKeyword"
ET_UnknownIdentifier = "UnknownIdentifier"

AET_TooArgumentError = "TooArgumentError"
AET_AttributeTypeError = "AttributeTypeError"
AET_IllegalArgumentValue = "IllegalArgumentValue"
AET_UnknownAttributeError = "UnknownAttributeError"

errorFormatting = {ET_IllegalChar: "",
                   ET_ExpectedChar: "",
                   ET_ExpectedExpr: "",
                   ET_InvalidSyntax: "",
                   ET_IllegalOperation: "",
                   ET_IllegalValue: "",
                   ET_IllegalVariableAssignment: "",
                   ET_ExpectedIdentifier: "",
                   ET_ExpectedTypeIdentifier: "",
                   ET_ExpectedKeyword: "",
                   ET_UnknownIdentifier: "",
                   AET_IllegalArgumentValue: "\n{arg_explain}",
                   AET_AttributeTypeError: "\n{arg_explain}",
                   AET_TooArgumentError: "\n{arg_explain}",
                   AET_UnknownAttributeError: "\n{arg_explain}"}

errorNames = {ET_IllegalChar: "Illegal character",
              ET_ExpectedChar: "Expected character",
              ET_ExpectedExpr: "Expected expression",
              ET_InvalidSyntax: "Invalid syntax",
              ET_IllegalOperation: "Illegal operation",
              ET_IllegalValue: "Illegal value",
              ET_IllegalVariableAssignment: "Illegal variable value",
              ET_ExpectedIdentifier: "Expected Identifier",
              ET_ExpectedTypeIdentifier: "Expected variable type",
              ET_ExpectedKeyword: "Expected keyword",
              ET_UnknownIdentifier: "Unknown identifier",
              AET_UnknownAttributeError: "Unknown Attribute Error",
              AET_TooArgumentError: "Too Argument Error",
              AET_IllegalArgumentValue: "Illegal Argument Value",
              AET_AttributeTypeError: "Attribute Type Error"}


class ErrorNew:
    def __init__(self, error_name, details, pos_start, pos_end, context, **kwargs):
        self.pos_start = pos_start
        self.pos_end = pos_end
        self.context = context
        self.error_name = error_name
        self.details = details
        self.__dict__.update(kwargs)

    def generate_traceback(self):
        result = ''
        pos = self.pos_start
        ctx = self.context

        while ctx:
            result = f'  Line {pos.ln}, in {ctx.display_name}\n' + result
            pos = ctx.parent_entry_pos
            ctx = ctx.parent

        return f'Traceback (most recent call):\n{result[:-1]}\n' \
               f'{string_with_arrows(self.context.ftxt, self.pos_start, self.pos_end)}'

    def as_string(self):
        def wrap_(text: str):
            return '\n'.join(wrap(text, wrap_length))
        title = f'\nError:\033[35m {errorNames[self.error_name]}\033[0m\n'
        body = '\n'.join([wrap_(self.details),
                          errorFormatting[self.error_name].format(**dict(self.__dict__.items())),
                          self.generate_traceback()])
        longest = max([len(i) for i in body.split('\n')])
        bounding = '\033[31m' + "*" * longest
        return bounding + title + (f'{body}\n' if body else '') + bounding + '\033[0m'
