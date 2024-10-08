class TestClass
{
  public:
    virtual int add(int val1, int val2)
    {
        return val1 + val2;
    }
    virtual int sub(int val1, int val2);
};

int main(int argc, char *argv[])
{
    TestClass test;
    return test.add(1, 2) + test.sub(1, 2);
}
