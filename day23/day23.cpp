#include <iostream>
#include <vector>

class Game()
{
private:
    int current;
    int lo;
    int hi;
    vector<int> cups;
public:
    Game(vector<int> data): current{ data[0] }, lo{ min(data) }, hi{ max(data) }
    {
        
    }
};

int main()
{
    return 0;
}