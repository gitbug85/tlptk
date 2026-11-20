#include <iostream>
#include <bitset>
using namespace std;

int main() {
    int number;

    cout << "Enter a decimal number: ";
    cin >> number;

    cout << "Binary: " << bitset<8>(number) << endl;

    return 0;
}
