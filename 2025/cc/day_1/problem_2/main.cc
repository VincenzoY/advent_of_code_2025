#include <iostream>
#include <fstream>
#include <string>

int main() {
    std::ifstream input ("../../input.txt");
    std::string line;
    size_t zeros_count = 0;
    int dial_position = 50;
    while (input >> line) {
        char direction = line[0];
        int length = stoi(line.substr(1, line.length() - 1));

        size_t full_spins = length / 100;
        int remaining_length = length - full_spins * 100;
        zeros_count += full_spins;

        if (remaining_length == 0) continue;

        int new_dial_position = direction == 'L' ? dial_position - remaining_length : dial_position + remaining_length;

        if ((new_dial_position <= 0 || new_dial_position >= 100) && dial_position != 0) zeros_count += 1;

        dial_position = (new_dial_position + 100) % 100;
    }

    std::cout << zeros_count << std::endl;
}