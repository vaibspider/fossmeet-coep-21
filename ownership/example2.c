int main()
{
    int j = 1;
    int a[4];
    int i = 2;

    a[4] = 5; // bug
    a[5] = 6; // bug
    a[10000] = 7; // bug
}
