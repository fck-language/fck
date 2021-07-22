#include <iostream>
#include "../headers/bases.h"

void MasterPosition::advance() {
  index ++;
  col ++;
  if (index >= text.length()) {
    current_char = NULL;
    return;
  }
  current_char = text[index];
  if (current_char == '\n'){
    ln ++;
    col = 0;
  }
}

MasterPosition MasterPosition::copy() {
  MasterPosition out;
  out.index = index;
  out.ln = ln;
  out.col = col;
  out.current_char = current_char;
  out.text = text;
  return out;
}

Position MasterPosition::make_position() {
  Position out;
  out.ln = ln;
  out.col = col;
  return out;
}

bool Token::matches(int other_type, std::string other_value) {
  return type == other_type and value == other_value;
}

// TODO work out how to check if value is in a list
// bool Token::list_matches(int other_type, std::vector<std::string> other_value_list) {
//   return type == other_type and value in other_value_list;
// }
