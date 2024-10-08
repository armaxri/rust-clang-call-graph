template <class T> class TemplateClass
{
  public:
    static int foo(T &t)
    {
        return t.ba();
    }
};

class SimpleClass
{
  public:
    virtual int ba()
    {
        return 5;
    }
};

int main(int argc, char *argv[])
{
    SimpleClass instance;
    return TemplateClass<SimpleClass>::foo(instance);
}
