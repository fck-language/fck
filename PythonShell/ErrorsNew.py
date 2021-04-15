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

AET_TooArgumentError = "TooArgumentError"
AET_AttributeTypeError = "AttributeTypeError"
AET_IllegalArgumentValue = "IllegalArgumentValue"

errorFormatting = {ET_IllegalChar: "{name}: {details}",
                   ET_ExpectedChar: "{name}: {details}",
                   ET_ExpectedExpr: "{name}: {details}",
                   ET_InvalidSyntax: "{name}: {details}",
                   ET_IllegalOperation: "{name}: {details}",
                   ET_IllegalValue: "{name}: {details}",
                   ET_IllegalVariableAssignment: "{name}: {details}"}

errorNames = {ET_IllegalChar: "Illegal character",
              ET_ExpectedChar: "Expected character",
              ET_ExpectedExpr: "Expected expression",
              ET_InvalidSyntax: "Invalid syntax",
              ET_IllegalOperation: "Illegal operation",
              ET_IllegalValue: "Illegal value",
              ET_IllegalVariableAssignment: "Illegal variable value"}


class Error:
    def __init__(self, pos_start, pos_end, error_name, details):
        self.pos_start = pos_start
        self.pos_end = pos_end
        self.error_name = error_name
        self.details = details

    def as_string(self):
        if self.details:
            name = errorFormatting[self.error_name].format(name=errorNames[self.error_name], details=self.details)
        else:
            name = errorNames[self.error_name]
        return '\n'.join(['\n'.join(wrap(name, wrap_length)),
                          '\n'.join(wrap(f'File {self.pos_start.fn}, line {self.pos_start.ln + 1}', wrap_length)),
                          f'{string_with_arrows(self.pos_start.ftxt, self.pos_start, self.pos_end)}'])
