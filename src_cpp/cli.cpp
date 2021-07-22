#ifdef WINDOWS
#include <direct.h>
#define GetCurrentDir _getcwd
#else
#include <unistd.h>
#define GetCurrentDir getcwd
#endif

#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <string>
#include <json> //value.h>
#include <cmath>

using namespace std;

ifstream error_codes_file("~/Documents/Code/.fck/source/fck/error_codes.json", ifstream::binary);
error_codes_file >> error_codes;

string get_current_dir() {
   char buff[FILENAME_MAX];
   GetCurrentDir( buff, FILENAME_MAX );
   string current_working_dir(buff);
   return current_working_dir;
}

string get_errWarn_code(bool error = true, int code) {
  string out = to_string(code);
  out = string(3 - out.length(), '0') + out;
  string returned = error_codes[(error ? "E" : "W") + out];
  return returned;
}

int main(int argc, char const *argv[]) {
  cout << get_current_dir() << endl;
  for (int i = 1; i < argc; i++) {
    cout << argv[i] << endl;
  }
  return 0;
}
