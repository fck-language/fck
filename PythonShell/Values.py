import sys
from math import sqrt, sin, cos, floor, ceil, pi, atan

from Bases import *
from Errors import *
from Results import RTResult
import os
from ErrorsNew import *
from Nodes import FuncArgNode


def type_(value):
    return type_values[type(value)]


def assignment_error(value, parent, pos_start, pos_end, context):
    return ErrorNew(ET_IllegalVariableAssignment, f'Cannot assign {value} of type {type(value)} to a '
                                                  f'{type(parent)} type variable', pos_start, pos_end, context)


class Value:
    def __init__(self):
        self.pos_start = None
        self.pos_end = None
        self.context = None

    def set_pos(self, pos_start=None, pos_end=None):
        self.pos_start = pos_start
        self.pos_end = pos_end
        return self

    def set_context(self, context=None):
        self.context = context
        return self

    def assign_checks(self, value, pos_start, pos_end, context):
        raise Exception(f'Missing \'assign_checks()\' method for {type(self)}!')
        pass

    def as_type(self, to_type, pos_start, pos_end, context):
        if to_type == 'auto' or isinstance(self, Null):
            return RTResult().success(self)
        return RTResult().failure(ErrorNew(ET_InvalidSyntax, f'Cannot cast \'{self.value}\' of type {type_(self.value)}'
                                                             f' into a \'{to_type}\'', pos_start, pos_end, context))

    def post_init(self, value):
        raise Exception(f'Missing \'post_init()\' method for {type(self)}!')
        pass

    def added_to(self, other):
        return self.illegal_operation()

    def subbed_by(self, other):
        return self.illegal_operation()

    def multed_by(self, other):
        return self.illegal_operation()

    def dived_by(self, other):
        return self.illegal_operation()

    def fdived_by(self, other):
        return self.illegal_operation()

    def modded_by(self, other):
        return self.illegal_operation()

    def powed_by(self, other):
        return self.illegal_operation()

    def get_comparison_lt(self, other):
        return self.illegal_operation()

    def get_comparison_gt(self, other):
        return self.illegal_operation()

    def get_comparison_lte(self, other):
        return self.illegal_operation()

    def get_comparison_gte(self, other):
        return self.illegal_operation()

    def get_comparison_eq(self, other):
        return self.illegal_operation()

    def get_comparison_ne(self, other):
        return self.illegal_operation()

    def anded_by(self, other):
        return self.illegal_operation()

    def ored_by(self, other):
        return self.illegal_operation()

    def execute(self, args):
        return self.illegal_operation()

    def notted(self):
        return self.illegal_operation()

    def is_true(self):
        return self.illegal_operation()

    def copy(self):
        copy = Number(self.value)
        copy.set_pos(self.pos_start, self.pos_end).set_context(self.context)

        return copy

    def illegal_operation(self, other=None):
        other = self if not other else other
        return None, RTError(self.pos_start, other.pos_end, "Illegal operation. No can do sorry", self.context)

    def get_type(self, log: bool):
        return "<" * log + 'value (base)' + ">" * log

    def __repr__(self):
        return f'{type_(self)}: {self.value}'


class Null(Value):
    def __init__(self):
        super().__init__()

    def added_to(self, other):
        if isinstance(other, String):
            return String(self.__repr__() + other.value).set_context(self.context), None
        return other

    def get_type(self, log: bool):
        return "<" * log + 'Null' + ">" * log

    def copy(self):
        copy = Null()
        return copy.set_pos(self.pos_start, self.pos_end).set_context(self.context)

    def __repr__(self):
        return "<Null value>"


class Infinity(Value):
    def __init__(self, value, previous_type):
        super().__init__()
        self.mult_zero = value
        self.previous_type = previous_type

    def added_to(self, other):
        return self, None

    def subbed_by(self, other):
        return self, None

    def multed_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return self.previous_type(self.mult_zero).set_context(self.context), None
            return Infinity(self.mult_zero * other.value, self.previous_type).set_context(self.context), None
        return self.illegal_operation()

    def dived_by(self, other):
        if isinstance(other, Number):
            return self, None
        elif isinstance(other, Infinity):
            return Int(0).set_context(self.context), None
        return self.illegal_operation()

    def fdived_by(self, other):
        return self.dived_by(other)

    def modded_by(self, other):
        # TODO: Decide what happens here
        return self.illegal_operation()

    def powed_by(self, other):
        return self.illegal_operation()

    def get_comparison_lt(self, other):
        # TODO: Decide what happens here
        return self.illegal_operation()

    def get_comparison_gt(self, other):
        # TODO: Decide what happens here
        return self.illegal_operation()

    def get_comparison_lte(self, other):
        return self.get_comparison_lt(other)

    def get_comparison_gte(self, other):
        return self.get_comparison_gt(other)

    def get_comparison_eq(self, other):
        # TODO: Return false
        return self.illegal_operation()

    def get_comparison_ne(self, other):
        # TODO: Decide what happens here
        return self.illegal_operation()

    def anded_by(self, other):
        # TODO: Convert self.value to bool?
        return self.illegal_operation()

    def ored_by(self, other):
        # TODO: Convert self.value to bool?
        return self.illegal_operation()

    def notted(self):
        # TODO: Convert self.value to bool?
        return self.illegal_operation()

    def is_true(self):
        # TODO: Convert self.value to bool?
        return self.illegal_operation()

    def get_type(self, log: bool):
        return "<" * log + 'Infinity' + f' ({self.mult_zero})>' * log

    def copy(self):
        copy = Infinity(self.mult_zero, self.previous_type)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_start)
        return copy

    def __repr__(self):
        return f'<Infinity ({self.mult_zero})>'


class Number(Value):
    def __init__(self, value):
        super().__init__()
        self.value = value

    def assign_checks(self, value, pos_start, pos_end, context):
        res = RTResult()
        illegal_value_error = ErrorNew(ET_IllegalVariableAssignment, '', pos_start, pos_end, context)
        if isinstance(value, Number.allowed_types):
            return res.success(value)
        elif isinstance(value, String):
            try:
                return res.success(type(self)(float(value.value)))
            except ValueError:
                illegal_value_error.details = f'Cannot assign \'{value}\' of type \'str\' to a \'{type_(self)}\' ' \
                                              f'type variable'
                return res.failure(illegal_value_error)
        elif isinstance(value, List):
            recursion_check = value.recursive_single()
            if recursion_check[1]:
                NonBreakError(pos_start, pos_end, context, WT_ValueFromList).print_method()
                if isinstance(recursion_check[2], Number.allowed_types):
                    return res.success(recursion_check[2])
                elif isinstance(recursion_check[2], String):
                    try:
                        return res.success(Float(recursion_check[2].value))
                    except ValueError:
                        illegal_value_error.details = f'Cannot assign \'{value.elements}\' of type \'list\' to a ' \
                                                      f'\'{type_(self)}\' type variable'
                        return res.failure(illegal_value_error)
            elif recursion_check[0]:
                return res.success(Float(0))
        illegal_value_error.details = f'Cannot assign \'{value}\' of type \'{type_(value)}\' to a \'{type_(self)}\' ' \
                                      f'type variable'
        return res.failure(illegal_value_error)

    def as_type(self, to_type, pos_start, pos_end, context):
        return super().as_type(to_type, pos_start, pos_end, context)

    def ret_type(self, other):
        if isinstance(self, Float) or isinstance(other, Float):
            return Float
        return Int

    def added_to(self, other) -> [Value, None or Error]:
        if isinstance(other, Number):
            return self.ret_type(other)(self.value + other.value).set_context(self.context), None
        elif isinstance(other, Infinity):
            return Infinity(other.mult_zero, other.previous_type).set_context(self.context), None
        elif isinstance(other, String):
            value = other.as_type(type_values[type(self)], self.pos_start, other.pos_end, None)
            if value.error: return None, value.error
            return self.added_to(value.value)
        elif isinstance(other, List):
            res = other.recursive_single()
            if res[0]:
                return self
            elif res[1]:
                return self.added_to(res[2])
            return other.added_to(self)
        return self.illegal_operation(other)

    def subbed_by(self, other):
        if isinstance(other, Number):
            return self.ret_type(other)(self.value - other.value).set_context(self.context), None
        elif isinstance(other, Infinity):
            return Infinity(-other.mult_zero, other.previous_type).set_context(self.context), None
        return self.illegal_operation(other)

    def multed_by(self, other):
        if isinstance(other, Number):
            return self.ret_type(other)(self.value * other.value).set_context(self.context), None
        elif isinstance(other, Infinity):
            return Infinity(self.value * other.mult_zero, other.previous_type).set_context(self.context), None
        return self.illegal_operation()

    def dived_by(self, other):
        if isinstance(other, Number):
            if isinstance(other.value, Infinity):
                NonBreakError(self.pos_start, other.pos_end, self.context, WT_ValueDivInfinity).print_method()
                return Int(0).set_context(self.context), None
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, WT_DivideByZero).print_method()
                return Infinity(self.value, Int).set_context(self.context), None
            return Float(self.value / other.value).set_context(self.context), None
        elif isinstance(other, Infinity):
            return Int(0).set_context(self.context), None
        return self.illegal_operation()

    def fdived_by(self, other):
        if isinstance(other, Number):
            if isinstance(other.value, Infinity):
                NonBreakError(self.pos_start, other.pos_end, self.context, WT_ValueDivInfinity).print_method()
                return Int(0).set_context(self.context), None
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, WT_DivideByZero).print_method()
                return Infinity(self.value, Int).set_context(self.context), None
            return Int(self.value // other.value).set_context(self.context), None
        elif isinstance(other, Infinity):
            return Int(0).set_context(self.context), None
        return self.illegal_operation()

    def modded_by(self, other):
        if isinstance(other, Number):
            if isinstance(other.value, Infinity):
                return self.illegal_operation()
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, WT_ModByZero).print_method()
                return Int(0).set_context(self.context), None
            return self.ret_type(other)(self.value % other.value).set_context(self.context), None
        return self.illegal_operation()

    def powed_by(self, other):
        if isinstance(other, Number):
            if isinstance(other.value, Infinity):
                return self.illegal_operation()
            return self.ret_type(other)(self.value ** other.value).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_lt(self, other):
        if isinstance(other, Number):
            return Bool(self.value < other.value).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_gt(self, other):
        if isinstance(other, Number):
            return Bool(self.value > other.value).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_lte(self, other):
        if isinstance(other, Number):
            return Bool(self.value <= other.value).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_gte(self, other):
        if isinstance(other, Number):
            return Bool(self.value >= other.value).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_eq(self, other):
        if isinstance(other, Number):
            return Bool(self.value == other.value).set_context(self.context), None
        return self.illegal_operation()

    def get_comparison_ne(self, other):
        if isinstance(other, Number):
            return Bool(self.value != other.value).set_context(self.context), None
        return self.illegal_operation()

    def anded_by(self, other):
        if isinstance(other, Number):
            return Bool(__value_to_bool__(self.value) and __value_to_bool__(other.value)) \
                       .set_context(self.context), None
        return self.illegal_operation()

    def ored_by(self, other):
        if isinstance(other, Number):
            return Bool(__value_to_bool__(self.value) or __value_to_bool__(other.value)) \
                       .set_context(self.context), None
        return self.illegal_operation()

    def notted(self):
        return Bool(not __value_to_bool__(self.value)).set_context(self.context), None

    def is_true(self):
        return self.value > 0

    def get_type(self, log: bool):
        return "<" * log + 'number (base)' + ">" * log

    def __repr__(self):
        return str(self.value)


class Int(Number):
    def __init__(self, value):
        super().__init__(Infinity(round(value.mult_zero), Int) if isinstance(value, Infinity) else round(value))

    def as_type(self, to_type, pos_start, pos_end, context):
        res = RTResult()
        if to_type == 'int':
            return res.success(self)
        if to_type == 'float':
            return res.success(Float(self.value))
        if to_type == 'str':
            return res.success(String(str(self.value)))
        if to_type == 'list':
            return res.success(List([self]))
        if to_type == 'bool':
            return res.success(Bool(self.value))
        return super().as_type(to_type, pos_start, pos_end, context)

    def copy(self):
        copy = Int(self.value)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def get_type(self, log: bool):
        return "<" * log + 'int' + ">" * log

    def __repr__(self):
        return str(self.value)


class Float(Number):
    def __init__(self, value):
        super().__init__(float(value))

    def as_type(self, to_type, pos_start, pos_end, context):
        res = RTResult()
        if to_type == 'int':
            return res.success(Int(self.value))
        if to_type == 'float':
            return res.success(self)
        if to_type == 'str':
            return res.success(String(str(self.value)))
        if to_type == 'list':
            return res.success(List([self]))
        if to_type == 'bool':
            return res.success(Bool(self.value))
        return super().as_type(to_type, pos_start, pos_end, context)

    def copy(self):
        copy = Float(self.value)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def get_type(self, log: bool):
        return "<" * log + 'float' + ">" * log

    def __repr__(self):
        return str(self.value)


# class sFloat(Value):
#     def __init__(self, value, sig_figs):
#         super().__init__()
#         self.value = value
#         self.error = 0
#
#     def added_to(self, other):
#         if isinstance(other, Number):
#             return Number(self.value + other.value).set_context(self.context), None
#         return self.illegal_operation(other)
#
#     def subbed_by(self, other):
#         if isinstance(other, Number):
#             return Number(self.value - other.value).set_context(self.context), None
#         return self.illegal_operation(other)
#
#     def multed_by(self, other):
#         if isinstance(other, Number):
#             return Number(self.value * other.value).set_context(self.context), None
#         return self.illegal_operation()
#
#     def dived_by(self, other):
#         if isinstance(other, Number):
#             if other.value == 0:
#                 NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero).print_method()
#                 return Infinity(self.value).set_context(self.context), None
#             return Number(self.value / other.value).set_context(self.context), None
#         return self.illegal_operation()
#
#     def fdived_by(self, other):
#         if isinstance(other, Number):
#             if other.value == 0:
#                 repr(NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero))
#                 return Infinity(self.value).set_context(self.context), None
#             return Number(self.value // other.value).set_context(self.context), None
#         return self.illegal_operation()
#
#     def modded_by(self, other):
#         if isinstance(other, Number):
#             if other.value == 0:
#                 repr(NonBreakError(self.pos_start, other.pos_end, self.context, ET_ModByZero))
#                 return Number(0).set_context(self.context), None
#             return Number(self.value % other.value).set_context(self.context), None
#         return self.illegal_operation()
#
#     def powed_by(self, other):
#         if isinstance(other, Number):
#             return Number(self.value ** other.value).set_context(self.context), None
#         return self.illegal_operation()
#
#     def get_comparison_lt(self, other):
#         if isinstance(other, Number):
#             return Number(int(self.value < other.value)).set_context(self.context), None
#         return self.illegal_operation()
#
#     def get_comparison_gt(self, other):
#         if isinstance(other, Number):
#             return Number(int(self.value > other.value)).set_context(self.context), None
#         return self.illegal_operation()
#
#     def get_comparison_lte(self, other):
#         if isinstance(other, Number):
#             return Number(int(self.value <= other.value)).set_context(self.context), None
#         return self.illegal_operation()
#
#     def get_comparison_gte(self, other):
#         if isinstance(other, Number):
#             return Number(int(self.value >= other.value)).set_context(self.context), None
#         return self.illegal_operation()
#
#     def get_comparison_eq(self, other):
#         if isinstance(other, Number):
#             return Number(int(self.value == other.value)).set_context(self.context), None
#         return self.illegal_operation()
#
#     def get_comparison_ne(self, other):
#         if isinstance(other, Number):
#             return Number(int(self.value != other.value)).set_context(self.context), None
#         return self.illegal_operation()
#
#     def anded_by(self, other):
#         if isinstance(other, Number):
#             return Number(int(self.value and other.value)).set_context(self.context), None
#         return self.illegal_operation()
#
#     def ored_by(self, other):
#         if isinstance(other, Number):
#             return Number(int(self.value or other.value)).set_context(self.context), None
#         return self.illegal_operation()
#
#     def notted(self):
#         return Number(1 if self.value <= 0 else 0).set_context(self.context), None
#
#     def is_true(self):
#         return self.value > 0
#
#     def __repr__(self):
#         return str(self.value)


class Bool(Number):
    def __init__(self, value):
        super().__init__(True if value > 0 else False)

    def assign_checks(self, value, pos_start, pos_end, context):
        res = RTResult()
        if isinstance(value, Number):
            return res.success(Bool(__value_to_bool__(value.value)))
        elif isinstance(value, String):
            return res.success(Bool(value.value != ''))
        elif isinstance(value, List):
            recursive = value.recursive_single()
            return res.success(Bool(not recursive[0]))
        return res.failure(ErrorNew(ET_IllegalVariableAssignment, f'Cannot assign \'{value}\' of type \'{type_(value)}'
                                                                  f'\' to a \'{type_(self)}\' type variable',
                                    pos_start, pos_end, context))

    def as_type(self, to_type, pos_start, pos_end, context):
        res = RTResult()
        if to_type == 'int':
            return res.success(Int(self.value))
        if to_type == 'float':
            return res.success(Float(float(self.value)))
        if to_type == 'str':
            return res.success(String('true' if self.value else 'false'))
        if to_type == 'list':
            return res.success(List([self]))
        if to_type == 'bool':
            return res.success(self)
        return super().as_type(to_type, pos_start, pos_end, context)

    def copy(self):
        copy = Bool(self.value)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def is_true(self):
        return self.value

    def get_type(self, log: bool):
        return "<" * log + 'bool' + ">" * log

    def __repr__(self):
        return 'true' if self.value else 'false'


class String(Value):
    def __init__(self, value):
        super().__init__()
        self.value = value

    def assign_checks(self, value, pos_start, pos_end, context):
        res = RTResult()
        if isinstance(value, String):
            return res.success(value)
        elif isinstance(value, Number):
            NonBreakError(pos_start, pos_end, context, WT_StringFromValue).print_method()
            return res.success(String(str(value.value)))
        return res.failure(assignment_error(value, self, pos_start, pos_end, context))

    def as_type(self, to_type, pos_start, pos_end, context) -> RTResult:
        res = RTResult()
        if to_type == 'int':
            try:
                out = int(self.value)
                return res.success(Int(out))
            except ValueError:
                return res.failure(ErrorNew(ET_IllegalValue, f'\'{self.value}\' cannot be cast to an \'int\' type',
                                            pos_start, pos_end, context))
        elif to_type == 'float':
            try:
                out = float(self.value)
                return res.success(Float(out))
            except ValueError:
                return res.failure(ErrorNew(ET_IllegalValue, f'\'{self.value}\' cannot be cast to an \'float\' type',
                                            pos_start, pos_end, context))
        elif to_type == 'str':
            return res.success(self)
        elif to_type == 'list':
            return res.success(List([self]))
        elif to_type == 'bool':
            return res.success(Bool(len(self.value)))
        return super().as_type(to_type, pos_start, pos_end, context)

    def added_to(self, other):
        if isinstance(other, String):
            return String(self.value + other.value).set_context(self.context), None
        elif isinstance(other, Number):
            return String(self.value + str(other.value)).set_context(self.context), None
        elif isinstance(other, Null):
            return String(self.value + other.__repr__()).set_context(self.context), None
        return self.illegal_operation()

    def multed_by(self, other):
        if isinstance(other, Number) and not isinstance(other.value, Infinity):
            return String(self.value * other.value).set_context(self.context), None
        return self.illegal_operation()

    def is_true(self):
        return len(self.value) > 0, None

    def get_comparison_eq(self, other):
        return Bool(self.value == str(other.value)), None

    def copy(self):
        copy = String(self.value)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def get_type(self, log: bool):
        return "<" * log + 'string' + ">" * log

    def __str__(self):
        return f'{self.value}'

    def __repr__(self):
        return f'\"{self.value}\"'


class List(Value):
    def __init__(self, elements):
        super().__init__()
        self.elements = elements

    def recursive_single(self):
        """
        :return: [was recursively empty, recursively had 1 element in,
        single element if had 1 element else original]
        """
        test_list = self.elements
        while isinstance(test_list, list):
            if len(test_list) == 1:
                test_list = test_list[0]
                if isinstance(test_list, List):
                    test_list = test_list.elements
            elif len(test_list) == 0:
                return True, False, self.elements
            else:
                return False, False, self.elements
        return False, True, test_list

    def as_type(self, to_type, pos_start, pos_end, context):
        res = RTResult()
        if to_type == 'int':
            _, recursive, value = self.recursive_single()
            if recursive:
                return value.as_type(to_type, pos_start, pos_end, context)
            return res.failure(ErrorNew(ET_IllegalValue, f'Cannot convert \'{value}\' of type \'{type_(self)}\' '
                                                         f'into an \'int\'',
                                        pos_start, pos_end, context))
        if to_type in ('float', 'str'):
            _, recursive, value = self.recursive_single()
            if recursive:
                return value.as_type(to_type, pos_start, pos_end, context)
            return res.failure(ErrorNew(ET_IllegalValue, f'Cannot convert \'{value}\' into a \'{to_type}\'', pos_start,
                                        pos_end, context))
        if to_type == 'bool':
            recursive, _, _ = self.recursive_single()
            return res.success(Bool(recursive))
        if to_type == 'list':
            return self
        return super().as_type(to_type, pos_start, pos_end, context)

    def assign_checks(self, value, pos_start, pos_end, context):
        res = RTResult()
        if isinstance(value, List):
            return res.success(value)
        elif isinstance(value, Number) or isinstance(value, String):
            NonBreakError(pos_start, pos_end, context, WT_ListFromValue).print_method()
            return res.success(List([value]))
        return res.failure(assignment_error(value, self, pos_start, pos_end, context))

    def added_to(self, other):
        if isinstance(other, List):
            self.elements.extend(other.elements)
            return self, None
        elif isinstance(other, Number):
            self.elements = [i.value + other.value for i in self.elements]
            return self, None
        else:
            return self.illegal_operation()

    def multed_by(self, other):
        if isinstance(other, Number):
            self.elements = [i.value * other.value for i in self.elements]
            return self, None
        else:
            return self.illegal_operation()

    def copy(self):
        copy = List(self.elements)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def __str__(self):
        return repr(self)

    def __repr__(self):
        return f'[{", ".join([repr(i) for i in self.elements])}]'


class BaseFunction(Value):
    def __init__(self, name, ret_type=None):
        super().__init__()
        self.name = name or "<anonymous>"
        self.ret_type = ret_type

    def generate_new_context(self):
        new_context = Context(self.name, '', self.context, self.pos_start)
        new_context.symbol_table = SymbolTable(new_context.parent.symbol_table)
        new_context.symbol_table.options = new_context.parent.symbol_table.options
        return new_context

    def populate_args(self, args, exec_ctx):
        for arg_name, arg_value in args.items():
            arg_value.set_context(exec_ctx)
            exec_ctx.symbol_table.set(arg_name, arg_value)
        return RTResult().success(None)


class Function(BaseFunction):
    def __init__(self, name, body_node, arg_names, ret_type):
        super().__init__(name, ret_type)
        self.body_node = body_node
        self.arg_names = arg_names

    def execute(self, args):
        pass

    def copy(self):
        copy = Function(self.name, self.body_node, self.arg_names, self.ret_type)
        copy.set_context(self.context)
        copy.set_pos(self.pos_start, self.pos_end)
        return copy

    def __repr__(self):
        return f"<function {self.name}>"


class BuiltInFunction(BaseFunction):
    def __init__(self, name, arg_names, executable):
        super().__init__(name)
        self.arg_names = arg_names
        self.executable = executable

    def pre_execute(self, args):
        exec_ctx = self.generate_new_context()
        self.populate_args(args, exec_ctx)
        return exec_ctx

    def execute(self, args):
        return self.executable(self, self.pre_execute(args))

    # def execute_root(self, exec_ctx):
    #     value = exec_ctx.symbol_table.get('value')[0]
    #     root_ = exec_ctx.symbol_table.get('root_')[0]
    #     mod = sqrt(value.re ** 2 + value.im ** 2)
    #     arg = atan(value.im / value.re)
    #     arg = arg - 2 * pi if arg > pi else arg
    #     k = list(range(ceil((-root_ * pi - arg) / (2 * pi)), floor((root_ * pi - arg) / (2 * pi)) + 1))
    #     k = [(arg + 2 * n_ * pi) / root_ for n_ in k]
    #     return List([Imag(mod * cos(n_), mod * sin(n_)) for n_ in k])
    #
    # execute_root.arg_names = ['value', 'root_']

    def copy(self):
        copy = BuiltInFunction(self.name, self.arg_names, self.executable)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def get_type(self, log: bool):
        return "<" * log + 'built-in method' + ">" * log

    def __repr__(self):
        return f'<built-in function {self.name}>'


def __value_to_bool__(value):
    return value > 0


def execute_log(self, exec_ctx: Context):
    added = ''
    out = exec_ctx.symbol_table.get("value")[0]
    if exec_ctx.parent.display_name == '<shell>':
        if str(out)[-1] != '\n':
            added = '\033[1;7m%\033[0m\n'
    print(repr(out) + added, end='')
    return RTResult().success(None)


def execute_print(self, exec_ctx: Context):
    value, _ = exec_ctx.symbol_table.get("value")
    print(value) if not isinstance(value, Null) else None
    del value, exec_ctx
    return RTResult().success(None)


def execute_type(self, exec_ctx: Context):
    return RTResult().success(String(exec_ctx.symbol_table.get("value")[0].get_type(True)))


def execute_input(self, exec_ctx: Context):
    text = input(exec_ctx.symbol_table.get('prompt')[0])
    return RTResult().success(String(text))


def execute_clear(self, exec_ctx: Context):
    os.system('cls' if os.name == 'nt' else 'clear')
    return RTResult().success(None)


def execute_quit(self, exec_ctx: Context):
    sys.exit(exec_ctx.symbol_table.get('exit_code')[0])


func_log = BuiltInFunction('log', {'value': FuncArgNode('str', String(''), 'Value to be printed to the console')},
                           execute_log)
func_print = BuiltInFunction('print', {'value': FuncArgNode('str', String(''), 'Value to be printed to the console')},
                             execute_print)
func_type = BuiltInFunction('type', {'value': FuncArgNode('auto', explain='Value to have its type calculated and '
                                                                          'returned')}, execute_type)
func_input = BuiltInFunction('input', {'prompt': FuncArgNode('str', String(''), 'Prompt to be printed to the console '
                                                                                'before the user can input anything')},
                             execute_input)
func_clear = BuiltInFunction('clear', {}, execute_clear)
func_run = BuiltInFunction('run', {'fn': FuncArgNode('str', explain='Path to an executable .fck script. Relative to the'
                                                                    ' current working directory')}, None)
func_quit = BuiltInFunction('quit', {'exit_code': FuncArgNode('int', Int(0), 'Exit code to exit the program with')},
                            execute_quit)

class_identifier_values = {int: 0, float: 0.5, bool: 0}

default_values = {'int': Int(0), 'float': Float(0), 'str': String(''), 'list': List([]), 'bool': Bool(False),
                  }  # 'imag': Imag(1, 0)}
type_values = {}
for i, n in default_values.items():
    type_values[type(n)] = i
Number.allowed_types = (Int, Float, Bool)
