#include <stdio.h>
#include <strings.h>
int main()
{
    char s[100] = "hello";
    char *p;

    p = index(s, 'f');
    *p = 'a'; // bug!

    printf("%s\n", s);

    return 0;
}
