#include <map>
#include <string>

#ifndef BASES_H
#define BASES_H

class Context;
struct get_value_res;

struct Position {
  int ln, col;
};

class Value {
public:
  Position pos_start, pos_end;
  Context *context;
};

class SymbolTable {
public:
  std::map<std::string, Value> symbols;
  std::map<std::string, Value> const_symbols;
  std::map<std::string, bool> options;
  SymbolTable *parent;
  get_value_res get(std::string name);
  void set(std::string name, Value value);
  void set_const(std::string name, Value value);
};

struct Context {
  std::string display_name;
  Context *parent;
  SymbolTable symbol_table;
};

class MasterPosition {
public:
  int index, ln, col;
  char current_char;
  std::string text;
  void advance();
  MasterPosition copy();
  Position make_position();
};

class Token {
public:
  int type;
  std::string value;
  Position pos_start, pos_end;
  bool matches(int other_type, std::string other_value);
  bool list_matches(int other_type, std::vector<std::string> other_value_list);
  operator std::string () const {
    return "'" + value + "'";
  }
};

#endif
