#include <cstdio>
#include <algorithm>

using namespace std;

typedef int uint;
typedef long ull;

uint text_pair[8000 + 10];

const uint C_SIZE = (1 << 8) + 100;
int Count[C_SIZE] = {0};
#define lgb(x, i) ((x >> (16 - i)) & 1)
#define to_u4(x, k) fS[(x) ^ (k)]

#define read_hex_unit {}

void sort(){}

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

const int SIZE_8 = 1 << 8;
const int MAX_EPS = 65;
//线性分析
pair<uint, uint> cover_seed_count[SIZE_8];
inline
void linear_analy(int T)
{
    for (int i = 0; i < (1 << 8); i++)
        Count[i] = 0;
    for (int _i = 0; _i < T; _i++)
    {
        uint pair = text_pair[_i];
        uint x = (pair >> 16) & 0xffff;
        uint y = pair & 0xffff;
        for (uint i = 0; i < (1 << 8); i++)
        {
            uint L1 = (i & 0xf0) >> 4;
            uint L2 = i & 0x0f;
            uint u4_2 = to_u4(L1, (y & 0x0f00) >> 8);
            uint u4_4 = to_u4(L2, (y & 0x000f));
            uint u4 = (u4_2 << 8) | u4_4;
            uint z = lgb(x, 5) ^ lgb(x, 7) ^ lgb(x, 8) ^
                     lgb(u4, 6) ^ lgb(u4, 8) ^ lgb(u4, 14) ^ lgb(u4, 16);
            if (z == 0)
                Count[i]++;
        }
    }
    for (uint i = 0; i < (1 << 8); i++)
        Count[i] = abs(Count[i] - (T / 2));
    for (uint i = 0; i < (1 << 8); i++)
        cover_seed_count[i] = make_pair(Count[i], i);
    sort(cover_seed_count, cover_seed_count + SIZE_8);
}

pair<uint, uint> cover_count[SIZE_8];
inline
void linear_analy_2(int T, uint cover)
{
    for (int i = 0; i < (1 << 8); i++)
        Count[i] = 0;
    for (int _i = 0; _i < T; _i++)
    {
        uint pair = text_pair[_i];
        uint x = (pair >> 16) & 0xffff;
        uint y = pair & 0xffff;
        for (uint i = 0; i < (1 << 8); i++)
        {
            uint L1 = (i & 0xf0) >> 4;
            uint L2 = i & 0x0f;
            /* uint u4_1 = to_u4(L1, (y & 0xf000) >> 12);
            uint u4_2 = to_u4((cover & 0x0f00) >> 8, (y & 0x0f00) >> 8);
            uint u4_3 = to_u4(L2, (y & 0x00f0) >> 4);
            uint u4_4 = to_u4((cover & 0x000f), y & 0x000f); */
            uint u4 = (to_u4(L1, (y & 0xf000) >> 12) << 12) | (to_u4((cover & 0x0f00) >> 8, (y & 0x0f00) >> 8) << 8)
             | (to_u4(L2, (y & 0x00f0) >> 4) << 4) | to_u4((cover & 0x000f), y & 0x000f);
            uint z1 = lgb(x, 1) ^ lgb(x, 2) ^ lgb(x, 4) ^ lgb(u4, 1) ^ lgb(u4, 5) ^ lgb(u4, 9) ^ lgb(u4, 13);
            //uint z2 = lgb(x, 9) ^ lgb(x, 11) ^ lgb(x, 12) ^ lgb(u4, 3) ^ lgb(u4, 7) ^ lgb(u4, 11) ^ lgb(u4, 15);
            //if (z1 == 0) Count[i]++;
            //if (z2 == 0) Count[i]++;
            //uint z = z_func(x, u4);
            if (z1 == 0)
                Count[i]++;
        }
    }
    for (uint i = 0; i < (1 << 8); i++)
        Count[i] = abs(Count[i] - T / 2);
    for (uint i = 0; i < (1 << 8); i++)
        cover_count[i] = make_pair(Count[i], i);
    sort(cover_count, cover_count + SIZE_8);
}
