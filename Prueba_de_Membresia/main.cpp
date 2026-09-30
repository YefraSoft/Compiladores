#include <iostream>
#include <vector>
#include <map>
#include <set>
using namespace std;

typedef struct
{
    int G1 = 0, G2 = 0, G3 = 0;
} produ;
typedef vector<produ> grammatrix;
typedef set<int> Gens;
typedef map<int, Gens> row;
typedef map<int, row> table;

bool cykAlgorithm(grammatrix &, string &);
/*
void printCYKTable(const table& virTable, int n) {
    cout << "            ";
    for (int i = 0; i < n; i++) {
        cout << i << "     ";
    }
    cout << endl;
    for (int i = 0; i < n; i++) {
        cout << i << " [";
        for (int j = i; j < n; j++) {
            if (virTable.at(i).count(j)) {
                cout << "[";
                for (const auto& gen : virTable.at(i).at(j)) {
                    cout << gen << ", ";
                }
                cout << "] ";
            } else {
                cout << "... ";
            }
        }
        cout << "]" << endl;
    }
}
*/
int main()
{
    /*
    Original grammar values
    Q-> E ? Q : Q | E
    E-> T + E | T - E | T
    T-> F * T | F / T | F
    F-> (Q) | -F | n
    Q: 81, E: 69, T: 84, F: 70 - is a gennerative Simbols
    ?: 63, : 58 , +: 43, -: 45, *: 42, /: 47, (: 40, ): 41 - is a terminal Simbols
    FNC Grammar Values:
    X: is a terminal Simbols, Y: is a generative Simbols
    X: range is 1000, Y: range is 2000
    X0 -> ?, X1 -> :, X2 -> +, X3 -> -, X4 -> *, X5 -> /, X6 -> (, X7 -> ), X8 -> n
    Y0 -> X0Y1, Y1 -> QY2, Y2 -> X1Q, Y3 -> X2T, Y4 -> X3T, Y5 -> X4F, Y6 -> X5F, Y7 -> QX7
    Q-> EY0 | EY3 | EY4
    E-> EY3 | EY4 | TY5 | TY6
    T-> TY5 | TY6 | X6Y7 | X3F | n
    F-> X6Y7 | X3F | n
    */
    grammatrix grammar;
    // FNC: Grammar
    grammar.push_back({1063, 0, 63}); // X0 -> ?
    grammar.push_back({1058, 0, 58}); // X1 -> :
    grammar.push_back({1043, 0, 43}); // X2 -> +
    grammar.push_back({1045, 0, 45}); // X3 -> -
    grammar.push_back({1042, 0, 42}); // X4 -> *
    grammar.push_back({1047, 0, 47}); // X5 -> /
    grammar.push_back({1040, 0, 40}); // X6 -> (
    grammar.push_back({1041, 0, 41}); // X7 -> )

    grammar.push_back({1081, 1069, 2000}); // Q -> EY0
    grammar.push_back({1081, 1069, 2003}); // Q -> EY3
    grammar.push_back({1081, 1069, 2004}); // Q -> EY4
    grammar.push_back({1081, 1084, 2005}); // Q -> TY5
    grammar.push_back({1081, 1084, 2006}); // Q -> TY6
    grammar.push_back({1081, 1040, 2007}); // Q -> X6Y7
    grammar.push_back({1081, 1045, 1070}); // Q -> X3F
    grammar.push_back({1081, 0, 110});     // Q -> n

    grammar.push_back({1069, 1069, 2003}); // E -> EY3
    grammar.push_back({1069, 1069, 2004}); // E -> EY4
    grammar.push_back({1069, 1084, 2005}); // E -> TY5
    grammar.push_back({1069, 1084, 2006}); // E -> TY6
    grammar.push_back({1069, 1040, 2007}); // E -> X6Y7
    grammar.push_back({1069, 1045, 1070}); // E -> X3F
    grammar.push_back({1069, 0, 110});     // E -> n

    grammar.push_back({1084, 1084, 2005}); // T -> TY5
    grammar.push_back({1084, 1084, 2006}); // T -> TY6
    grammar.push_back({1084, 1040, 2007}); // T -> X6Y7
    grammar.push_back({1084, 1045, 1070}); // T -> X3F
    grammar.push_back({1084, 0, 110});     // T -> n

    grammar.push_back({1070, 1040, 2007}); // F -> X6Y7
    grammar.push_back({1070, 1045, 1070}); // F -> X3F
    grammar.push_back({1070, 0, 110});     // F -> n

    grammar.push_back({2000, 1063, 2001}); // Y0 -> X0Y1
    grammar.push_back({2001, 1081, 2002}); // Y1 -> QY2
    grammar.push_back({2002, 1058, 1081}); // Y2 -> X1Q
    grammar.push_back({2003, 1043, 1084}); // Y3 -> X2T
    grammar.push_back({2004, 1045, 1084}); // Y4 -> X3T
    grammar.push_back({2005, 1042, 1070}); // Y5 -> X4F
    grammar.push_back({2006, 1047, 1070}); // Y6 -> X5F
    grammar.push_back({2007, 1081, 1041}); // Y7 -> QX7

    string input = "";
    cin >> input;
    if (cykAlgorithm(grammar, input))
        cout << "Alcanzable" << endl;
    else
        cout << "Inalcanzable" << endl;
    return 0;
}

bool cykAlgorithm(grammatrix &grammar, string &word)
{
    int n = word.length();
    table virTable;
    for (int i = 0; i < n; i++)
    {
        for (auto &G : grammar)
        {
            if (G.G2 == 0 && G.G3 == word[i])
            {
                virTable[i][i].insert(G.G1);
            }
        }
    }
    for (int j = 2; j <= n; j++)
    {
        for (int i = 0; i < n - j + 1; i++)
        {
            for (int k = 1; k < j; k++)
            {
                for (const auto &prod : grammar)
                {
                    if (virTable[i][k - 1].count(prod.G3) || virTable[i + k][i + j - 1].count(prod.G3))
                    {
                        virTable[i][i + j - 1].insert(prod.G1);
                        virTable[i][i + j - 1].insert(prod.G2);
                    }
                }
            }
        }
    }
    for (const auto &prod : grammar)
    {
        if (virTable[0][n - 1].count(1084))
        {
            return true;
        }
    }
    return false;
}
