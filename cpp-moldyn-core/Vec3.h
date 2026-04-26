/**
 * @file Vec3.h
 * @brief Definition of the Vec3 class for three-dimensional vector operations.
 */

#pragma once

/**
 * @brief A struct representing a three-dimensional [mathematical vector](https://en.wikipedia.org/wiki/Vector_%28mathematics_and_physics%29).
 * @tparam T The type of the vector components, defaulting to double.
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

    /**
     * @brief Initializes a vector with equal components.
     * @param e The value to set for all components of the vector.
     * 
     * # Example
     * 
     * ```cpp
     * Vec3<double> a(1.0); // == Vec3(1.0, 1.0, 1.0)
     * Vec3<double> a(-1.0); // == Vec3(-1.0, -1.0, -1.0)
     * ```
     */
    Vec3(T e) : x(e), y(e), z(e) {}

    /**
     * @brief Initializes a vector with specified components.
     */
    Vec3(T x, T y, T z) : x(x), y(y), z(z) {}

    // remove implicit copy constructors
    // https://stackoverflow.com/questions/33776697/deleting-copy-constructors-and-copy-assignment-operators-which-of-them-are-esse
    Vec3 (const Vec3&) = delete;
    Vec3& operator= (const Vec3&) = delete;
    inline constexpr Vec3 clone() const { return Vec3<T>(x, y, z); }
};
