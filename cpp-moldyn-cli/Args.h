/**
 * @file Args.h
 * @author Anatoly Weinstein
 * @brief Definition of the Args class for CLI argument parsing,
 * wrapping around getopt_long for parsing.
 */

#pragma once

#include <getopt.h>
#include <string>
#include <vector>

// todo. remove
#include <iostream>

/**
 * @brief A CLI argument parser inspired by the [argparse](https://github.com/p-ranav/argparse)
 * library for C++.
 *
 * # Example
 *
 * ```cpp
 * int main(int argc, char* argv[]) {
 *     int int_arg;
 *     double double_arg;
 *     std::string string_arg;
 *
 *     Args()
 *         .required<int>('i', &int_arg)
 *         .required<double>('d', &double_arg)
 *         .required<std::string>('s', "string", &string_arg)
 *         .help("This is a help message for the CLI application.")
 *         .parse(argc, argv);
 * }
 * ```
 */
struct Args
{
private:
    std::string optstring;
    std::vector<const option> options;
    int positional_arguments = 0;

    /**
     * @brief Prints the help message and exits the application.
     */
    [[noreturn]]
    void print_help() {
        std::cout << "Help message for the CLI application.\n";
        exit(0);
    }


public:
    /**
     * @brief Initializes an empty argument parser.
     */
    Args() = default;

    // remove implicit copy constructors
    // https://stackoverflow.com/questions/33776697/deleting-copy-constructors-and-copy-assignment-operators-which-of-them-are-esse
    Args(const Args &) = delete;
    Args &operator=(const Args &) = delete;

    /**
     * @brief Registers a required positional argument with a reference.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required<int>(&int_arg)
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required(T *value)
    {
        positional_arguments += 1;

        // TODO
        return *this;
    }

    /**
     * @brief Registers a required positional argument with a brief details message.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required<int>(&int_arg, "This is an integer argument.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required(T *value, char *details)
    {
        // TODO
        return *this;
    }

    /**
     * @brief Registers a required argument with a short name and a reference.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app -i <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required<int>('i', &int_arg)
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required(char short_name, T *value)
    {
        // TODO
        return *this;
    }

    /**
     * @brief Registers a required argument with a short name, a reference and a
     * brief details message.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app -i <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required<int>('i', &int_arg, "This is an integer argument.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required(char short_name, T *value, char *details)
    {
        // TODO
        return *this;
    }

    /**
     * @brief Registers a required argument with a short and long name,
     * and a reference.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the commands `./app -i <int_arg>`
     * // and `./app --integer <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required<int>('i', "integer", &int_arg)
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required(char short_name, char *long_name, T *value)
    {
        // TODO
        return *this;
    }

    /**
     * @brief Registers a required argument with a short name, long name, a
     * reference and a brief details message.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the commands `./app -i <int_arg>`
     * // and `./app --integer <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required<int>('i', "integer", &int_arg, "This is an integer argument.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required(char short_name, char *long_name, T *value, char *details)
    {
        // TODO
        return *this;
    }

    /**
     * @brief Registers the help message for the CLI application.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app --help` or `./app -h`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .help("This is a help message for the CLI application.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    Args &help(char *message)
    {
        optstring += "h:";
        options.push_back(option{"help", optional_argument, nullptr, 'h'});
        return *this;
    }

    /**
     * @brief Registers the help message for the CLI application.
     *
     * # Example
     *
     * ```cpp
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .help("This is a help message for the CLI application.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    void parse(int argc, char *argv[])
    {
        const char *progname = argv[0];

        int opt;
        while ((opt = getopt_long(argc, argv, optstring.c_str(), &options[0], nullptr)) != -1)
        {
            // TODO options
            // TODO help
        }

        // TODO positional
    }
};
