#include <iostream>
#include <string>
#include <vector>
#include <map>
#include "../headers/tokens.h"
#include "../headers/bases.h"
#include "../headers/results.h"
#include "../headers/errors.h"

using namespace std;

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
      else if (position.current_char == '\'' or position.current_char == '"') {
        tokens.push_back(word_make());
      }
      else {
        Token tok_to_append;
        tok_to_append.pos_start = position.make_position();
        int tok = single_char_token();
        if (tok != 0) {
          tok_to_append.pos_end = position.make_position();
          tok_to_append.type = tok;
          tokens.push_back(tok_to_append);
          position.advance();
          continue;
        }
        parse_res multi_tok = double_char_token();
        if (multi_tok.tok != 0) {
          tok_to_append.pos_end = position.make_position();
          tok_to_append.type = multi_tok.tok;
          tokens.push_back(tok_to_append);
          continue;
        } else if (position.current_char == ':') {
          position.advance();
          if (position.current_char == ':' or position.current_char == '>') {
            tok_to_append.type = position.current_char == ':' ? TT_SET : TT_SET_RET;
          } else if (position.current_char == ' ') {
            tok_to_append.type = TT_COLON;
          } else {
            tok = single_char_token();
            if (tok < 6 and tok > 2) {
              tok_to_append.type = tok;
              position.advance();
            } else {
              multi_tok = double_char_token();
              if (multi_tok.tok > 5 and multi_tok.tok < 10) {
                tok_to_append.type = multi_tok.tok;
              } else {
                break;
              }
            }
            cout << char(position.current_char) << endl;
            if (not (position.current_char == ':' or position.current_char == '>')) {
              break;
            }

            tok_to_append.type += position.current_char == ':' ? 30 : 38;
          }

          tok_to_append.pos_end = position.make_position();
          tokens.push_back(tok_to_append);
          position.advance();
          continue;
        }
        break;
      }
    }
  }
private:
  Token make_identifier() {
    Token out;
    out.pos_start = position.make_position();
    while (isalpha(position.current_char) or isdigit(position.current_char) or position.current_char == '_') {
      out.value += char(position.current_char);
      position.advance();
    }
    out.type = TT_KEYWORD;
    if (find(begin(KEYWORDS), end(KEYWORDS), out.value) == end(KEYWORDS)) {
      if (find(begin(VAR_KEYWORDS), end(VAR_KEYWORDS), out.value) == end(VAR_KEYWORDS)) {
        if (find(begin(NON_STATIC_VAR_KEYWORDS), end(NON_STATIC_VAR_KEYWORDS), out.value) == end(NON_STATIC_VAR_KEYWORDS)) {
          out.type = TT_IDENTIFIER;
        }
      }
    }
    out.pos_end = position.make_position();
    return out;
  }
  Token word_make() {
    Token out;
    out.pos_start = position.make_position();
    out.type = TT_STRING;
    char end_char = position.current_char;
    position.advance();

    map<char, char> escape_chars;
    escape_chars['n'] = '\n';
    escape_chars['t'] = '\t';

    bool escaped = false;

    while (position.current_char != end_char and position.current_char != NULL) {
      if (escaped) {
        auto pos = escape_chars.find(char(position.current_char));
        if (pos == escape_chars.end()) {
          out.value += char(position.current_char);
        } else {
          out.value += pos->second;
        }
        escaped = false;
      } else {
        if (position.current_char == '\\') {
          escaped = true;
        } else {
          out.value += char(position.current_char);
        }
      }
      position.advance();
    }
    position.advance();
    out.pos_end = position.make_position();
    return out;
  }
  Token number_make() {
    Token out;
    out.pos_start = position.make_position();
    bool has_dot = false;
    while (isdigit(position.current_char) or position.current_char == '.') {
      if (position.current_char == '.') {
        if (has_dot) {
          break;
        }
        has_dot = true;
        out.value += '.';
      }
      else {
        out.value += char(position.current_char);
      }
      position.advance();
    }
    out.pos_end = position.make_position();
    out.type = has_dot ? TT_FLOAT : TT_INT;
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
    out.tok = 0;
    // Setting default error
    Error err;
    err.pos_start = position.make_position();
    err.pos_end = position.make_position();
    out.err = err;
    switch (position.current_char) {
      // != or !
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

      // == or error
      case '=':
        position.advance();
        if (position.current_char == '=') {
          out.tok = TT_EQ;
          position.advance();
        }
        return out;

      // < or <=
      case '<':
        out.tok = TT_LT;
        position.advance();
        if (position.current_char == '=') {
          out.tok = TT_LTE;
          position.advance();
        }
        return out;

      // > or >=
      case '>':
        out.tok = TT_GT;
        position.advance();
        if (position.current_char == '=') {
          out.tok = TT_GTE;
          position.advance();
        }
        return out;

      // * or **
      case '*':
        out.tok = TT_MULT;
        position.advance();
        if (position.current_char == '*') {
          out.tok = TT_POW;
          position.advance();
        }
        return out;

      // div or fdiv
      case '/':
        out.tok = TT_DIV;
        position.advance();
        if (position.current_char == '/') {
          out.tok = TT_FDIV;
          position.advance();
        }
        return out;
    }
    return out;
  }
};

int main(int argc, char const *argv[]) {
  tokenise parser;
  getline(cin, parser.position.text);
  parser.parse();
  cout << "Tokens : ";
  for (auto i : parser.tokens) {
    cout << endl << "'" << i.value << "' | " << i.type;
  }
  cout << endl;
  return 0;
}
