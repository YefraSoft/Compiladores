#include "operationsStrings.h"

set<string> OperationStrings::getRandomSymbolStringSet() {
    set<string> stringSet;
    for (int i = 0; i < setLengt; i++) {
        string newStr = OperationStrings::randomSymbolSequence();
        if (newStr.length() == 0) {
            stringSet.insert("");
        }
        stringSet.insert(newStr);
    }
    return stringSet;
}

string OperationStrings::randomSymbolSequence() {
    string str;
    int lengt = rand() % stringsLengt;
    if (stringsLengt > 0) {
        
        for (int i = 0; i < lengt; i++) {
            str += rand() % (endCharacter - indexCharacter + 1) + indexCharacter;
        }
    }
    return str;
}

void OperationStrings::operationsSets::unionSet(set<string>& rest, set<string>& set) {
    for (string str : set) {
        rest.insert(str);
    }
}

void OperationStrings::operationsSets::intersectionSet(set<string>& rest, set<string>& base, set<string>& set) {
    if (base.empty()) {
        base = set;
    }
    else {
        for (string str : set) {
            if (base.count(str)) {
                rest.insert(str);
            }
            else {
                rest.erase(str);
            }
        }
        base = rest;
    }
}

void OperationStrings::operationsSets::differenceSet(set<string>& rest, set<string>& base, set<string>& set) {
    if (base.empty()) {
        base = set;
    }
    else {
        for (string str : set) {
            if (base.count(str)) {
                rest.erase(str);

            }
            else {
                rest.insert(str);
            }
        }
        base = rest;
    }
}

void OperationStrings::operationsSets::concatSet(set<string>& rest, set<string>& base, set<string>& set) {
    if (base.empty()) {
        base = set;
    }
    else {
        for (string strA : base) {
            for (string strB : set) {
                rest.insert(strA + strB);
            }
        }
    }
}

set<string> OperationStrings::operationsSets::powerSet(set<string>& setA, int pow) {
    set<string> result, base;
    if (pow == 0) {
       result.insert("");
    }
    else if (pow == 1) {
        return setA;
    }
    else {
        for (int i = 0; i < pow; i++) {
            OperationStrings::operationsSets::concatSet(result, base, setA);
        }
    }
    return result;
}

int OperationStrings::getSetLengt() { return OperationStrings::setLengt; }

void OperationStrings::setSetLengt(int SetLengt) {
    OperationStrings::setLengt = SetLengt;
}

int OperationStrings::getStringsLengt() {
    return OperationStrings::stringsLengt;
}

void OperationStrings::setStringsLengt(int stringsLengt) {
    OperationStrings::stringsLengt = stringsLengt;
}

int OperationStrings::getPow() { return OperationStrings::pow; }

void OperationStrings::setPow(int pow) { OperationStrings::pow = pow; }

char OperationStrings::getIndexCharacter() {
    return OperationStrings::indexCharacter;
}

void OperationStrings::setIndexCharacter(char indexCharacter) {
    OperationStrings::indexCharacter = indexCharacter;
}

char OperationStrings::getEndCharacter() {
    return OperationStrings::endCharacter;
}

void OperationStrings::setEndCharacter(char endCharacter) {
    OperationStrings::endCharacter = endCharacter;
}