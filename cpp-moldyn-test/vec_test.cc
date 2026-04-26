#include <gtest/gtest.h>
#include "Vec3.h"

/**
 * @brief The default vector is the vector containing all zeroes.
 */
TEST(VecZeroTest, BasicAssertions) {
    Vec3<double> a = Vec3();

    // Test Zero
    EXPECT_EQ(a.x, 0.0);
    EXPECT_EQ(a.y, 0.0);
    EXPECT_EQ(a.z, 0.0);
}
