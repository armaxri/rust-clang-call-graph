class TestClass
{
  public:
    static int add(int val1, int val2)
    {
        return val1 + val2;
    }
};

int main(int argc, char *argv[])
{
    return []() { return TestClass::add(1, 2); }();
}
