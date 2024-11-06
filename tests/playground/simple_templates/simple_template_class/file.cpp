template <class T> class TemplateClass
{
  public:
    static int foo()
    {
        return T::ba();
    }
};

class SimpleClass
{
  public:
    static int ba()
    {
        return 5;
    }
};

int main(int argc, char *argv[])
{
    TemplateClass<SimpleClass> instance;
    return instance.foo();
}
