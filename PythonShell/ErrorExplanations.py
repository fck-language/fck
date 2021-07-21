# List of names and identifiers
# *_names   -> expanded names of identifiers
# *_format  -> formatting strings for the error/warning
# *_explain -> explanation list

from os.path import dirname

##########
# Errors #
##########

ET_ExpectedChar = "ExpectedChar"
ET_ExpectedExpr = "ExpectedExpr"
ET_ExpectedType = "ExpectedType"
ET_ExpectedSilencaleType = "ExpectedSilencaleType"
ET_ExpectedIdentifier = "ExpectedIdentifier"
ET_ExpectedLoopIdentifier = "ExpectedLoopIdentifier"
ET_ExpectedAssignmentOperator = "ExpectedAssignmentOperator"
ET_ExpectedAttribute = "ExpectedAttribute"
ET_InvalidGlobalOption = "InvalidGlobalOption"
ET_IllegalChar = "IllegalChar"
ET_IllegalValueType = "IllegalValueType"
ET_UnexpectedToken = "UnexpectedToken"
ET_UnmatchedBracket = "UnmatchedBracket"
ET_IllegalCastType = "IllegalCastType"
ET_IllegalVariableValue = "IllegalVariableValue"
ET_IllegalArgumentValue = "IllegalArgumentValue"  # Not included
# This is for later when arguments can have user defined illegal values
ET_UndefinedLoopIdentifier = "UndefinedLoopIdentifier"
ET_InvalidIterable = "InvalidIterable"
ET_IllegalValue = "IllegalValueError"
ET_IllegalOperation = "IllegalOperation"
ET_ArgumentType = "ArgumentType"
ET_TooArgument = "TooArgument"
ET_UnknownAttribute = "UnknownAttribute"
ET_UnknownIdentifier = "UnknownIdentifier"
ET_ExpectedCompOp = "ExpectedCompOp"  # Not included

error_names = {ET_ExpectedChar: "Expected character",
               ET_ExpectedExpr: "Expected expression",
               ET_ExpectedType: "Expected type",
               ET_ExpectedSilencaleType: "Expected silencable type",
               ET_ExpectedIdentifier: "Expected identifier",
               ET_ExpectedLoopIdentifier: "Expected loop identifier",
               ET_ExpectedAssignmentOperator: "Expected assignment operator",
               ET_ExpectedAttribute: "Expected attribute",
               ET_InvalidGlobalOption: "Invalid global option",
               ET_IllegalChar: "Illegal character",
               ET_IllegalValueType: "Illegal value type",
               ET_UnexpectedToken: "Unexpected token",
               ET_UnmatchedBracket: "Unmatched bracket",
               ET_IllegalCastType: "Illegal cast type",
               ET_IllegalVariableValue: "Illegal variable value",
               ET_IllegalArgumentValue: "Illegal argument value",
               ET_UndefinedLoopIdentifier: "Unidentified loop identifier",
               ET_InvalidIterable: "Invalid iterable",
               ET_IllegalValue: "Illegal value error",
               ET_IllegalOperation: "Illegal operation",
               ET_ArgumentType: "Illegal argument casting",
               ET_TooArgument: "Too arguments",
               ET_UnknownAttribute: "Unknown attribute",
               ET_UnknownIdentifier: "Unknown identifier",
               ET_ExpectedCompOp: "Expected comparison operator"}

error_format = {ET_ExpectedChar: "{details}\n{traceback}",
                ET_ExpectedExpr: "{details}\n{traceback}",
                ET_ExpectedType: "{details}\n{traceback}",
                ET_ExpectedSilencaleType: "{details}\n{traceback}",
                ET_ExpectedIdentifier: "{details}\n{traceback}",
                ET_ExpectedLoopIdentifier: "{details}\n{traceback}",
                ET_ExpectedAssignmentOperator: "{details}\n{traceback}",
                ET_ExpectedAttribute: "{details}\n{arg_explain}\n{traceback}",
                ET_InvalidGlobalOption: "{details}\n{traceback}",
                ET_IllegalChar: "{details}\n{traceback}",
                ET_IllegalValueType: "{details}\n{traceback}",
                ET_UnexpectedToken: "{details}\n{traceback}",
                ET_UnmatchedBracket: "{details}\n{traceback}",
                ET_IllegalCastType: "{details}\n{traceback}",
                ET_IllegalVariableValue: "{details}\n{traceback}",
                ET_IllegalArgumentValue: "{details}\n{arg_explain}\n{traceback}",
                ET_UndefinedLoopIdentifier: "{details}\n{traceback}",
                ET_InvalidIterable: "{details}\n{traceback}",
                ET_IllegalValue: "{details}\n{traceback}",
                ET_IllegalOperation: "{details}\n{traceback}",
                ET_ArgumentType: "{details}\n{arg_explain}\n{traceback}",
                ET_TooArgument: "{details}\n{arg_explain}\n{traceback}",
                ET_UnknownAttribute: "{details}\n{arg_explain}\n{traceback}",
                ET_UnknownIdentifier: "{details}\n{traceback}",
                ET_ExpectedCompOp: "{details}\n{traceback}"}

error_explain = {ET_ExpectedChar: ["Returned when a character was expected but not found",
                                   'print(3 ? "hello")\n'
                                   '                 ^',
                                   'A \':\' was expected with the \'?\' but was not found'],
                 ET_ExpectedExpr: ["Returned when an expression was expected but not found",
                                   "auto example_variable\n"
                                   "                     ^",
                                   "'auto' type variables must be initialised with a value, so leaving this blank "
                                   "raised an error"],
                 ET_ExpectedType: ["Returned when a type was expected but not found",
                                   "print(1.0 as )\n"
                                   "             ^",
                                   "When casting a value, a type must be given to cast to"],
                 ET_ExpectedSilencaleType: ["Returned when a silent type variable is made, but the given type is not "
                                            "silencable",
                                            "silent<int> example_variable :: 1\n"
                                            "       ^^^",
                                            "'int' is not a silencable type, so a 'silent<int>' type cannot be made"],
                 ET_ExpectedIdentifier: ["Returned when an identifier was expected and not found",
                                         "int :: 3\n"
                                         "    ^^", ""],
                 ET_ExpectedLoopIdentifier: ["Returned when '@' is not followed by an identifier",
                                             "@ iterate 10 { ... }\n"
                                             " ^", ""],
                 ET_ExpectedAssignmentOperator: ["Returned when a variable with no default value is initialised and not"
                                                 "given a value",
                                                 "auto a\n"
                                                 "      ^",
                                                 "When initialising a variable with the 'auto' keyword, a value must "
                                                 "be given to initialise it to"],
                 ET_ExpectedAttribute: ["Returned when a '.' is not followed by an attribute identifier",
                                        "silent<case> example :: case(5){}\n"
                                        "example.\n"
                                        "        ^", ""],
                 ET_InvalidGlobalOption: ["Returned when a global option is specified which does not exist",
                                          "`example\n"
                                          "^^^^^^^^", ""],  # TODO: move to a warning
                 ET_IllegalChar: ["Returned when a non-utf8 character is given",
                                  "print('é')\n"
                                  "       ^", ""],
                 ET_IllegalValueType: ["Returned when a value is given that cannot be used in a meaningful way",
                                       "list fib :: [1, 1, 2, 3, 5, 8]\n"
                                       "list range :: [1, 2]\n"
                                       "print(fib[range])\n"
                                       "          ^^^^^",
                                       "A list cannot be used to index another list currently"],
                 ET_UnexpectedToken: ["Returned after a section of code has been parsed and a token is found where an "
                                      "end of file token was expected instead",
                                      "(1 + 2) 3\n"
                                      "        ^", ""],
                 ET_UnmatchedBracket: ["Returned when an open bracket is found, but no closing bracket was found",
                                       "list example :: [(2 ** 5]\n"
                                       "                        ^", ""],
                 ET_IllegalCastType: ["Returned when a values cannot be cast to the given type",
                                      "[1, 2] as int\n"
                                      "          ^^^",
                                      "To cast a list as an int, the list has to recursively have one element only"],
                 ET_IllegalVariableValue: ["", "", ""],  # Not included?
                 ET_IllegalArgumentValue: ["", "", ""],  # Not included?
                 ET_UndefinedLoopIdentifier: ["Returned when a loop identifier is not defined when referenced",
                                              "iterate 10 :: i {\n"
                                              "    if i == 2 {\n"
                                              "        break @loop\n"
                                              "               ^^^^\n"
                                              "    } else {\n"
                                              "        print(i)\n"
                                              "    }\n"
                                              "}", "A loop identifier must be defined before it can be used"],
                 ET_InvalidIterable: ["Returned when an iterate statement is given a value to iterate over that cannot "
                                      "be iterated over",
                                      "iterate true { ... }\n"
                                      "        ^^^^", ""],
                 ET_IllegalValue: ["Returned when a value is given that cannot be used",
                                   "list a :: [3, 9, 2, 15]\n"
                                   "list b :: [2]"
                                   "print(a[b])\n"
                                   "        ^", "In this example, 'b' is a list and cannot be used to index values "
                                                "in 'a'. A 'correct' way would be print(a[b[0]]) in this example"],
                 ET_IllegalOperation: ["Returned when an operation is performed on a type that does not have the "
                                       "that operation implemented as a trait", "Can't think of anything", ""],
                 ET_ArgumentType: ["Returned when a ", "", ""],  # Check this one over. Not sure what it is. B x
                 ET_TooArgument: ["Returned when too many or too few arguments are passed into a function",
                                  "print(1, 2, 3)\n"
                                  "       ^^^^^^", ""],
                 ET_UnknownAttribute: ["Returned when a function is called and a named argument is given, where the "
                                       "argument name does not exist",
                                       "def example(int arg_1) -> str{ ... }\n"
                                       "print(example(arg_2 :: 3))\n"
                                       "              ^^^^^", ""],
                 ET_UnknownIdentifier: ["Returned when a variable identifier is referenced but the identifier has not "
                                        "been defined",
                                        "print(a)\n"
                                        "      ^", ""],
                 ET_ExpectedCompOp: ["", "", ""]}  # Not included?
