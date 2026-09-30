#include "Funtions.h"
#include <sstream>

void Funtions::userImputs(OperationStrings &operationStrings) {
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

void Funtions::userImputs(OperationStrings &operationStrings, bool usePow) {
  char indexWords, endWords;
  int n, m, pow;
  cout << ":> ";
  cin >> n >> m >> pow >> indexWords >> endWords;
  operationStrings.setSetLengt(n);
  operationStrings.setStringsLengt(m);
  operationStrings.setPow(pow);
  operationStrings.setIndexCharacter(indexWords);
  operationStrings.setEndCharacter(endWords);
  /*
cout << "Enter the length of the string set: ";
cin >> n;

cout << "Enter a string lengt: ";
cin >> m;

cout << "Enter a string pow: ";
cin >> pow;

cout << "Enter the first element of the string: ";
cin >> indexWords;

cout << "Enter the last element of the string: ";
cin >> endWords;

  */
}

void Funtions::printSet(set<string> &set, int index) {
  char setLabel = 'A' + (index - 1);
  cout << setLabel << ": [ ";
  for (const string &str : set) {
    if (str.empty()) {
      cout << "<void>"
           << " ";
    } else {
      cout << str << " ";
    }
  }
  cout << "]" << endl;
}

void Funtions::printSet(set<string> &set, string title) {
  string setLabel = title;
  if (set.empty()) {
    cout << endl;
  } else {
    cout << setLabel << ":" << endl << "[ ";
    for (const string &str : set) {
      if (str == "") {
        cout << "<void>";
      } else {
        cout << str << " ";
      }
    }
    cout << "]" << endl;
  }
}