#include "Funtions.h"

void Funtions::userImputs(OperationStrings& operationStrings) {
    char indexWords, endWords;
    int n, m;
    cout << "Enter the length of the string set: ";
    cin >> n;
    operationStrings.setSetLengt(n);
    cout << "Enter a string lengt: ";
    cin >> m;
    operationStrings.setStringsLengt(m);
    cout << "Enter the first element of the string: ";
    cin >> indexWords;
    operationStrings.setIndexCharacter(indexWords);
    cout << "Enter the last element of the string: ";
    cin >> endWords;
    operationStrings.setEndCharacter(endWords);
}
void Funtions::userImputs(OperationStrings& operationStrings, bool usePow) {
    char indexWords, endWords;
    int n, m, pow;
    cout << "Enter the length of the string set: ";
    cin >> n;
    operationStrings.setSetLengt(n);
    cout << "Enter a string lengt: ";
    cin >> m;
    operationStrings.setStringsLengt(m);
    cout << "Enter a string pow: ";
    cin >> pow;
    operationStrings.setPow(pow);
    cout << "Enter the first element of the string: ";
    cin >> indexWords;
    operationStrings.setIndexCharacter(indexWords);
    cout << "Enter the last element of the string: ";
    cin >> endWords;
    operationStrings.setEndCharacter(endWords);
}
void Funtions::printSet(set<string>& set, int index) {
    char setLabel = 'A' + (index - 1);
    cout << setLabel << ": [ ";
    for (const string& str : set) {
        cout << str << " ";
    }
    cout << "]" << endl;
}
void Funtions::printSet(set<string>& set, string title) {
    string setLabel = title;
    if (set.empty()) {
        cout << "[  ]" << endl;
    }
    else {
        cout << setLabel << ":" << endl << "[ ";
        for (const string& str : set) {
            cout << str << " ";
        }
        cout << "]" << endl;
    }
}