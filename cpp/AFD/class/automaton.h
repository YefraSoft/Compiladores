#include <iostream>
#include <map>

class automaton
{
private:
    typedef std::map<char, unsigned> event;
    typedef std::map<unsigned, event> transition;
    transition delta;
    int startState = 0;
    bool finalStates[8];

    void zeroToNineEvents(int, int, transition &);

public:
    automaton();
    bool run(const std::string &);
    void fori();
};
