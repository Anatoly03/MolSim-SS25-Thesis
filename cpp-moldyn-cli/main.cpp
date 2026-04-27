/**
 * @file main.cpp
 * @brief Main entry point for the cpp-moldyn-cli application.
 */

#include <iostream>
#include "Args.h"

/**
 * @brief Entry point for the application.
 */
int main(int argc, char *argv[])
{
    Args().help("This is a simple help message.").parse(argc, argv);

    std::cout << "Hello, C++!\n";

    return 0;
}
