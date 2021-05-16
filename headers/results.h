#include "errors.h"

#ifndef RESULTS_H
#define RESULTS_H

class Value;

struct parse_res {
  Error err;
  int tok;
};

struct get_value_res {
  Value value;
  bool is_const;
};

#endif
