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
    std::string input_path;

    Args()
        .required_details(&input_path, "The input file for the simulation. The parser will be selected from the file extension.")
        .help("Molecular Dynamics Thesis Code. This library implements a simple engine to simulate molecular dynamics.")
        .parse(argc, argv);

    std::cout << "Hello, C++!\n";

    return 0;
}
