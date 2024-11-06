#include "file.h"

int mult(int val1, int val2)
{
    auto result = 0;
    for (int i = 0; i < val2; i++)
    {
        result = foo::add(val1, result);
    }
    return result;
}

namespace foo
{

int add(int val1, int val2)
{
    return val1 + val2;
}

} // namespace foo
