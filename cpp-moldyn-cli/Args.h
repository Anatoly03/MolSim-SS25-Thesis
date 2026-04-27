/**
 * @file Args.h
 * @author Anatoly Weinstein
 * @brief Definition of the Args class for CLI argument parsing,
 * wrapping around getopt_long for parsing.
 */

#pragma once

#include <getopt.h>
#include <iostream>
#include <map>
#include <optional>
#include <set>
#include <string>
#include <typeinfo>
#include <vector>

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
    /**
     * Helper construct to store reference to registered arguments along their
     * type information for parsing and error handling.
     */
    struct ArgsRef
    {
    public:
        const size_t type_hint;
        const std::string type_name;

    private:
        void *value;
        // @brief Did the parser ask for this argument to be read?
        bool is_set = false;

    public:
        ArgsRef(size_t type_hint, std::string type_name, void *value)
            : type_hint(type_hint), type_name(type_name), value(value) {}
        // type_hint(typeid(P).hash_code()), type_name(typeid(P).name()), value(value)

        template <typename P>
        static ArgsRef with(P *value)
        {
            return ArgsRef(typeid(P).hash_code(), typeid(P).name(), value);
        }

        template <typename P>
        P *try_get() const
        {
            if (typeid(P).hash_code() == type_hint)
            {
                return static_cast<P *>(value);
            }

            return nullptr;
        }
    };

    /**
     * @brief The string of short options for getopt_long. The string format is
     * a sequence of characters, followed by a colon if the option requires an
     * argument, or two colons if the option takes an optional argument.
     *
     * @see [getopt](https://www.man7.org/linux/man-pages/man3/getopt.3.html)
     */
    std::string optstring;

    /**
     * @brief The vector of long options for command parsing.
     *
     * @see [getopt](https://www.man7.org/linux/man-pages/man3/getopt.3.html)
     */
    std::vector<option> options;

    /**
     * @brief An ordered map of option arguments mapped to their outer references.
     */
    std::map<char, ArgsRef> references;

    /**
     * @brief An vector of positional argument mapped to their outer references.
     */
    std::vector<ArgsRef> positional_references;

    // @brief Amount of, registered as required, positional arguments.
    int required_positional_arguments = 0;

    // @brief Internal help argument tag: `--help` or `-h`.
    std::optional<std::string> help_flag = std::nullopt;
    // @brief Internal help argument tag: `--version` or `-v`.
    std::optional<std::string> version_flag = std::nullopt;

    /**
     * @brief Prints the help message and exits the application.
     */
    [[noreturn]]
    void print_help()
    {
        std::cout << "Help message for the CLI application.\n";
        exit(0);
    }

    /**
     * @brief Parse single argument with the appropriate type and store in outer
     * reference.
     */
    bool parse_into_ref(char *optarg, const ArgsRef &ref)
    {
        // booleans are flags and treated as true if present
        // TODO support for `-B0` syntax for setting booleans to false
        if (const auto v = ref.try_get<bool>())
        {
            *v = true;
            return true;
        }

        if (const auto v = ref.try_get<double>())
        {
            *v = atof(optarg);
            return true;
        }

        if (const auto v = ref.try_get<int>())
        {
            *v = atoi(optarg);
            return true;
        }

        if (const auto v = ref.try_get<std::string>())
        {
            *v = std::string(optarg);
            return true;
        }

        if (const auto v = ref.try_get<std::optional<std::string>>())
        {
            *v = std::optional<std::string>(optarg ? optarg : "");
            return true;
        }

        return false;
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
        required_positional_arguments += 1;
        positional_references.push_back(ArgsRef::with<T>(value));
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
        // https://stackoverflow.com/questions/1472048/how-to-append-a-char-to-a-stdstring
        optstring += short_name;
        optstring += ':';

        options.push_back(option{long_name, required_argument, nullptr, short_name});
        references.insert({short_name, ArgsRef::with<T>(value)});
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
    Args &help(const char *message)
    {
        optstring.append("h::");
        options.push_back(option{"help", optional_argument, nullptr, 'h'});
        references.insert({'h', ArgsRef::with<std::optional<std::string>>(&help_flag)});
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
            for (const auto &option : options)
            {
                if (option.val != opt)
                    continue;

                // parse argument into appropriate type and store in outer reference
                const auto ref = references.find(opt)->second;

                if (parse_into_ref(optarg, ref))
                    ;
                break;

                // We land here if the option type could not be parsed.
                std::cerr << "Error: Failed to parse argument for option -" << (char)opt << "\n";
                std::cerr << "Expected type: " << ref.type_name << "\n";
                std::cerr << "Received value: `" << (optarg ? optarg : "null") << "`\n";
                exit(1);
            }
        }

        // if help flag was set, print the help message and exit
        if (help_flag.has_value())
        {
            print_help();
        }

        // TODO version flag

        // parse positional arguments
        for (int i = 0; i < required_positional_arguments && optind < argc; i++, optind++)
        {
            // parse positional argument into appropriate type and store in outer reference
            const auto ref = positional_references[i];

            if (parse_into_ref(argv[optind], ref))
                ;
            break;

            // We land here if the option type could not be parsed.
            std::cerr << "Error: Failed to parse argument at position " << i + 1 << "\n";
            std::cerr << "Expected type: " << ref.type_name << "\n";
            std::cerr << "Received value: `" << (argv[optind] ? argv[optind] : "null") << "`\n";
            exit(1);
        }
    }
};
