#include <stdlib.h>
int main() 
{
    int *v = malloc(4*sizeof(int));
    v[0] = 1, v[1]  = 2; 
    v[2] = 3, v[3] = 4;
}
