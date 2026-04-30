/**
 * @file Reader.h
 * @author Anatoly Weinstein
 *
 * @brief
 */

#pragma once

#include <fstream>
#include <iostream>
#include <memory>
#include <string>

struct Reader
{
private:
    /**
     * @brief The input file stream for reading particle data. The parser will
     * be selected from the file extension.
     */
    std::unique_ptr<std::ifstream> input_file;

public:
    /**
     * @brief Initializes a file reader for a given file path without consuming
     * the file stream. This will shut down the program if the file cannot be opened.
     */
    Reader(std::string file_path) {
        std::unique_ptr<std::ifstream> pointer = std::make_unique<std::ifstream>(file_path);

        if (!pointer.get()->is_open())
        {
            std::cerr << "Error: Failed to open file at path `" << file_path << "`\n";
            exit(1);
        }

        input_file = std::move(pointer);
    }
};
