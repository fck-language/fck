from Lexer import *
from Parser import *
from Interpreter import *

global_symbol_table = SymbolTable()
global_symbol_table.set("null", Number(0))
global_symbol_table.set("true", Number(1))
global_symbol_table.set("false", Number(0))
global_symbol_table.set("log", BuiltInFunction("log"))
global_symbol_table.set("print", BuiltInFunction("print"))
global_symbol_table.set("input", BuiltInFunction("input"))
global_symbol_table.set("clear", BuiltInFunction("clear"))
global_symbol_table.set("run", BuiltInFunction("run"))


def execute_run(self, exec_ctx):
    fn = exec_ctx.symbol_table.get("fn")

    if not isinstance(fn, String):
        return RTResult().failure(RTError(self.pos_start, self.pos_end, "Filename must be a string", exec_ctx))

    fn = fn.value

    try:
        with open(fn, 'r') as f:
            script = f.read()
    except Exception as e:
        return RTResult().failure(RTError(self.pos_start, self.pos_end, f'Failed to load script \"{fn}\"\n'
                                          + str(e), exec_ctx))

    _, error = run(fn, script)

    if error:
        return RTResult().failure(RTError(self.pos_start, self.pos_end, f"An error was returned while running"
                                                                        f" \"{fn}\"\n{error.as_string()}", exec_ctx))

    return RTResult().success(None)


execute_run.arg_names = ["fn"]


BuiltInFunction.execute_run = execute_run


def run(fn, text):
    lexer = Lexer(fn, text)
    tokens, error = lexer.make_tokens()

    if error: return None, error

    parser = Parser(tokens)
    ast = parser.parse()
    if ast.error: return None, ast.error

    interpreter = Interpreter()
    context = Context("<main>")
    context.symbol_table = global_symbol_table
    res = interpreter.visit(ast.node, context)

    return res.value, res.error
