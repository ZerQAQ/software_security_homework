void unsafe_stack(){
    char var[100];
    int qwq[200];
    short owo[50];
    char test[10] = "12345678901234567890";
    memset(qwq, 0, 100);
    memcpy(owo, qwq, 200);
    memcpy(qwq, qwq, 1000);
    memset(test, 0, 200);
}

int unsafe_heap(){
    char *p = malloc(100);
    int var[100];
    memcpy(p, var, 1000);
}

void unsafe_int(){
    short a;
    int b;
    a = b;

    unsigned c;
    int d;
    c = d;
    d = c;
}