#include <iostream>
#include <string>
#include <vector>
#include "../headers/tokens.h"
#include "../headers/bases.h"
#include "../headers/results.h"
#include "../headers/errors.h"

using namespace std;

class Token{
public:
  Position pos_start, pos_end;
  int type_;
  string value;
  operator string () const {
        return "'" + value + "'";
    }
};

class tokenise{
public:
  MasterPosition position;
  vector<Token> tokens;
  void parse(){
    position.index = 0;
    position.ln = 0;
    position.col = 0;
    position.current_char = position.text[0];
    while (position.index < position.text.length()) {
      if (position.current_char == ' ' or position.current_char == '\t') {
        position.advance();
        continue;
      }
      if (isalpha(position.current_char)) {
        tokens.push_back(make_identifier());
      }
      else if (isdigit(position.current_char) or position.current_char == '.') {
        tokens.push_back(number_make());
      }
      else {
        int tok = single_char_token();
        Token tok_to_append;
        tok_to_append.pos_start = position.make_position();
        if (tok != 0) {
          tok_to_append.pos_end = position.make_position();
          tok_to_append.type_ = tok;
          tokens.push_back(tok_to_append);
          position.advance();
          continue;
        }
      }
    }
  }
private:
  Token make_identifier() {
    Token out;
    out.pos_start = position.make_position();
    out.type_ = TT_IDENTIFIER;
    while (isalpha(position.current_char) or isdigit(position.current_char) or position.current_char == '_') {
      out.value += char(position.current_char);
      position.advance();
    }
    out.pos_end = position.make_position();
    return out;
  }
  Token word_make() {
    Token out;
    out.pos_start = position.make_position();
    out.type_ = TT_IDENTIFIER;
    while (isalpha(position.current_char) or isdigit(position.current_char) or position.current_char == '_') {
      out.value += char(position.current_char);
      position.advance();
    }
    return out;
  }
  Token number_make() {
    Token out;
    out.pos_start = position.make_position();
    bool has_dot;
    while (isdigit(position.current_char) or position.current_char == '.') {
      if (position.current_char == '.') {
        if (has_dot) {
          break;
        }
        has_dot = true;
        out.value += '.';
        continue;
      }
      else {
        out.value += char(position.current_char);
      }
      position.advance();
    }
    out.pos_end = position.make_position();
    return out;
  }
  int single_char_token() {
    switch (position.current_char) {
      case '+':  return TT_PLUS;
      case '-':  return TT_MINUS;
      case '%':  return TT_MOD;
      case '(':  return TT_LPAREN;
      case ')':  return TT_RPAREN;
      case '{':  return TT_LPAREN_CURLY;
      case '}':  return TT_RPAREN_CURLY;
      case '[':  return TT_LPAREN_SQUARE;
      case ']':  return TT_RPAREN_SQUARE;
      case ',':  return TT_COMMA;
      case '\n': return TT_NEWLINE;
      case ';':  return TT_NEWLINE;
      case '?':  return TT_QUESTION_MARK;
      case '@':  return TT_AT;
      default: return 0;
    }
  }
  parse_res double_char_token() {
    parse_res out;
    switch (position.current_char) {
      case '!':
        position.advance();
        if (position.current_char == '=') {
          out.tok = TT_NE;
          position.advance();
        }
        else {
          out.tok = TT_NOT;
        }
        return out;
    }
  }
};

int main(int argc, char const *argv[]) {
  tokenise parser;
  getline(cin, parser.position.text);
  parser.parse();
  cout << "Tokens : ";
  for (auto i : parser.tokens) {
    cout << endl << string(i);
  }
  cout << endl;
  return 0;
}
