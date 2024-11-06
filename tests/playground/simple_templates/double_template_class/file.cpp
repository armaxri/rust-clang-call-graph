template <class T, class K> class TemplateClass
{
  public:
    static int foo()
    {
        return T::ba() + K::ba();
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
        return 5;
    }
};

int main(int argc, char *argv[])
{
    TemplateClass<SimpleClassA, SimpleClassB> instance;
    return instance.foo();
}
