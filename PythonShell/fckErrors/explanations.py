# List of names and identifiers
# *_names   -> expanded names of identifiers
# *_format  -> formatting strings
# *_explain -> explanation list


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
ET_IllegalChar = "IllegalChar"
ET_IllegalValueType = "IllegalValueType"
ET_UnexpectedToken = "UnexpectedToken"
ET_UnmatchedBracket = "UnmatchedBracket"
ET_IllegalCastType = "IllegalCastType"
ET_IllegalVariableValue = "IllegalVariableValue"
ET_UndefinedLoopIdentifier = "UndefinedLoopIdentifier"
ET_InvalidIterable = "InvalidIterable"
ET_IllegalValue = "IllegalValueError"
ET_IllegalOperation = "IllegalOperation"
ET_ArgumentType = "ArgumentType"
ET_TooArgument = "TooArgument"
ET_IllegalArgumentValue = "IllegalArgumentValue"
ET_UnknownAttribute = "UnknownAttribute"
ET_UnknownIdentifier = "UnknownIdentifier"

error_names = {ET_ExpectedChar: "Expected character",
               ET_ExpectedExpr: "Expected expression",
               ET_ExpectedType: "Expected type",
               ET_ExpectedSilencaleType: "Expected silencable type",
               ET_ExpectedIdentifier: "Expected identifier",
               ET_ExpectedLoopIdentifier: "Expected loop identifier",
               ET_ExpectedAssignmentOperator: "Expected assignment operator",
               ET_ExpectedAttribute: "Expected attribute",
               ET_IllegalChar: "Illegal character",
               ET_IllegalValueType: "Illegal value type",
               ET_UnexpectedToken: "Unexpected token",
               ET_UnmatchedBracket: "Unmatched bracket",
               ET_IllegalCastType: "Illegal cast type",
               ET_IllegalVariableValue: "Illegal variable value",
               ET_UndefinedLoopIdentifier: "Unidentified loop identifier",
               ET_InvalidIterable: "Invalid iterable",
               ET_IllegalValue: "Illegal value error",
               ET_IllegalOperation: "Illegal operation",
               ET_ArgumentType: "Illegal argument casting",
               ET_TooArgument: "Too arguments",
               ET_IllegalArgumentValue: "Illegal argument value",
               ET_UnknownAttribute: "Unknown attribute",
               ET_UnknownIdentifier: "Unknown identifier"}

error_format = {ET_ExpectedChar: "{details}\n{traceback}",
                ET_ExpectedExpr: "{details}\n{traceback}",
                ET_ExpectedType: "{details}\n{traceback}",
                ET_ExpectedSilencaleType: "{details}\n{traceback}",
                ET_ExpectedIdentifier: "{details}\n{traceback}",
                ET_ExpectedLoopIdentifier: "{details}\n{traceback}",
                ET_ExpectedAssignmentOperator: "{details}\n{traceback}",
                ET_ExpectedAttribute: "{details}\n{arg_explain}\n{traceback}",
                ET_IllegalChar: "{details}\n{traceback}",
                ET_IllegalValueType: "{details}\n{traceback}",
                ET_UnexpectedToken: "{details}\n{traceback}",
                ET_UnmatchedBracket: "{details}\n{traceback}",
                ET_IllegalCastType: "{details}\n{traceback}",
                ET_IllegalVariableValue: "{details}\n{traceback}",
                ET_UndefinedLoopIdentifier: "{details}\n{traceback}",
                ET_InvalidIterable: "{details}\n{traceback}",
                ET_IllegalValue: "{details}\n{traceback}",
                ET_IllegalOperation: "{details}\n{traceback}",
                ET_ArgumentType: "{details}\n{arg_explain}\n{traceback}",
                ET_TooArgument: "{details}\n{arg_explain}\n{traceback}",
                ET_IllegalArgumentValue: "{details}\n{arg_explain}\n{traceback}",
                ET_UnknownAttribute: "{details}\n{arg_explain}\n{traceback}",
                ET_UnknownIdentifier: "{details}\n{traceback}"}

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
                 ET_IllegalVariableValue: ["Returned when a value cannot be assigned to a given variable",
                                           "str example :: [1, 2]\n"
                                           "               ^^^^^^", ""],
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
                 ET_ArgumentType: ["Returned when a function argument does not have the correct type",
                                   "def example(int a) -> int {;return a + 3;}\n"
                                   "example(\"123\")\n"
                                   "         ^^^", ""],
                 ET_TooArgument: ["Returned when too many or too few arguments are passed into a function",
                                  "print(1, 2, 3)\n"
                                  "       ^^^^^^", ""],
                 ET_IllegalArgumentValue: ["Returned when an argument is given that is defined as illegal",
                                           "silent<case> a :: case 5 { ... }\n"
                                           "a.new_case(3, <suite>, @default)\n"
                                           "                       ^^^^^^^^",
                                           "The @default tag is reserved for the default statement"],
                 ET_UnknownAttribute: ["Returned when a function is called and a named argument is given, where the "
                                       "argument name does not exist",
                                       "def example(int arg_1) -> str{ ... }\n"
                                       "print(example(arg_2 :: 3))\n"
                                       "              ^^^^^", ""],
                 ET_UnknownIdentifier: ["Returned when a variable identifier is referenced but the identifier has not "
                                        "been defined",
                                        "print(a)\n"
                                        "      ^", ""]}

############
# Warnings #
############

WT_DivideByZero = "DivideByZero"
WT_ModByZero = "ModByZero"
WT_ValueMultString = "ValueMultString"
WT_StringMultFloat = "StringMultFloat"
WT_InfinityDivValue = "InfinityDivValue"
WT_ValueDivInfinity = "ValueDivInfinity"
WT_InfinityDivInfinity = "InfinityDivInfinity"
WT_ListFromValue = "ListFromValue"
WT_ListIndexOutOfRange = "ListIndexOutOfRange"
WT_ListIndexFloat = "ListIndexFloat"
WT_SilentCaseResetDefault = "SilentCaseReset"
WT_IterateStepLoop = "IterateStepLoop"
WT_IterateStepZero = "IterateStepZero"
WT_ValueFromList = "ValueFromList"
WT_ValueFromString = "ValueFromString"
WT_StringFromValue = "StringFromValue"
WT_NoStringEnd = "NoStringEnd"
WT_FuncArgRet = "FuncArgRet"
WT_FuncAssignOperator = "FuncAssignOperator"
WT_ArgCastError = "ArgCastError"
WT_UnknownGlobalOpt = "UnknownGlobalOpt"

wrn_names = {WT_DivideByZero: "Divide by zero",
             WT_ModByZero: "Modulo by zero",
             WT_ValueMultString: "Value multiplied by string",
             WT_StringMultFloat: "String multiplied by float",
             WT_InfinityDivValue: "Infinity divided by value",
             WT_ValueDivInfinity: "Value divided by infinity",
             WT_InfinityDivInfinity: "Infinity divided by infinity",
             WT_ListFromValue: "List assigned to a value",
             WT_ListIndexOutOfRange: "List index out of range",
             WT_ListIndexFloat: "List index was float",
             WT_SilentCaseResetDefault: "Silent case default reset",
             WT_IterateStepLoop: "Iterate step loop",
             WT_IterateStepZero: "Iterate step zero",
             WT_ValueFromList: "Value from list",
             WT_ValueFromString: "Value from string",
             WT_StringFromValue: "String from value",
             WT_NoStringEnd: "First string delimiter had no ending delimiter on the same line",  # TODO: MAKE INTO ERROR
             WT_FuncArgRet: "Function argument return",
             WT_FuncAssignOperator: "Function assignment operator",
             WT_ArgCastError: "Argument cast error",
             WT_UnknownGlobalOpt: "Unknown global option"}

wrn_format = {WT_DivideByZero: "{details}\n{traceback}",
              WT_ModByZero: "{details}\n{traceback}",
              WT_ValueMultString: "{details}\n{traceback}",
              WT_StringMultFloat: "{details}\n{traceback}",
              WT_InfinityDivValue: "{details}\n{traceback}",
              WT_ValueDivInfinity: "{details}\n{traceback}",
              WT_InfinityDivInfinity: "Infinity divided by infinity",
              WT_ListFromValue: "{details}\n{traceback}",
              WT_ListIndexOutOfRange: "{details}\n{traceback}",
              WT_ListIndexFloat: "{details}\n{traceback}",
              WT_SilentCaseResetDefault: "{details}\n{traceback}",
              WT_IterateStepLoop: "{details}\n{traceback}",
              WT_IterateStepZero: "{details}\n{traceback}",
              WT_ValueFromList: "{details}\n{traceback}",
              WT_ValueFromString: "{details}\n{traceback}",
              WT_StringFromValue: "{details}\n{traceback}",
              WT_NoStringEnd: "{details}\n{traceback}",  # TODO: MAKE INTO ERROR
              WT_FuncArgRet: "{details}\n{traceback}",
              WT_FuncAssignOperator: "{details}\n{traceback}",
              WT_ArgCastError: "{details}\n{traceback}",
              WT_UnknownGlobalOpt: "{details}\n{traceback}"}

wrn_explain = {WT_DivideByZero: ["Raised when a value is divided by zero. Returns an infinity instance",
                                 "int a :: 5 / 0\n"
                                 "         ^^^^^", ""],
               WT_ModByZero: ["Raised when a value is modded by zero. Returns 0",
                              "int a :: 12 % 0\n"
                              "         ^^^^^^", ""],
               WT_ValueMultString: ["Raised when a value is multiplied by a string. This is considered bad practice and"
                                    " is changed into the string multiplied by the value",
                                    ">>> str a :> 3 * \"hello\"\n"
                                    "             ^^^^^^^^^^^\n"
                                    "\"hellohellohello\"",
                                    "Multiplying a value by a string is considered the same as multiplying a string by"
                                    " a value, resulting in the string being repeated n times"],
               WT_StringMultFloat: ["Raised when a string is multiplied by a float. The float is rounded before being"
                                    " used",
                                    ">>> \"abc\" * 1.8\n"
                                    "    ^^^^^^^^^^^\n"
                                    "\"abcabc\"", ""],
               WT_InfinityDivValue: ["Raised when infinity is divided by a numerical value", "", ""],
               WT_ValueDivInfinity: ["", "", ""],
               WT_InfinityDivInfinity: ["", "", ""],
               WT_ListFromValue: ["", "", ""],
               WT_ListIndexOutOfRange: ["", "", ""],
               WT_ListIndexFloat: ["", "", ""],
               WT_SilentCaseResetDefault: ["", "", ""],
               WT_IterateStepLoop: ["", "", ""],
               WT_IterateStepZero: ["", "", ""],
               WT_ValueFromList: ["", "", ""],
               WT_ValueFromString: ["", "", ""],
               WT_StringFromValue: ["", "", ""],
               WT_NoStringEnd: ["", "", ""],
               WT_FuncArgRet: ["", "", ""],
               WT_FuncAssignOperator: ["", "", ""],
               WT_ArgCastError: ["", "", ""],
               WT_UnknownGlobalOpt: ["", "", ""]}
