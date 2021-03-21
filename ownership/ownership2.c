#include <stdlib.h>
int main() 
{
    int *v = malloc(4*sizeof(int));
    v[0] = 10, v[1] = 20; 
    v[2] = 30, v[3] = 40;
}
