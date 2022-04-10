#include <iostream>
#include <Windows.h>
#include <sstream>

using namespace std;

int main() {
	const char* username = "amane";
	int username_len = strlen(username);
	int factor = 26;
	ostringstream serial_buffer;
	for (int i = 0; i < username_len; ++i) {
		int operand = username[i] ^ factor;
		int reminder = operand % username_len;
		serial_buffer << username[reminder];
	}
	cout << serial_buffer.str() << endl;
}