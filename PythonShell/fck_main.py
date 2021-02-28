from Lexer import *
from Parser import *
from Interpreter import *

global_symbol_table = SymbolTable()
global_symbol_table.set("null", Number(0))
global_symbol_table.set("true", Number(1))
global_symbol_table.set("false", Number(0))


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
