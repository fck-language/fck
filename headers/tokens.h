#define TT_INT            0
#define TT_FLOAT          1
#define TT_STRING         2
#define TT_PLUS           3
#define TT_MINUS          4
#define TT_MULT           5
#define TT_DIV	      	  6
#define TT_FDIV		        7
#define TT_MOD 		        8
#define TT_POW	          9
#define TT_LPAREN      	  10
#define TT_RPAREN         11
#define TT_LPAREN_CURLY   12
#define TT_RPAREN_CURLY   13
#define TT_LPAREN_SQUARE  14
#define TT_RPAREN_SQUARE  15
#define TT_EOF	       	  16
#define TT_AT 	      	  17
#define TT_NOT	      	  18
#define TT_SEMICOLON  	  19
#define TT_IDENTIFIER	    20
#define TT_KEYWORD	      21
#define TT_QUESTION_MARK  22
#define TT_DOT		        23
#define TT_EQ		          24
#define TT_NE		          25
#define TT_LT		          26
#define TT_GT		          27
#define TT_LTE	      	  28
#define TT_GTE	      	  29
#define TT_COMMA      	  30
#define TT_NEWLINE     	  31
// Just set equivalents are just the number -8
// To check if the token is a SET, it has to be in the range 32 <= x <= 39
// SET_RET is in the range 40 <= x <= 47
// ALL veriable SET or SET_RET tokens are 32 <= x <= 47
// To calculate the type of operator, just get the token of the nested operator
// and add it to either SET or SET_RET
#define TT_SET	      	  32
#define TT_SET_PLUS   	  33
#define TT_SET_MINUS  	  34
#define TT_SET_MULT	      35
#define TT_SET_DIV    	  36
#define TT_SET_FDIV    	  37
#define TT_SET_MOD    	  38
#define TT_SET_POW    	  39
#define TT_SET_RET	      40
#define TT_SET_RET_PLUS	  41
#define TT_SET_RET_MINUS  42
#define TT_SET_RET_MULT   43
#define TT_SET_RET_DIV    44
#define TT_SET_RET_FDIV   45
#define TT_SET_RET_MOD    46
#define TT_SET_RET_POW    47
