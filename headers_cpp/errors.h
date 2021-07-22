#include "bases.h"

#ifndef ERRORS_H
#define ERRORS_H

class Error {
public:
  Position pos_start, pos_end;
  Context context;
  std::string ErrorName, details;
};

#endif
