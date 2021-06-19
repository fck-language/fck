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
ET_IllegalArgumentValue = "IllegalArgumentValue"
ET_UndefinedLoopIdentifier = "UndefinedLoopIdentifier"
ET_InvalidIterable = "InvalidIterable"
ET_IllegalValueError = "IllegalValueError"
ET_IllegalOperation = "IllegalOperation"
ET_IllegalArgumentCasting = "ArgumentTypeError"
ET_TooArgument = "TooArgumentError"
ET_UnknownAttribute = "UnknownAttributeError"
ET_UnknownIdentifier = "UnknownIdentifier"
ET_ExpectedCompOp = "ExpectedCompOp"

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
               ET_IllegalValueError: "Illegal value error",
               ET_IllegalOperation: "Illegal operation",
               ET_IllegalArgumentCasting: "Illegal argument casting",
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
                ET_IllegalValueError: "{details}\n{traceback}",
                ET_IllegalOperation: "{details}\n{traceback}",
                ET_IllegalArgumentCasting: "{details}\n{arg_explain}\n{traceback}",
                ET_TooArgument: "{details}\n{arg_explain}\n{traceback}",
                ET_UnknownAttribute: "{details}\n{arg_explain}\n{traceback}",
                ET_UnknownIdentifier: "{details}\n{traceback}",
                ET_ExpectedCompOp: "{details}\n{traceback}"}

error_explain = {ET_ExpectedChar: ["Returned when a character was expected but not found",
                                   'print(3 ? "hello")\n'
                                   '                 ^\n'
                                   'A \':\' was expected with the \'?\' but was not found'],
                 ET_ExpectedExpr: ["Returned when an expression was expected but not found",
                                   "auto example_variable\n"
                                   "                     ^\n"
                                   "'auto' type variables must be initialised with a value, so leaving this blank "
                                   "raised an error"],
                 ET_ExpectedType: ["Returned when a type was expected but not found",
                                   "print(1.0 as )\n"
                                   "             ^\n"
                                   "When casting a value, a type must be given to cast to"],
                 ET_ExpectedSilencaleType: ["Returned when a silent type variable is made, but the given type is not "
                                            "silencable",
                                            "silent<int>\n"
                                            "       ^^^\n"
                                            "'int' is not a silencable type, so a 'silent<int>' type cannot be made"],
                 ET_ExpectedIdentifier: ["Returned when an identifier was expected and not found",
                                         "int :: 3\n"
                                         "    ^^"],
                 ET_ExpectedLoopIdentifier: ["Returned when '@' is not followed by an identifier",
                                             "@ iterate 10 { ... }\n"
                                             " ^"],
                 ET_ExpectedAssignmentOperator: ["Returned when a variable with no default value is initialised and not"
                                                 "given a value",
                                                 "auto a\n"
                                                 "      ^\n"
                                                 "When initialising a variable with the 'auto' keyword, a value must "
                                                 "be given to initialise it to"],
                 ET_ExpectedAttribute: ["Returned when a '.' is not followed by an attribute identifier",
                                        "(silent<case> example :> case(5){}).\n"
                                        "                                    ^"],
                 ET_InvalidGlobalOption: ["", ""],
                 ET_IllegalChar: ["", ""],
                 ET_IllegalValueType: ["", ""],
                 ET_UnexpectedToken: ["", ""],
                 ET_UnmatchedBracket: ["", ""],
                 ET_IllegalCastType: ["", ""],
                 ET_IllegalVariableValue: ["", ""],
                 ET_IllegalArgumentValue: ["", ""],
                 ET_UndefinedLoopIdentifier: ["", ""],
                 ET_InvalidIterable: ["", ""],
                 ET_IllegalValueError: ["", ""],
                 ET_IllegalOperation: ["", ""],
                 ET_IllegalArgumentCasting: ["", ""],
                 ET_TooArgument: ["", ""],
                 ET_UnknownAttribute: ["", ""],
                 ET_UnknownIdentifier: ["", ""],
                 ET_ExpectedCompOp: ["", ""]}
