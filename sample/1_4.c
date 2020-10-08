#include<stdio.h>

// 更改变量名
// 使用typedef

typedef int myint;
typedef int yourint;

myint B_FUNC(yourint QQQQ, myint FA){
    return FA + QQQQ;
}

myint A_FUNC(yourint n, myint MMM){
    myint A_VAR = 0;
    for(myint i = 0; i < MMM; i++){
        A_VAR = B_FUNC(A_VAR, n);
    }
    return A_VAR;
}

myint FUNC(yourint val, myint y){
    myint RET = 1;
    for(yourint i = 0; i < y; i++){
        RET = A_FUNC(RET, val);
    }
    return RET;
}

myint main(){
    yourint input_a, input_b;
    scanf("%d %d", &input_a, &input_b);
    printf("%d\n", FUNC(input_a, input_b));
}