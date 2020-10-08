#include<stdio.h>

int add(int a, int b){
    return a + b;
}

int mul(int n, int m){
    int total = 0;
    for(int i = 0; i < m; i++){
        total = add(total, n);
    }
    return total;
}

int power(int x, int y){
    int total = 1;
    for(int i = 0; i < y; i++){
        total = mul(total, x);
    }
    return total;
}

int main(){
    int n, m;
    scanf("%d %d", &n, &m);
    printf("%d\n", power(n, m));
}