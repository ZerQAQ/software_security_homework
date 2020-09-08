typedef unsigned int uint;

const uint S[] = {
    0xE, 4, 0xD, 1, 2, 0xF, 0xB, 8, 3, 0xA, 6, 0xC, 5, 9, 0, 7};
const uint fS[] = {
    14, 3, 4, 8, 1, 12, 10, 15, 7, 13, 9, 6, 11, 2, 0, 5};
const uint P[] = {
    0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15};
const uint fP[] = {
    0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15};

void prtBinary(uint text)
{
    for (uint i = 12; i >= 0; i -= 4)
    {
        cout << bitset<4>(text >> i) << " ";
    }
    cout << endl;
}

uint Sbox(uint text, const uint *sbox = S)
{
    uint ret = 0;
    for (uint i = 0; i < 16; i += 4)
    {
        uint origin = text & (0xf << i);
        origin >>= i;
        ret |= sbox[origin] << i;
    }
    return ret;
}

uint Pbox(uint text, const uint *pbox = P)
{
    bool origin[16], after[16];
    for (uint i = 0; i < 16; i++)
    {
        origin[i] = (text >> i) & 1;
    }
    for (uint i = 0; i < 16; i++)
    {
        after[pbox[i]] = origin[i];
    }
    uint ret = 0;
    for (uint i = 0; i < 16; i++)
    {
        ret |= after[i] << i;
    }
    return ret;
}

// 使用第i轮密钥
uint Kbox(uint text, uint k, uint i)
{
    i--;
    uint Ki = 0x3a94d63f & (0xffff0000 >> (i * 4));
    Ki >>= (4 - i) * 4;
    //pruintf("K%d = ", i + 1);
    //prtBinary(Ki);
    return text ^ Ki;
}

uint SPN_encryp(uint x, uint k)
{
    for (uint i = 1; i <= 4; i++)
    {
        x = Kbox(x, k, i);
        //pruintf("u%d = ", i);
        //prtBinary(x);
        x = Sbox(x);
        //pruintf("v%d = ", i);
        //prtBinary(x);
        if (i != 4)
        {
            x = Pbox(x);
            //pruintf("w%d = ", i);
            //prtBinary(x);
        }
        else
        {
            x = Kbox(x, k, i + 1);
        }
    }
    return x;
}

uint SPN_decrypt(uint x, uint k)
{
    for (uint i = 4; i >= 1; i--)
    {
        if (i != 4)
        {
            x = Pbox(x, fP);
        }
        else
        {
            x = Kbox(x, k, i + 1);
        }
        x = Sbox(x, fS);
        x = Kbox(x, k, i);
    }
    return x;
}

int main()
{
    uint n;
    scanf("%d", &n);
    while (n--)
    {
        uint k, x;
        scanf("%x %x", &k, &x);
        uint v1 = SPN_encryp(x, k);
        uint v2 = SPN_decrypt(v1 ^ 1, k);
        printf("%04x %04x\n", v1, v2);
    }
    return 0;
}