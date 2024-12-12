namespace foo::bar
{
int add(int val1, int val2)
{
    return val1 + val2;
}
}

int main(int argc, char *argv[])
{
    return foo::bar::add(foo::bar::add(1, 1), 1);
}
