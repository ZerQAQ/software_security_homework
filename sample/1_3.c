#include<stdio.h>

// 更改变量名
// 添加冗余语句
// 添加冗余函数

int li1ilil1i(int l1ll1ll, int l1ll1lll1ll1){
    return l1ll1lll1ll1 + l1ll1ll;
}

int iilil1il1i(int n, int l1lll1ll1){
    int liiil1ilii1il = 0;
    for(int i = 0; i < l1lll1ll1; i++){
        liiil1ilii1il = li1ilil1i(liiil1ilii1il, n);
    }
    //冗余代码块1
    int iilillilli = 1;
    for(int j = 0; j < liiil1ilii1il; j += liiil1ilii1il / l1lll1ll1){
        iilillilli += liiil1ilii1il;
    }

    return liiil1ilii1il;
}

//冗余函数

int iilillilli(int illiil, int illlillli){
    return li1ilil1i(illiil, illlillli) + li1ilil1i(illlillli, illiil);
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
    // 冗余代码块2
    for(int i = 0; i < 100; i++){
        ililiii1il = li1ilil1i(i, ililiii1il);
    }
    scanf("%d %d", &ililiii1il, &liiiliiliii);
    printf("%d\n", lili1ilil(ililiii1il, liiiliiliii));
}