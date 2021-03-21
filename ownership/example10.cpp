#include <vector>
#include <iostream>

using namespace std;

int main()
{
    vector<int> v;
    int *p;
    v.push_back(1);
    p = &v[0];
    *p = 100;
    cout << v[0] << endl;
} 
