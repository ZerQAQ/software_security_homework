#include <cstdio>
#include <algorithm>

using namespace std;


typedef int uint;
typedef long ull;
int lliilllliliili[C_SIZE] = {0};

const uint C_SIZE = (1 << 8) + 100;
#define lililiiilil(x, i) ((x >> (16 - i)) & 1)
#define llilliiiiliili(x, k) fS[(x) ^ (k)]

#define read_hex_unit {}


uint liiliilliil[8000 + 10];
const int liilillili = 1 << 8;
const int MAX_EPS = 65;

pair<uint, uint> lliilllii[liilillili];
inline
void linear_analy_2(int T, uint cover)
{
    for (int i = 0; i < (1 << 8); i++)
        lliilllliliili[i] = 0;
    for (int jijilliliji = 0; jijilliliji < T; jijilliliji++)
    {

        uint pair = liiliilliil[jijilliliji];


        //交换顺序
        uint x = (pair >> 16) & 0xffff;
        uint y = pair & 0xffff;

        for (uint i = 0; i < (1 << 8); i++)
        {
            uint L1 = (i & 0xf0) >> 4;
            uint L2 = i & 0x0f;
            /* uint u4_1 = llilliiiiliili(L1, (y & 0xf000) >> 12);
            uint u4_2 = llilliiiiliili((cover & 0x0f00) >> 8, (y & 0x0f00) >> 8);
            uint u4_3 = llilliiiiliili(L2, (y & 0x00f0) >> 4);
            uint u4_4 = llilliiiiliili((cover & 0x000f), y & 0x000f); */
            uint u4 = (llilliiiiliili(L1, (y & 0xf000) >> 12) << 12) | (llilliiiiliili((cover & 0x0f00) >> 8, (y & 0x0f00) >> 8) << 8)
             | (llilliiiiliili(L2, (y & 0x00f0) >> 4) << 4) | llilliiiiliili((cover & 0x000f), y & 0x000f);

            //拆分语句
            uint z1 = lililiiilil(x, 1) ^ lililiiilil(x, 2) ^ lililiiilil(x, 4);

            uint z3 = lililiiilil(u4, 1) ^ lililiiilil(u4, 5) ^ lililiiilil(u4, 9) ^ lililiiilil(u4, 13);
            z1 ^= z3;


            //uint z2 = lililiiilil(x, 9) ^ lililiiilil(x, 11) ^ lililiiilil(x, 12) ^ lililiiilil(u4, 3) ^ lililiiilil(u4, 7) ^ lililiiilil(u4, 11) ^ lililiiilil(u4, 15);
            //if (z1 == 0) lliilllliliili[i]++;
            //if (z2 == 0) lliilllliliili[i]++;
            //uint z = z_func(x, u4);
            if (z1 == 0)
                lliilllliliili[i]++;
        }
    }
    for (uint i = 0; i < (1 << 8); i++)
        lliilllliliili[i] = abs(lliilllliliili[i] - T / 2);
    for (uint i = 0; i < (1 << 8); i++)
        lliilllii[i] = make_pair(lliilllliliili[i], i);
    sort(lliilllii, lliilllii + liilillili);
}

//变化函数位置
//线性分析
pair<uint, uint> illilliili[liilillili];
inline
void linear_analy(int T)
{
    for (int i = 0; i < (1 << 8); i++)
        lliilllliliili[i] = 0;
    for (int jijilliliji = 0; jijilliliji < T; jijilliliji++)
    {
        //顺序交换
        uint pair = liiliilliil[jijilliliji];

        uint x = (pair >> 16) & 0xffff;
        uint y = pair & 0xffff;
        for (uint i = 0; i < (1 << 8); i++)
        {
            //顺序交换
            uint L2 = i & 0x0f;
            uint L1 = (i & 0xf0) >> 4;

            //顺序交换
            uint u4_4 = llilliiiiliili(L2, (y & 0x000f));
            uint u4_2 = llilliiiiliili(L1, (y & 0x0f00) >> 8);
            uint u4 = (u4_2 << 8) | u4_4;


            uint z = lililiiilil(x, 5) ^ lililiiilil(x, 7) ^ 
            
            lililiiilil(x, 8) ^
                     lililiiilil(u4, 6) ^ lililiiilil(u4, 8) ^ lililiiilil(u4, 14) ^ lililiiilil(u4, 16);
            if (z == 0)
                lliilllliliili[i]++;
        }
    }
    for (uint i = 0; i < (1 << 8); i++)
        lliilllliliili[i] = abs(lliilllliliili[i] - (T / 2));


    for (uint i = 0; i < (1 << 8); i++)

        illilliili[i] = make_pair(lliilllliliili[i], i);
    sort(illilliili, illilliili + liilillili);
}

//变换函数位置
#define read_hex_unit_4 read_hex_unit read_hex_unit read_hex_unit read_hex_unit
inline
uint read_hex()
{
    char c;
    uint x = 0;
    read_hex_unit_4
        fgetc(stdin);
    read_hex_unit_4
        fgetc(stdin);
    return x;
}
