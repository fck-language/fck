#ifndef BASES_H
#define BASES_H

struct Position {
  int ln, col;
};

struct Context {
  std::string display_name;
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

#endif
