#include <iostream>
#include <set>
#include <string>

using namespace std;

class OperationStrings {
public:
    /*
        get a set of symbol strings of a random length
    */

    set<string> getRandomSymbolStringSet();
    class operationsSets {
    public:
        template <typename... temSets>
        static set<string> unionSets(temSets &&...sets) {
            set<string> result;
            ((operationsSets::unionSet(result, sets)), ...);
            return result;
        }
        template <typename... temSets>
        static set<string> intersectionSets(temSets &&...sets) {
            set<string> result, base;
            ((operationsSets::intersectionSet(result, base, sets)), ...);
            return result;
        }
        template <typename... temSets>
        static set<string> differenceSets(temSets &&...sets) {
            set<string> result, base;
            ((operationsSets::differenceSet(result, base, sets)), ...);
            return result;
        }
        template <typename... temSets>
        static set<string> concatSets(temSets &&...sets) {
            set<string> result, base;
            ((operationsSets::concatSet(result, base, sets)), ...);
            return result;
        }
        static set<string> powerSet(set<string>&, int);

    private:
        static void unionSet(set<string>&, set<string>&);
        static void intersectionSet(set<string>&, set<string>&, set<string>&);
        static void differenceSet(set<string>&, set<string>&, set<string>&);
        static void concatSet(set<string>&, set<string>&, set<string>&);
    };
    int getSetLengt();
    void setSetLengt(int);
    int getStringsLengt();
    void setStringsLengt(int);
    int getPow();
    void setPow(int);
    char getIndexCharacter();
    void setIndexCharacter(char);
    char getEndCharacter();
    void setEndCharacter(char);

private:
    /*
        generar una secuencia de simbolos aleatorios entre el inicio y el final de
       una logitud establecida
    */
    string randomSymbolSequence();
    int setLengt, stringsLengt, pow;
    char indexCharacter, endCharacter;
};