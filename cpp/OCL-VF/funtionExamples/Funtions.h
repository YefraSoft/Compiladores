#include "../operationsLibrary/operationsStrings.h"

class Funtions {
public:

    static void languageOperations();

private:
    static void userImputs(OperationStrings&);
    static void userImputs(OperationStrings&, bool);
    template <typename... setsString>
    static void printSets(setsString &&...sets) {
        int counter = 0;
        ((printSet(sets, ++counter)), ...);
    }
    static void printSet(set<string>&, int);
    static void printSet(set<string>&, string);
};
