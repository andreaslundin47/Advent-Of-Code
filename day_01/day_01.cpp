#include <iostream>
#include <vector>

int main() 
{
    // Read input from file piped to standard input
    std::vector<int> values;
    int value;
    while (std::cin >> value)
    {
        values.push_back(value);
    }

    // Part 1
    {
        int increases = 0;
        for (int i = 1; i < values.size(); i++)
        {
            if (values[i] > values[i-1]) increases++;
        }
        std::cout << "Part 1. Number of increases: " << increases << std::endl;
    }

    // Part 2
    {
        int increases = 0;
        int previous = values[0] + values[1] + values[2];
        for (int i = 3; i < values.size(); i++) 
        {
            int current = previous - values[i-3] + values[i];
            if (current > previous) increases++;
            previous = current;
        }
        std::cout << "Part 2. Number of increases: " << increases << std::endl;
    }
    
    return 0;
}