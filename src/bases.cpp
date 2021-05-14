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
