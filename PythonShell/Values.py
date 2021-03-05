from Bases import *
from Errors import *
from Results import RTResult
import os


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
        return f'{self.type}: {self.value}'


class Null(Value):
    def __init__(self):
        super().__init__()

    def added_to(self, other):
        if isinstance(other, String):
            return String(self.__repr__() + other.value).set_context(self.context), None
        return other

    def get_type(self, log: bool):
        return "<" * log + 'Null' + ">" * log

    def __repr__(self):
        return "<Null value>"


class Infinity(Value):
    def __init__(self, value):
        super().__init__()
        self.mult_zero = value

    def added_to(self, other):
        return self

    def subbed_by(self, other):
        self.mult_zero = -self.mult_zero
        return self

    def multed_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                return Number(self.mult_zero).set_context(self.context), None
            return self
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

    def __repr__(self):
        return f'<Infinity ({self.mult_zero})>'


class Number(Value):
    def __init__(self, value):
        super().__init__()
        self.value = value

    def added_to(self, other):
        if isinstance(other, Number):
            return Number(self.value + other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def subbed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value - other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def multed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value * other.value).set_context(self.context), None
        return self.illegal_operation()

    def dived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero).print_method()
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value / other.value).set_context(self.context), None
        return self.illegal_operation()

    def fdived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero)
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value // other.value).set_context(self.context), None
        return self.illegal_operation()

    def modded_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_ModByZero).print_method()
                return Number(0).set_context(self.context), None
            return Number(self.value % other.value).set_context(self.context), None
        return self.illegal_operation()

    def powed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value ** other.value).set_context(self.context), None
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
        super().__init__(round(value))

    def copy(self):
        copy = Int(self.value)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def added_to(self, other):
        if isinstance(other, Number):
            return Number(self.value + other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def subbed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value - other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def multed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value * other.value).set_context(self.context), None
        return self.illegal_operation()

    def dived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero).print_method()
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value / other.value).set_context(self.context), None
        return self.illegal_operation()

    def fdived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero).print_method()
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value // other.value).set_context(self.context), None
        return self.illegal_operation()

    def modded_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_ModByZero).print_method()
                return Number(0).set_context(self.context), None
            return Number(self.value % other.value).set_context(self.context), None
        return self.illegal_operation()

    def powed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value ** other.value).set_context(self.context), None
        return self.illegal_operation()

    def get_type(self, log: bool):
        return "<" * log + 'int' + ">" * log

    def __repr__(self):
        return str(self.value)


class Float(Number):
    def __init__(self, value):
        super().__init__(float(value))

    def copy(self):
        copy = Float(self.value)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def added_to(self, other):
        if isinstance(other, Number):
            return Number(self.value + other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def subbed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value - other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def multed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value * other.value).set_context(self.context), None
        return self.illegal_operation()

    def dived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero).print_method()
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value / other.value).set_context(self.context), None
        return self.illegal_operation()

    def fdived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                repr(NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero))
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value // other.value).set_context(self.context), None
        return self.illegal_operation()

    def modded_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                repr(NonBreakError(self.pos_start, other.pos_end, self.context, ET_ModByZero))
                return Number(0).set_context(self.context), None
            return Number(self.value % other.value).set_context(self.context), None
        return self.illegal_operation()

    def powed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value ** other.value).set_context(self.context), None
        return self.illegal_operation()

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

    def copy(self):
        copy = Bool(self.value)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def added_to(self, other):
        if isinstance(other, Number):
            ret_type = None
            if isinstance(other, Int) or isinstance(other, Bool):
                ret_type = Int
            elif isinstance(other, Float):
                ret_type = Float
            if ret_type:
                return ret_type(self.value + other.value).set_context(self.context), None
        elif isinstance(other, String):
            return String(f'{repr(self)}' + other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def subbed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value - other.value).set_context(self.context), None
        return self.illegal_operation(other)

    def multed_by(self, other):
        if isinstance(other, Number):
            ret_type = None
            if isinstance(other, Int) or isinstance(other, Bool):
                ret_type = Int
            elif isinstance(other, Float):
                ret_type = Float
            if ret_type:
                return ret_type(self.value * other.value).set_context(self.context), None
        elif isinstance(other, String):
            NonBreakError(self.pos_start, other.pos_end, self.context, ET_ValueMultString).print_method()
            return String(other.value * self.value).set_context(self.context), None
        return self.illegal_operation(other)

    def dived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero).print_method()
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value / other.value).set_context(self.context), None
        return self.illegal_operation()

    def fdived_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_DivideByZero).print_method()
                return Infinity(self.value).set_context(self.context), None
            return Number(self.value // other.value).set_context(self.context), None
        return self.illegal_operation()

    def modded_by(self, other):
        if isinstance(other, Number):
            if other.value == 0:
                NonBreakError(self.pos_start, other.pos_end, self.context, ET_ModByZero).print_method()
                return Number(0).set_context(self.context), None
            return Number(self.value % other.value).set_context(self.context), None
        return self.illegal_operation()

    def powed_by(self, other):
        if isinstance(other, Number):
            return Number(self.value ** other.value).set_context(self.context), None
        return self.illegal_operation()

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

    def added_to(self, other):
        if isinstance(other, String):
            return String(self.value + other.value).set_context(self.context), None
        elif isinstance(other, Number):
            return String(self.value + str(other.value)).set_context(self.context), None
        elif isinstance(other, Null):
            return String(self.value + other.__repr__()).set_context(self.context), None
        return self.illegal_operation()

    def multed_by(self, other):
        if isinstance(other, Number):
            return String(self.value * other.value).set_context(self.context), None
        return self.illegal_operation()

    def is_true(self):
        return len(self.value) > 0, None

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
        return ", ".join([str(i) for i in self.elements])

    def __repr__(self):
        return f'[{", ".join([str(i) for i in self.elements])}]'


class BaseFunction(Value):
    def __init__(self, name):
        super().__init__()
        self.name = name or "<anonymous>"

    def generate_new_context(self):
        new_context = Context(self.name, self.context, self.pos_start)
        new_context.symbol_table = SymbolTable(new_context.parent.symbol_table)
        return new_context

    def check_args(self, arg_names, args):
        res = RTResult()

        if len(args) > len(arg_names):
            return res.failure(RTError(
                self.pos_start, self.pos_end,
                f"{len(args) - len(arg_names)} too many args passed into {self}",
                self.context
            ))

        if len(args) < len(arg_names):
            return res.failure(RTError(
                self.pos_start, self.pos_end,
                f"{len(arg_names) - len(args)} too few args passed into {self}",
                self.context
            ))

        return res.success(None)

    def populate_args(self, arg_names, args, exec_ctx):
        for i in range(len(args)):
            arg_name = arg_names[i]
            arg_value = args[i]
            arg_value.set_context(exec_ctx)
            exec_ctx.symbol_table.set(arg_name, arg_value)

    def check_and_populate_args(self, arg_names, args, exec_ctx):
        res = RTResult()
        res.register(self.check_args(arg_names, args))
        if res.should_return(): return res
        self.populate_args(arg_names, args, exec_ctx)
        return res.success(None)


class Function(BaseFunction):
    def __init__(self, name, body_node, arg_names):
        super().__init__(name)
        self.body_node = body_node
        self.arg_names = arg_names

    def execute(self, args):
        pass

    def copy(self):
        copy = Function(self.name, self.body_node, self.arg_names)
        copy.set_context(self.context)
        copy.set_pos(self.pos_start, self.pos_end)
        return copy

    def __repr__(self):
        return f"<function {self.name}>"


class BuiltInFunction(BaseFunction):
    def __init__(self, name):
        super().__init__(name)

    def execute(self, args):
        res = RTResult()
        exec_ctx = self.generate_new_context()

        method = getattr(self, f'execute_{self.name}', self.no_visit_method)
        res.register(self.check_and_populate_args(method.arg_names, args, exec_ctx))
        if res.should_return(): return res

        return_value = res.register(method(exec_ctx))
        if res.should_return(): return res
        return res.success(return_value)

    def no_visit_method(self, context):
        raise Exception(f'No \'execute_{self.name}\' method defined silly x')

    def execute_log(self, exec_ctx):
        print(repr(exec_ctx.symbol_table.get("value")))
        return RTResult().success(None)

    execute_log.arg_names = ['value']

    def execute_print(self, exec_ctx):
        value = exec_ctx.symbol_table.get("value")
        print(str(value)) if not isinstance(value, Null) else None
        return RTResult().success(None)

    execute_print.arg_names = ['value']

    def execute_type(self, exec_ctx):
        print(exec_ctx.symbol_table.get("value").get_type(True))
        return RTResult().success(None)

    execute_type.arg_names = ['value']

    def execute_input(self, exec_ctx):
        text = input()
        return RTResult().success((String(text)))

    execute_input.arg_names = []

    def execute_clear(self, exec_ctx):
        os.system('cls' if os.name == 'nt' else 'clear')
        return RTResult().success(None)

    execute_clear.arg_names = []

    def execute_run(self, exec_ctx):
        pass

    def copy(self):
        copy = BuiltInFunction(self.name)
        copy.set_context(self.context).set_pos(self.pos_start, self.pos_end)
        return copy

    def get_type(self, log: bool):
        return "<" * log + 'built-in method' + ">" * log

    def __repr__(self):
        return f'<built-in function {self.name}>'


def __value_to_bool__(value):
    return True if value > 0 else False
