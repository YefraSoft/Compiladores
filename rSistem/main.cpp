#include "class/automaton.h"
#include <set>
using namespace std;

void printSet(set<string> &set, string title)
{
    cout << title << ":" << endl;
    if (set.empty())
    {
        cout << "[  ]" << endl;
    }
    else
    {
        cout << "[ ";
        for (const string &str : set)
        {
            cout << str << " ";
        }
        cout << "]" << endl;
    }
}

int main()
{
    automaton rSystem;
    set<string> recognizedString;
    set<string> refusedString;
    set<string> strings;
    string word = "";
    int leng = 0;
    cout << ":> ";
    cin >> leng;
    for (int i = 0; i < leng; i++)
    {
        cin >> word;
        strings.insert(word);
    }

    for (const auto &str : strings)
    {
        if (rSystem.run(str))
        {
            recognizedString.insert(str);
        }
        else
        {
            refusedString.insert(str);
        }
    }
    printSet(recognizedString, "Cadenas reconocidas");
    printSet(refusedString, "Cadenas rechazadas");
    return 0;
}
