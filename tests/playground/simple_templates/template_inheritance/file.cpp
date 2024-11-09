template <class T>
class ParentTemplateClass
{
public:
  static int fooParent()
  {
    return T::ba();
  }
};

template <class T>
class ChildTemplateClass : public ParentTemplateClass<T>
{
public:
  static int foo()
  {
    return ParentTemplateClass<T>::fooParent();
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
    ChildTemplateClass<SimpleClass> instance;
    return instance.foo();
}
