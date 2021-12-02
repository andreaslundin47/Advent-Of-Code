#include <iostream>
#include <regex>
#include <vector>
#include <string>

struct Move
{
    std::string direction;
    int steps;
};

int main()
{
    std::vector<Move> moves;

    std::string line;
    std::smatch match;
    while (std::getline(std::cin, line))
    {
        std::regex_search(line, match, std::regex("([a-z]+) ([0-9]+)"));
        moves.push_back( {match[1], std::stoi(match[2])} );
    }

    // Part 1
    {
        int position = 0, depth = 0;
        for (Move m : moves)
        {
            if (m.direction == "forward") position += m.steps;
            else if (m.direction == "down") depth += m.steps; 
            else if (m.direction == "up") depth -= m.steps; 
        }
        long score = position * depth;
        std::cout << "Part 1. Score: " << score << std::endl;
    }

    // Part 2
    {
        int position = 0, depth = 0, aim = 0;
        for (Move m : moves)
        {
            if (m.direction == "forward")
            {
                position += m.steps;
                depth += aim * m.steps;
            } 
            else if (m.direction == "down") aim += m.steps; 
            else if (m.direction == "up") aim -= m.steps; 
        }
        long score = position * depth;
        std::cout << "Part 2. Score: " << score << std::endl;
    }

    return 0;
}