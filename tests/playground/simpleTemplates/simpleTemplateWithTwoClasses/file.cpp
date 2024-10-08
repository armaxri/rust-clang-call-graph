template <class T> class TemplateClass
{
  public:
    static int foo()
    {
        return T::ba();
    }
};

class SimpleClassA
{
  public:
    static int ba()
    {
        return 5;
    }
};

class SimpleClassB
{
  public:
    static int ba()
    {
        return 6;
    }
};

int main(int argc, char *argv[])
{
    TemplateClass<SimpleClassA> instance1;
    TemplateClass<SimpleClassB> instance2;
    return instance1.foo() + instance2.foo();
}
