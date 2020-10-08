import random

ids = []
def _random_id():
    len = random.randrange(5, 10)
    ret = ""
    for _ in range(len):
        ret += random.choice("qwertyuiopasdfghjklzxcvbnm_QWERTYUIOPASDFGHJKLZXCVBNM")
    return ret

def random_id():
    ret = _random_id()
    while ret in ids:
        ret = _random_id()
    ids.append(ret)
    return ret

def random_type():
    return random.choice(["int", "short", "char", "unsigned"])

def stack_new_array(tp, id):
    return tp + " " + id + "[" + str(random.randrange(50, 1000)) + "];"

def malloc_new_array(tp, id):
    return tp + " *" + id + " = " "malloc(" + str(random.randrange(50, 1000)) + ");"

def memset_array(id, _):
    return "memset(" + id + ", 0, " + str(random.randrange(50, 2000)) + ");"

def memcpy_array(id1, id2):
    return "memcpy(" + id1 + ", " + id2 + ", " + str(random.randrange(50, 2000)) + ");"

def def_single(tp, id):
    return tp + " " + id + ";"

def use_single(id1, id2):
    return id1 + " = " + id2 + ";"

arr_def_func = [malloc_new_array, stack_new_array]
arr_use_func = [memset_array, memcpy_array]

def random_func():
    func_name = random_id()
    ret = "void " + func_name + "(){\n\t"

    _vars = [random_id() for _ in range(random.randrange(3, 10))]
    expr = [random.choice(arr_def_func)(random_type(), var_id) for var_id in _vars]
    for _ in range(6, 20):
        expr.append(random.choice(arr_use_func)(random.choice(_vars), random.choice(_vars)))
    ret += '\n\t'.join(expr) + "\n\t"

    _vars = [random_id() for _ in range(random.randrange(3, 10))]
    expr = [def_single(random_type(), var_id) for var_id in _vars]
    for _ in range(6, 20):
        expr.append(use_single(random.choice(_vars), random.choice(_vars)))
    ret += '\n\t'.join(expr)

    ret +='''
    int {0}[10];
    int *{1} = malloc(10);
    memset({0}, 0, 100);
    memset({1}, 0, 100);
    unsigned {2};
    char {3};
    int {4};
    {2} = {3};
    {3} = {4};
    {4} = {2};
    {2} = {4};
'''.format(random_id(), random_id(), random_id(), random_id(), random_id())
    
    ret +=  "}\n\n"
    
    return ret

def random_sample():
    ret = ""
    while ret.count("\n") < 200:
        ret += random_func()
    return ret

def random_samples(n):
    for i in range(n):
        with open("sample/" + str(i) + ".c", "w") as f:
            f.write(random_sample())


random_samples(50)
