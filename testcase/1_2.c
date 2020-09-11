#include<stdio.h>

// 更改变量名

int li1ilil1i(int l1ll1ll, int l1ll1lll1ll1){
    return l1ll1lll1ll1 + l1ll1ll;
}

int iilil1il1i(int n, int l1lll1ll1){
    int liiil1ilii1il = 0;
    for(int i = 0; i < l1lll1ll1; i++){
        liiil1ilii1il = li1ilil1i(liiil1ilii1il, n);
    }
    return liiil1ilii1il;
}

int lili1ilil(int iililliiii1111, int ililiiili){
    int liiliii1lili = 1;
    for(int i = 0; i < ililiiili; i++){
        liiliii1lili = iilil1il1i(liiliii1lili, iililliiii1111);
    }
    return liiliii1lili;
}

int main(){
    int ililiii1il, liiiliiliii;
    scanf("%d %d", &ililiii1il, &liiiliiliii);
    printf("%d\n", lili1ilil(ililiii1il, liiiliiliii));
}