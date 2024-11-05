class TestGrandParentClass
{
  public:
    virtual int add(int val1, int val2)
    {
        return val1 + val2;
    }
};

class TestParentClass : public TestGrandParentClass
{
  public:
    int add(int val1, int val2) override
    {
        return TestGrandParentClass::add(val1, val2);
    }
};

class TestClass : public TestParentClass
{
  public:
    int add(int val1, int val2) override
    {
        return TestParentClass::add(val1, val2);
    }
};

int main(int argc, char *argv[])
{
    TestClass testClass;
    return testClass.add(1, 2);
}
