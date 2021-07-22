#include <vector>
#include "bases.h"

#ifndef NODES_H
#define NODES_H

class GeneralNode {
public:
  Position pos_start, pos_end;
};

class IntNode: public GeneralNode {
public:
  Token tok;
};

class FloatNode: public GeneralNode {
public:
  Token tok;
};

class BoolNode: public GeneralNode {
public:
  Token tok;
};

class StringNode: public GeneralNode {
public:
  Token tok;
};

class ListNode: public GeneralNode {
public:
  std::vector<Token> elements;
};

class VarAccessNode: public GeneralNode {
public:
  Token tok;
};

class VarGetSetNode: public GeneralNode {
public:
  Token var_name_tok;
  // Some form of range
};

// TODO
// class VarGetItemNode: public GeneralNode {
// public:
//   Token tok;
// };

class VarAssignNode: public GeneralNode {
public:
  Token name_token;
  Value default_value;
  GeneralNode value_node;
  bool ret;
};

class AutoVarAssignNode: public VarAssignNode {};

class VarReassignNode: public GeneralNode {
public:
  Token name_token;
  GeneralNode value_node;
  bool ret;
  Token operator_tok;
};



#endif
