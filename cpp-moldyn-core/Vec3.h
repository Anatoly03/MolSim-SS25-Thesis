/**
 * @file Vec3.h
 * @brief Definition of the Vec3 class for three-dimensional vector operations.
 */

#pragma once

/**
 * @brief A struct representing a three-dimensional [mathematical vector](https://en.wikipedia.org/wiki/Vector_%28mathematics_and_physics%29).
 * @param T The type of the vector components, defaulting to double.
 */
template <typename T = double>
struct Vec3
{
public:
    T x;
    T y;
    T z;

    /**
     * @brief Initializes a zero vector.
     */
    Vec3() : x(0), y(0), z(0) {}
};
