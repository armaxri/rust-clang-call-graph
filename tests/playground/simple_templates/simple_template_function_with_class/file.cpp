template <class T> int foo()
{
    return T::ba();
}

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
    return foo<SimpleClass>();
}
