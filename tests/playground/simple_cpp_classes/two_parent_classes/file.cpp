class TestBaseClass1
{
  public:
    virtual int add(int val1, int val2)
    {
        return val1 + val2;
    }
};

class TestBaseClass2
{
  public:
    virtual int sub(int val1, int val2)
    {
        return val1 + val2;
    }
};

class TestClass : public TestBaseClass1, public TestBaseClass2
{
  public:
    int add(int val1, int val2) override
    {
        return TestBaseClass1::add(val1, val2);
    }

    int sub(int val1, int val2) override
    {
        return TestBaseClass2::sub(val1, val2);
    }
};

int main(int argc, char *argv[])
{
    TestClass testClass;
    return testClass.add(1, 2) + testClass.sub(1, 2);
}
