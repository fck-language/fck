#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <ctype.h>
using namespace std;

/* Parser result types */
string SPACE = "SPACE";
string ID = "ID";
string OPERATOR = "OPERATOR";
string NUMBER = "NUMBER";
string LEFT_ROUND_BRACKET = "LEFT_ROUND_BRACKET";
string RIGHT_ROUND_BRACKET = "RIGHT_ROUND_BRACKET";
string LEFT_CURVED_BRACKET = "LEFT_CURVED_BRACKET";
string RIGHT_CURVED_BRACKET = "RIGHT_CURVED_BRACKET";


class ParserType {
public:
	string Type;
	string contents;
};

ParserType stringParser(string line, int string_char_index) {
	ParserType res;
	res.Type = ID;
	string contents;
	contents += line[string_char_index];
	string_char_index ++;
	char current_char = line[string_char_index];
	while (isalpha(current_char) || isdigit(current_char) || ('_' == current_char)) {
		contents += current_char;
		string_char_index ++;
		current_char = line[string_char_index];
	}
	cout << contents;
	res.contents = contents;
	return res;
}

ParserType numberParser(string line, int string_char_index) {
	ParserType res;
	res.Type = NUMBER;
	string contents;
	contents += line[string_char_index];
	string_char_index ++;
	char current_char = line[string_char_index];
	while (isdigit(current_char) || ('.' == current_char)) {
		contents += current_char;
		string_char_index ++;
		current_char = line[string_char_index];
	}
	cout << contents;
	res.contents = contents;
	return res;
}

ParserType semicolonLexer(string line, int string_char_index) {
	ParserType res;
	res.Type = OPERATOR;
	if (string_char_index + 1 == line.length()) {
		res.contents = ":";
	}
	else if (line[string_char_index + 1] == ':') {
		res.contents = "::";
	}
	return res;
}


void lineParser(string line) {
	int characters = line.length();
	int current_char_index = 0;
	vector<ParserType> res;
	ParserType id_res;
	while (current_char_index < characters) {
		char current_char = line[current_char_index];
		if (isalpha(current_char)) {
			id_res = stringParser(line, current_char_index);
			current_char_index += id_res.contents.length() - 1;
		}
		else if (isdigit(current_char)) {
			id_res = numberParser(line, current_char_index);
			current_char_index += id_res.contents.length() - 1;
		}
		else {
			switch (current_char) {
				case '(':
					id_res.Type = LEFT_ROUND_BRACKET;
					id_res.contents = "(";
					break;
				case ')':
					id_res.Type = RIGHT_ROUND_BRACKET;
					id_res.contents = ")";
					break;
				case ':':
					id_res = semicolonLexer(line, current_char_index);
					current_char_index += id_res.contents.length() - 1;
					break;
				default:
					id_res.Type = SPACE;
					id_res.contents = " ";
					break;
			}
			cout << current_char;
		}
		res.push_back(id_res);
		current_char_index ++;
	}
	for (const auto& item: res) {
		cout << item.contents;
	}
	cout << "\n";
}

int main(int argc, char const *argv[]) {
	fstream readFile;
	readFile.open("test.fck", ios::in);
	if (readFile.is_open()){   //checking whether the file is open
    string tp;
    while(getline(readFile, tp)) {
    	lineParser(tp);
    }
    readFile.close();
   }
	return 0;
}
