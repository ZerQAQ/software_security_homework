#include<stdio.h>

// 更改变量名
// 增加冗余空白符

int 

B_FUNC(int QQQQ,        int FA){
    return      FA 
    + QQQQ;
}

int A_FUNC(int          n,

 int        MMM){
    int A_VAR       = 0;
    for(int i           = 0; i < 
    
    
    MMM; i++){
        A_VAR =         B_FUNC(A_VAR, 
        n);
    }
    return A_VAR;
}

int FUNC(               int val,
 int y){
    int RET      = 1;
    for(int 
    i = 0;
     i <                y; i++){
        RET = 
        A_FUNC(     RET, val);
    }
    return 
    RET;
}

int 
main        (){
    int input_a,        
    
    input_b;
    scanf("%d %d", 
    
            &input_a,        &input_b);
    printf("%d\n",      
    FUNC(input_a,       input_b));
}