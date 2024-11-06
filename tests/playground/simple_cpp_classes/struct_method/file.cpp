struct TestStruct
{
    int add(int val1, int val2)
    {
        return val1 + val2;
    }
};

int main(int argc, char *argv[])
{
    TestStruct test;
    return test.add(1, 2);
}
