#include "automaton.h"

automaton::automaton()
{
    delta[startState][static_cast<char>('0')] = 1;
    zeroToNineEvents(startState, 2, delta); // X0 -> X2
    delta[1]['.'] = 3; // X1 -> X3
    delta[2]['.'] = 3; // X2 -> X3
    zeroToNineEvents(2, 2, delta); // X2 -> X2
    zeroToNineEvents(3, 4, delta); // X3 -> X4
    delta[4]['e'] = 5; // X4 -> X5
    delta[4]['E'] = 5; // X4 -> X5
    zeroToNineEvents(4, 4, delta); // X4 -> X4
    zeroToNineEvents(5, 7, delta); // X5 -> X7
    delta[5]['+'] = 6; // X5 -> X6
    delta[5]['-'] = 6; // X5 -> X6
    zeroToNineEvents(6, 7, delta); // X6 -> X7
    zeroToNineEvents(7, 7, delta); // X7 -> X7
    finalStates[1] = true;
    finalStates[2] = true;
    finalStates[4] = true;
    finalStates[7] = true;
}

void automaton::fori()
{
    std::map<char, unsigned>::iterator st;
    for (size_t i = 0; i < 5; i++)
    {
        for (st = delta[i].begin(); st != delta[i].end(); st++)
        {
            std::cout << "- " << st->first << "> " << st->second << std::endl;
        }
    }
}

bool automaton::run(const std::string &w)
{
    std::map<char, unsigned>::iterator st;
    int state = startState;
    for (auto &&i : w)
    {
        st = delta[state].find(i);
        if (st != delta[state].end())
            state = st->second;
        else
            return false;
    }
    if (finalStates[state])
    {
        return true;
    }
    return false;
}

void automaton::zeroToNineEvents(int actualState, int stateTransition, transition &delta)
{
    if (actualState == startState)
    {
        for (int cont = 1; cont < 10; cont++)
        {
            delta[actualState][static_cast<char>('0' + cont)] = stateTransition;
        }
    }
    else
    {
        for (int cont = 0; cont < 10; cont++)
        {
            delta[actualState][static_cast<char>('0' + cont)] = stateTransition;
        }
    }
}
