#include <iostream>
#include <cstdio>
#include <conio.h>
#include <ctime>
#include <windows.h>
#include <vector>
#include <string>
#include <cstring>
#include <cstdlib>
#include <algorithm>
#include "vbkaddlst.h"
using namespace std;

int main()
{

	//test_trie();
	//test_contact();
	UI Menu;

	Menu.render_menu();
	Menu.print();
	//Menu.system_debug();
	//	return 0;
	while (true)
	{
		if (!Menu.V_SYNC_ON())
		{
			if (_kbhit())
			{
				Menu.operation();
			}
			Sleep(CLOCKS_PER_SEC / Menu.refresh_rate_val());
		}
		else
		{
			Menu.operation();
		}
		system("cls");
		Menu.print();
		if (Menu.developer_mode())
			Menu.system_debug();
	}
	return 0;
}

/*
20

Isaac 15191480469
Sherard 13542238276
Elmer 13960846270
Ivan 13559788607
Lamont 13935480283
Earthy 13967658384
Ezra 13175358007
Emerson 13583272499
Falkner 13911287370
Dexter 13772658569
Agnes 15161418699
Maxine 13976100181
Blackbird 13155069476
Peace 13989215798
Olive 13796500707
Graceful 13787496664
Imagine 13195251197
Renata 13592885188
Warlike 13928433629
Whitney 13590807791


*/