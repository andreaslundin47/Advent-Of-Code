#include <iostream>
#include <string>
#include <vector>
#include <cmath>

using binary = std::vector<int>;
using binaryList = std::vector<binary>;

binaryList read_input()
{
    binaryList list;

    std::string line;
    while( std::getline(std::cin, line) )
    {
        binary number;
        for (char c : line) 
        {
            if (c == '1') number.push_back(1);
            if (c == '0') number.push_back(0);
        }
        list.push_back(number);
    }

    return list;
}


int bin2int(binary number)
{
    int value = 0;
    for (int i = 0; i < number.size(); i++)
    {
        value += number[number.size()-1-i] * std::pow(2, i);
    }
    return value;
}


long power_consumption(binaryList numbers)
{
    binary gamma, epsilon;
    for (int i = 0; i < numbers[0].size(); i++)
    {
        int acc = 0;
        for (binary number : numbers)
        {
            acc += number[i];
        }

        2 * acc > numbers.size() ? gamma.push_back(1) : gamma.push_back(0);
        2 * acc > numbers.size() ? epsilon.push_back(0) : epsilon.push_back(1);
    }
    return bin2int(gamma) * bin2int(epsilon);
}


binary bit_filter(binaryList& numbers, bool filterOnMostCommon=true)
{
    int bit_length = numbers[0].size();
    binaryList candidates = numbers;

    for (int i = 0; i < bit_length && 1 < candidates.size(); i++)
    {
        binaryList ones, zeros;

        for (binary number : candidates)
            number[i] ? ones.push_back(number) : zeros.push_back(number);

        if (filterOnMostCommon)
            candidates = ones.size() >= zeros.size() ? ones : zeros;
        else
            candidates = zeros.size() <= ones.size() ? zeros : ones;
    }
    return candidates[0];
}


long life_support_rating(binaryList numbers)
{
    binary ogr = bit_filter(numbers, true);
    binary csr = bit_filter(numbers, false);
    return bin2int(ogr) * bin2int(csr);
}


int main()
{
    binaryList number_list = read_input();
    std::cout << "Part 1. Power Consumption: " << power_consumption(number_list) << std::endl;
    std::cout << "Part 2. Life Support Rating: " << life_support_rating(number_list) << std::endl;
    return 0;
}