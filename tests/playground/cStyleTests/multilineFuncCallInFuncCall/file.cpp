int add(int val1, int val2)
{
    return val1 + val2;
}

int main(int argc, char *argv[])
{
    // clang-format off
    return add(
        add(1, 1),
        1);
    // clang-format on
}
