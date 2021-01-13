#include <iostream>
#include <cstdio>
#include <conio.h>
#include <ctime>
#include <windows.h>
#include <vector>
#include <string>
#include <cstring>
#include <cstdlib>
#include <sstream>
#include <algorithm>
#include "vbkaddlst.h"
using namespace std;

item::item(list *son_, vector<string> val_key_, string name_, string explain_)
{
	if (name_ == "Resolution")
	{
		int tmp = 0;
	}
	_son_ = son_;
	_val_ = (val_key_.size() > 0) ? 0 : -1;
	_val_key_ = val_key_;
	_name_ = name_;
	_explain_ = explain_;
}
int item::val()
{
	return _val_;
}
void item::val(int val_)
{
	if (val_ < 0)
		_val_ = _val_key_.size() - 1;
	else if (val_ >= _val_key_.size())
	{
		if(_val_key_.size() == 0) return;
		_val_ = 0;
	}
	else
		_val_ = val_;
}
list *item::son()
{
	return _son_;
}
void item::son(list *son_)
{
	_son_ = son_;
}
string item::val_key()
{
	return _val_key_[_val_];
}
string item::val_key(int val_)
{
	if (val_ >= _val_key_.size() || val_ < 0)
		return "NULL";
	return _val_key_[val_];
}
vector<string> item::all_val_key()
{
	return _val_key_;
}
void item::val_key(string val_key_, int val_)
{
	if (val_ < _val_key_.size() && val_ >= 0)
		_val_key_[val_] = val_key_;
}
void item::val_key(vector<string> val_key_)
{
	_val_key_ = val_key_;
}
string item::name()
{
	return _name_;
}
void item::name(string name_)
{
	_name_ = name_;
}
string item::explain()
{
	return _explain_;
}
void item::explain(string explain_)
{
	_explain_ = explain_;
}

list::list(bool const_, list *fa_, vector<item *> item_, string name_)
{
	_const_ = const_;
	_fa_ = fa_;
	_item_ = item_;
	_name_ = name_;
}
list *list::fa()
{
	return _fa_;
}
void list::fa(list *fa_)
{
	_fa_ = fa_;
}
void list::add_item(item *item_)
{
	_item_.push_back(item_);
}
item *list::list_item(int val_)
{
	if (val_ >= 0 && val_ < _item_.size())
		return _item_[val_];
	else
		return NULL;
}
vector<item *> list::list_item()
{
	return _item_;
}
string list::name()
{
	return _name_;
}
void list::name(string name_)
{
	_name_ = name_;
}
void list::delete_element()
{
	if (_const_)
		return;
	for (int i = 0; i < _item_.size(); i++)
	{
		delete _item_[i];
	}
}
void UI::resolution()
{
	int val_ = _resolution_->val();
	if (val_ == 0)
	{
		_width_ = 40;
		_height_ = 10;
	}
	else if (val_ == 1)
	{
		_width_ = 60;
		_height_ = 15;
	}
	else if (val_ == 2)
	{
		_width_ = 80;
		_height_ = 20;
	}
	else if (val_ == 3)
	{
		_width_ = 100;
		_height_ = 25;
	}
	list_range();
}

void UI::refresh_rate()
{
	int val_ = _refresh_rate_->val();
	if (val_ == 0)
	{
		_refresh_rate_val_ = 10;
	}
	else if (val_ == 1)
	{
		_refresh_rate_val_ = 20;
	}
	else if (val_ == 2)
	{
		_refresh_rate_val_ = 30;
	}
}

int UI::refresh_rate_val()
{
	return _refresh_rate_val_;
}

void UI::V_SYNC()
{
	_V_SYNC_ON_ = _V_SYNC_->val();
}

bool UI::V_SYNC_ON()
{
	return _V_SYNC_ON_;
}
bool UI::developer_mode()
{
	return _developer_mode_->val();
}

void UI::list_range()
{
	_lower_bound_ = _height_ + _upper_bound_ - _margin_up_ - _margin_down_ + 1;
}

void UI::init_map()
{
	for (int i = 1; i <= _height_; i++)
	{
		for (int j = 1; j <= _width_; j++)
		{
			_map_[i][j] = ' ';
		}
	}
	for (int i = 1; i <= _width_; i++)
	{
		_map_[1][i] = _map_[_height_][i] = '-';
	}
	for (int i = 1; i <= _height_; i++)
	{
		_map_[i][1] = _map_[i][_width_] = '|';
	}
	_map_[1][1] = _map_[1][_width_] = _map_[_height_][1] = _map_[_height_][_width_] = '+';
}

void UI::map_str_cpy(char *a, int s1, int len, const char *b, int s2)
{
	for (int i = 0; i < len; i++)
	{
		a[s1 + i] = b[s2 + i];
	}
}

UI::UI()
{

	_upper_bound_ = 0;
	list_range();
	_height_ = 10;
	_width_ = 40;
	_cursor_loc_ = 0;
	_margin_up_ = 3;
	_margin_down_ = 2;
	_margin_left_ = 2;
	_margin_right_ = 2;

	_now_item_loc_ = 0;

	_new_address_list_ = NULL;
	_load_address_list_ = NULL;
	_now_address_list_ = NULL;
	_num_address_list_ = 0;
	_address_list_tmp_ = NULL;
	_address_list_list_tmp_ = NULL;
	_show_address_list_ = NULL;
	_add_contact_ = NULL;
	_modify_contact_ = NULL;
	_rename_address_list_ = NULL;
	_delete_address_list_ = NULL;

	init_map();

	vector<item *> list_item; //存储列表元素的临时变量
	item *item_info;		  //存储条目信息的临时变量
	list *list_info;		  //存储列表的临时变量
	vector<string> val_key;	  //存储条目键值的临时变量
	val_key.clear();
	string explain = "Defaut"; //存储条目注释的临时变量

	item_info = new item(NULL, val_key, "New Address List", explain);
	list_item.push_back(item_info);
	_now_item_ = item_info;
	_new_address_list_ = item_info;
	item_info = new item(NULL, val_key, "Load Address List", explain);
	list_item.push_back(item_info);
	_load_address_list_ = item_info;
	item_info = new item(NULL, val_key, "Settings", explain);
	list_item.push_back(item_info);

	item_info = new item(NULL, val_key, "About", explain);
	list_item.push_back(item_info);

	_root_ = new list(true, NULL, list_item, "Main Menu");
	_now_list_ = _root_;

	list_item.clear();
	val_key.clear();
	val_key.push_back("40x10");
	val_key.push_back("60x15");
	val_key.push_back("80x20");
	val_key.push_back("100x25");
	item_info = new item(NULL, val_key, "Resolution", explain);
	list_item.push_back(item_info);
	_resolution_ = item_info;

	val_key.clear();
	val_key.push_back("15Hz");
	val_key.push_back("30Hz");
	val_key.push_back("60Hz");
	val_key.push_back("120Hz");
	val_key.push_back("144Hz");
	val_key.push_back("240Hz");
	val_key.push_back("300Hz");
	item_info = new item(NULL, val_key, "Refresh Rate", explain);
	list_item.push_back(item_info);
	_refresh_rate_ = item_info;

	val_key.clear();
	val_key.push_back("OFF");
	val_key.push_back("ON");
	item_info = new item(NULL, val_key, "V-SYNC", explain);
	list_item.push_back(item_info);
	item_info->val(1);
	_V_SYNC_ = item_info;

	val_key.clear();
	val_key.push_back("English");
	val_key.push_back("Chinese Simplified");
	val_key.push_back("Chinese Traditional");
	item_info = new item(NULL, val_key, "Language", explain);
	list_item.push_back(item_info);

	val_key.clear();
	val_key.push_back("OFF");
	val_key.push_back("ON");
	item_info = new item(NULL, val_key, "Developer Mode", explain);
	list_item.push_back(item_info);
	_developer_mode_ = item_info;

	list_info = new list(true, _root_, list_item, "Settings");
	_root_->list_item(2)->son(list_info);

	list_item.clear();
	val_key.clear();
	item_info = new item(NULL, val_key, "Show Address List", explain);
	list_item.push_back(item_info);
	_show_address_list_ = item_info;
	item_info = new item(NULL, val_key, "Add Contact", explain);
	list_item.push_back(item_info);
	_add_contact_ = item_info;
	item_info = new item(NULL, val_key, "Rename Address List", explain);
	list_item.push_back(item_info);
	_rename_address_list_ = item_info;
	item_info = new item(NULL, val_key, "Delete Address List", explain);
	list_item.push_back(item_info);
	_delete_address_list_ = item_info;
	_contact_op_ = new list(true, NULL, list_item, "address_list_op");

	list_item.clear();
	val_key.clear();
	item_info = new item(NULL, val_key, "Delete Contact", explain);
	list_item.push_back(item_info);
	_delete_contact_ = item_info;
	item_info = new item(NULL, val_key, "Modify Contact", explain);
	list_item.push_back(item_info);
	_modify_contact_ = item_info;
	_contact_item_op_ = new list(true, NULL, list_item, "contact_op");

	resolution();
	V_SYNC();
	refresh_rate();
	developer_mode();

	fscan_address_list();
}

void UI::render_menu()
{
	init_map();
	int x_ = _margin_up_, y_ = _margin_left_;
	string list_name = _now_list_->name();
	map_str_cpy(_map_[x_ - 1], y_ + 1, list_name.size(), list_name.data(), 0);
	vector<item *> list_item = _now_list_->list_item();
	int bottom_edge = min(_lower_bound_, (int)list_item.size() - 1);
	for (int i = _upper_bound_; i <= bottom_edge; i++)
	{
		int now_loc = i - _upper_bound_;
		item *now_item = list_item[i];
		char *a = _map_[x_ + now_loc];
		//string bb = now_item -> val_key();
		string b = now_item->name();
		int s1 = y_ + 1, len = now_item->name().size(), s2 = 0;
		map_str_cpy(a, y_ + 1, len, b.data(), s2);
		if (now_item->val() != -1)
		{
			string val_key = now_item->val_key();
			b = val_key;
			len = val_key.size();
			s2 = 0;
			map_str_cpy(a, _width_ / 2 + 1, len, b.data(), s2);
		}
		if (_cursor_loc_ == i - _upper_bound_)
			_map_[x_ + now_loc][y_] = _map_[x_ + now_loc][_width_ - y_ + 1] = '+';
	}
}

void UI::print()
{
	for (int i = 1; i <= _height_; i++)
	{
		for (int j = 1; j <= _width_; j++)
		{
			printf("%c", _map_[i][j]);
		}
		printf("\n");
	}
}

void UI::fscan_address_list()
{
	string save_ = "info.txt";
	FILE *fp;
	fp = fopen(save_.data(), "r");
	if (fp == NULL)
		return;
	int num_address_list;
	if (fscanf(fp, "%d", &num_address_list) != 1)
		return;
	char ch[2];
	contact *now_address_list_tmp;
	for (int i = 0; i < num_address_list; i++)
	{
		string name_;
		name_.clear();
		fscanf(fp, "%1s", ch);
		while (ch[0] != '\r' && ch[0] != '\n')
		{
			name_.append(1, ch[0]);
			fscanf(fp, "%1c", ch);
		}
		int num_info;
		now_address_list_tmp = create_address_list();
		_address_list_.push_back(now_address_list_tmp);
		now_address_list_tmp->name(name_);
		fscanf(fp, "%d", &num_info);
		for (int j = 0; j < num_info; j++)
		{
			string name_input, phone_number_input;
			name_input.clear();
			phone_number_input.clear();
			fscanf(fp, "%1s", ch);
			while (ch[0] != ' ')
			{
				name_input.append(1, ch[0]);
				fscanf(fp, "%c", ch);
			}
			fscanf(fp, "%1s", ch);
			while (ch[0] != '\n' && ch[0] != '\r')
			{
				phone_number_input.append(1, ch[0]);
				fscanf(fp, "%c", ch);
			}
			vector<string> lable;
			lable.push_back("name");
			lable.push_back("telephone_number");
			vector<string> info;
			info.push_back(name_input);
			info.push_back(phone_number_input);
			now_address_list_tmp->insert(info, lable);
		}
	}
	fclose(fp);
	int x;
	x = 0;
}

void UI::fprint_address_list()
{
	string save_ = "info.txt";
	FILE *fp;
	fp = fopen(save_.data(), "w+");
	if (fp == NULL)
		return;
	fprintf(fp, "%d\n\n", _address_list_.size());
	contact *address_list_tmp;
	vector<data> now_address_list;
	vector<string> info_;
	for (int i = 0; i < _address_list_.size(); i++)
	{
		address_list_tmp = _address_list_[i];
		now_address_list.clear();
		now_address_list = address_list_tmp->show(0);
		string name_ = address_list_tmp->name();
		fprintf(fp, "%s\n%d\n", name_.data(), now_address_list.size());
		for (int j = 0; j < now_address_list.size(); j++)
		{
			info_ = now_address_list[j].info();
			fprintf(fp, "%s %s\n", info_[0].data(), info_[1].data());
		}
		fprintf(fp, "\n");
	}
	fclose(fp);
}

void UI::add_contact()
{
	vector<string> lable;
	lable.push_back("name");
	lable.push_back("telephone_number");
	vector<char> begin_char;
	begin_char.push_back('a');
	begin_char.push_back('0');
}
contact *UI::create_address_list()
{
	vector<string> lable;
	lable.push_back("name");
	lable.push_back("telephone_number");
	vector<char> begin_char;
	begin_char.push_back('a');
	begin_char.push_back('0');
	stringstream ss_tmp;
	ss_tmp << (++_num_address_list_);
	string num_tmp = ss_tmp.str();
	contact *tmp = new contact("New_Address_List_" + num_tmp, lable, begin_char);
	return tmp;
}

void UI::operation()
{

	int ch, val_;
	ch = _getch();
	if (ch == 224)
	{
		ch = _getch();
	}
	else if (ch == 0)
	{
		ch = _getch();
	}

	val_ = ch;
	if (val_ == 80) //光标向下移动
	{
		if (_now_list_->list_item(_now_item_loc_ + 1) == NULL)
			return;
		if (_cursor_loc_ == _lower_bound_ - _upper_bound_)
		{
			_upper_bound_++;
			list_range();
		}
		else
		{
			_cursor_loc_++;
		}
		_now_item_loc_++;
		_now_item_ = _now_list_->list_item(_now_item_loc_);
	}
	else if (val_ == 72) //光标向上移动
	{
		if (_now_list_->list_item(_now_item_loc_ - 1) == NULL)
			return;
		if (_cursor_loc_ == 0)
		{
			_upper_bound_--;
			list_range();
		}
		else
		{
			_cursor_loc_--;
		}
		_now_item_loc_--;
		_now_item_ = _now_list_->list_item(_now_item_loc_);
	}

	else if (val_ == 75) //光标向左移动
	{
		int val = _now_item_->val();
		_now_item_->val(val - 1);
	}
	else if (val_ == 77) //光标向右移动
	{
		int val = _now_item_->val();
		_now_item_->val(val + 1);
	}

	else if (val_ == 13) //进入目录
	{
		if (_now_item_ == _new_address_list_) //新建通讯录
		{

			_now_address_list_ = create_address_list();
			_address_list_.push_back(_now_address_list_);
			_contact_op_->fa(_now_list_);
			_contact_op_->name(_now_address_list_->name());
			_now_list_ = _contact_op_;
			//return;
		}
		else if (_now_item_ == _load_address_list_) //读取通讯录列表
		{
			vector<item *> list_item; //存储列表元素的临时变量
			item *item_info;		  //存储条目信息的临时变量
			list *list_info;		  //存储列表的临时变量
			vector<string> val_key;	  //存储条目键值的临时变量
			val_key.clear();
			string explain = "Defaut"; //存储条目注释的临时变量
			for (int i = 0; i < _address_list_.size(); i++)
			{
				item *item_info = new item(NULL, val_key, _address_list_[i]->name(), explain);
				list_item.push_back(item_info);
			}
			_address_list_list_tmp_ = new list(0, _root_, list_item, "All Address List");
			_address_list_list_tmp_->fa(_now_list_);
			_now_list_ = _address_list_list_tmp_;
			_now_list_ -> fa(_root_);
		}
		else if (_now_list_ == _address_list_list_tmp_)	//选择一个通讯录
		{
			_contact_op_->fa(_now_list_);
			_contact_op_->name(_address_list_[_now_item_loc_]->name());
			_now_address_list_ = _address_list_[_now_item_loc_];
			_now_list_ = _contact_op_;
		}
		
		//对一个通讯录进行操作
		else if (_now_item_ == _show_address_list_) //显示通讯录联系人列表
		{

			vector<item *> list_item; //存储列表元素的临时变量
			item *item_info;		  //存储条目信息的临时变量
			list *list_info;		  //存储列表的临时变量
			vector<string> val_key;	  //存储条目键值的临时变量
			val_key.clear();
			string explain = "Defaut"; //存储条目注释的临时变量

			vector<data> contacts_ = _now_address_list_->show(0);
			vector<string> name_tmp;
			for (int i = 0; i < contacts_.size(); i++)
			{
				name_tmp.clear();
				name_tmp.push_back(contacts_[i].info(1));
				item *item_info = new item(NULL, name_tmp, contacts_[i].info(0), explain);
				list_item.push_back(item_info);
			}
			_address_list_tmp_ = new list(0, _root_, list_item, _now_address_list_->name());
			_address_list_tmp_->fa(_now_list_);
			_now_list_ = _address_list_tmp_;
		}
		else if (_now_item_ == _add_contact_) //添加联系人
		{
			bool wrong_input = true;
			string name_, phone_number_;
			char ch;
			printf("Allowed chars for name: [a-z, A-Z]\n");
			printf("Allowed chars for phone number: [0-9]\n");
			printf("Illigal input wound not be accepted.\n");
			printf("Input the name of the new contact :\n");
			while (wrong_input)
			{
				wrong_input = false;
				name_.clear();
				ch = getchar();
				while (ch != '\r' && ch != '\n')
				{
					name_.append(1, ch);
					if (!((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')))
						wrong_input = true;
					ch = getchar();
				}
			}
			//transform(name_.begin(), name_.end(), name_.begin(), ::tolower);
			printf("Input the telephone number of the new contact :\n");
			wrong_input = true;
			while (wrong_input)
			{
				wrong_input = false;
				phone_number_.clear();
				ch = getchar();
				while (ch != '\r' && ch != '\n')
				{
					phone_number_.append(1, ch);
					if (!(ch >= '0' && ch <= '9'))
						wrong_input = true;
					ch = getchar();
				}
			}
			vector<string> lable;
			lable.push_back("name");
			lable.push_back("telephone_number");
			vector<string> info;
			info.push_back(name_);
			info.push_back(phone_number_);
			_now_address_list_->insert(info, lable);
			render_menu();
			return;
		}
		else if (_now_item_ == _rename_address_list_)	//重命名通讯录
		{
			bool wrong_input = true;
			string name_;
			char ch;
			printf("Allowed any normal chars for the name of an address list.\n");
			printf("Fucking input wound not be accepted.\n");
			printf("Input the name of the address list :\n");
			while (wrong_input)
			{
				wrong_input = false;
				name_.clear();
				ch = getchar();
				while (ch != '\r' && ch != '\n')
				{
					name_.append(1, ch);
					ch = getchar();
				}
			}
			_now_address_list_->name(name_);
			_now_list_->name(name_);
			render_menu();
			return;
		}
		else if (_now_item_ == _delete_address_list_) //删除通讯录
		{
			
			int loc_ = -1;
			for (int i = 0; i < _address_list_.size(); i++)
			{
				if (_now_address_list_ == _address_list_[i])
				{
					loc_ = i;
				}
			}
			delete _address_list_[loc_];
			std::vector<contact *>::iterator it = _address_list_.begin() + loc_;
			_address_list_.erase(it);

			list *_fa_tmp_ = _now_list_->fa() -> fa();
			delete _address_list_list_tmp_;
			_address_list_list_tmp_ = NULL;
			vector<item *> list_item; //存储列表元素的临时变量
			item *item_info;		  //存储条目信息的临时变量
			list *list_info;		  //存储列表的临时变量
			vector<string> val_key;	  //存储条目键值的临时变量
			val_key.clear();
			string explain = "Defaut"; //存储条目注释的临时变量
			for (int i = 0; i < _address_list_.size(); i++)
			{
				item *item_info = new item(NULL, val_key, _address_list_[i]->name(), explain);
				list_item.push_back(item_info);
			}
			_address_list_list_tmp_ = new list(0, _fa_tmp_, list_item, "All Address List");
			_address_list_list_tmp_->fa(_fa_tmp_);
			_now_list_ = _address_list_list_tmp_;
		}
		else if (_now_list_ == _address_list_tmp_) //选择一个联系人，并进行操作
		{
			if(_now_item_ == NULL) return;
			_contact_item_op_->name(_now_item_->name());
			_contact_item_op_->fa(_now_list_);
			_now_list_ = _contact_item_op_;
		}
		else if (_now_item_ == _delete_contact_) //删除联系人操作
		{
			string name_tmp_ = _now_list_->name();
			data *info = _now_address_list_->query(name_tmp_, 0);
			_now_address_list_->erase(info->info());

			list *fa_tmp = _address_list_tmp_->fa();

			_address_list_tmp_->delete_element();
			delete _address_list_tmp_;
			_address_list_tmp_ = NULL;

			vector<item *> list_item; //存储列表元素的临时变量
			item *item_info;		  //存储条目信息的临时变量
			list *list_info;		  //存储列表的临时变量
			vector<string> val_key;	  //存储条目键值的临时变量
			val_key.clear();
			string explain = "Defaut"; //存储条目注释的临时变量

			vector<data> contacts_ = _now_address_list_->show(0);
			vector<string> name_tmp;
			for (int i = 0; i < contacts_.size(); i++)
			{
				name_tmp.clear();
				name_tmp.push_back(contacts_[i].info(1));
				item *item_info = new item(NULL, name_tmp, contacts_[i].info(0), explain);
				list_item.push_back(item_info);
			}
			_address_list_tmp_ = new list(0, _root_, list_item, _now_address_list_->name());
			_address_list_tmp_->fa(fa_tmp);
			_now_list_ = _address_list_tmp_;
		}
		else if (_now_item_ == _modify_contact_)	//修改联系人
		{
			//先删除当前联系人
			string name_tmp_ = _now_list_->name();
			data *info = _now_address_list_->query(name_tmp_, 0);
			_now_address_list_->erase(info->info());
			//在新添加一个联系人
			bool wrong_input = true;
			string name_, phone_number_;
			char ch;
			printf("Allowed chars for name: [a-z, A-Z]\n");
			printf("Allowed chars for phone number: [0-9]\n");
			printf("Illigal input wound not be accepted.\n");
			printf("Input the name of the new contact :\n");
			while (wrong_input)
			{
				wrong_input = false;
				name_.clear();
				ch = getchar();
				while (ch != '\r' && ch != '\n')
				{
					name_.append(1, ch);
					if (!((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')))
						wrong_input = true;
					ch = getchar();
				}
			}
			//transform(name_.begin(), name_.end(), name_.begin(), ::tolower);
			printf("Input the telephone number of the new contact :\n");
			wrong_input = true;
			while (wrong_input)
			{
				wrong_input = false;
				phone_number_.clear();
				ch = getchar();
				while (ch != '\r' && ch != '\n')
				{
					phone_number_.append(1, ch);
					if (!(ch >= '0' && ch <= '9'))
						wrong_input = true;
					ch = getchar();
				}
			}
			vector<string> lable;
			lable.push_back("name");
			lable.push_back("telephone_number");
			vector<string> info_;
			info_.push_back(name_);
			info_.push_back(phone_number_);
			_now_address_list_->insert(info_, lable);

			list *fa_tmp = _address_list_tmp_->fa();

			_address_list_tmp_->delete_element();
			delete _address_list_tmp_;
			_address_list_tmp_ = NULL;

			vector<item *> list_item; //存储列表元素的临时变量
			item *item_info;		  //存储条目信息的临时变量
			list *list_info;		  //存储列表的临时变量
			vector<string> val_key;	  //存储条目键值的临时变量
			val_key.clear();
			string explain = "Defaut"; //存储条目注释的临时变量

			vector<data> contacts_ = _now_address_list_->show(0);
			vector<string> name_tmp;
			for (int i = 0; i < contacts_.size(); i++)
			{
				name_tmp.clear();
				name_tmp.push_back(contacts_[i].info(1));
				item *item_info = new item(NULL, name_tmp, contacts_[i].info(0), explain);
				list_item.push_back(item_info);
			}
			_address_list_tmp_ = new list(0, _root_, list_item, _now_address_list_->name());
			_address_list_tmp_->fa(fa_tmp);
			_now_list_ = _address_list_tmp_;
		}
		else if (_now_item_->son() == NULL)
			return;
		else
			_now_list_ = _now_item_->son();
		_now_item_ = _now_list_->list_item(0);
		_upper_bound_ = 0;
		list_range();
		_cursor_loc_ = 0;
		_now_item_loc_ = 0;
	}
	else if (val_ == 27) //离开当前目录
	{
		if (_now_list_ == _root_)
			exit(0);
		if (_now_list_ == _address_list_list_tmp_) //离开通讯录列表
		{
			_now_list_->delete_element();
			_now_list_ = _now_list_->fa();
			delete _address_list_list_tmp_;
			_address_list_list_tmp_ = NULL;
			//_now_list_ = _root_;
		}
		else if (_now_list_ == _address_list_tmp_)	//离开通讯录
		{
			_now_list_->delete_element();
			_now_list_ = _now_list_->fa();
			delete _address_list_tmp_;
			_address_list_tmp_ = NULL;
		}
		else if (_now_list_ == _contact_op_)	//离开通讯录操作界面
		{
			_now_address_list_ = NULL;
			_now_list_ = _now_list_->fa();
			if(_now_list_ == _address_list_list_tmp_)
			{
				vector<item *> list_item; //存储列表元素的临时变量
				item *item_info;		  //存储条目信息的临时变量
				list *list_info;		  //存储列表的临时变量
				vector<string> val_key;	  //存储条目键值的临时变量
				val_key.clear();
				string explain = "Defaut"; //存储条目注释的临时变量
				for (int i = 0; i < _address_list_.size(); i++)
				{
					item *item_info = new item(NULL, val_key, _address_list_[i]->name(), explain);
					list_item.push_back(item_info);
				}
				_address_list_list_tmp_ = new list(0, _root_, list_item, "All Address List");
				_address_list_list_tmp_->fa(_now_list_);
				_now_list_ = _address_list_list_tmp_;
				_now_list_ -> fa(_root_);
			}
		}
		else if (_now_list_ == _address_list_tmp_)	//离开通讯录联系人列表
		{
			_now_list_ = _now_list_->fa();
			_contact_item_op_->fa(NULL);
		}
		else
			_now_list_ = _now_list_->fa();
		_now_item_ = _now_list_->list_item(0);
		_upper_bound_ = 0;
		list_range();
		_cursor_loc_ = 0;
		_now_item_loc_ = 0;
		if (_now_list_ == _root_)
			fprint_address_list();
	}

	if (_now_item_ == _resolution_)
		resolution();
	if (_now_item_ == _V_SYNC_)
		V_SYNC();
	if (_now_item_ == _refresh_rate_)
		refresh_rate();
	if (_now_item_ == _developer_mode_)
		developer_mode();

	render_menu();
}

void UI::system_debug()
{
	printf("debug_info:\n");
	printf("_now_list_ = 0x%p\n", _now_list_);
	printf("_now_item_ = 0x%p\n", _now_item_);
	if (_now_item_)
		printf("_now_item_name = %s\n", _now_item_->name().data());
	else
		printf("_now_item_name = NULL\n");
	printf("_cursor_loc_ = %d\n", _cursor_loc_);
	printf("_now_item_loc_ = %d\n", _now_item_loc_);
	printf("_upper_bound_ = %d, _lower_bound_ = %d\n", _upper_bound_, _lower_bound_);
}

data::data(vector<string> info_)
{
	_info_ = info_;
}

vector<string> data::info()
{
	return _info_;
}

string data::info(int lable_)
{
	if (lable_ < 0 || lable_ >= _info_.size())
		return "NULL";
	return _info_[lable_];
}

void data::info(string info_, int lable_)
{
	_info_[lable_] = info_;
}

char data::begin_char(int lable_)
{
	return _begin_char_[lable_];
}

char trie_node::to_lower(char ch_)
{
	if (ch_ >= '0' && ch_ <= '9')
		return ch_;
	if (!(ch_ >= 'a' && ch_ <= 'z'))
		return 'a' + ch_ - 'A';
	else
		return ch_;
}

trie_node::trie_node(trie_node *fa_, char freq_, int depth_, int type_, char begin_char_)
{
	_fa_ = fa_;
	_freq_ = freq_;
	for (int i = 0; i < 26; i++)
	{
		_son_[i] = NULL;
	}
	_data_ = NULL;
	_depth_ = depth_;
	_type_ = type_;
	_begin_char_ = begin_char_;
}

void trie_node::insert(data *data_, string val_)
{
	int k;
	k = to_lower(val_[_depth_]) - _begin_char_;
	if (_depth_ == val_.size())
	{
		_data_ = data_;
		return;
	}
	if (_son_[k] == NULL)
	{
		_son_[k] = new trie_node(this, val_[_depth_], _depth_ + 1, _type_, _begin_char_);
	}
	_son_[k]->insert(data_, val_);
}

void trie_node::erase(string val_)
{
	int k;
	k = to_lower(val_[_depth_]) - _begin_char_;
	if (_depth_ == val_.size())
	{
		_data_ = NULL;
		return;
	}
	if (_son_[k] == NULL)
	{
		return;
	}
	_son_[k]->erase(val_);
}


data *trie_node::query(string val_)
{
	int k;
	k = to_lower(val_[_depth_]) - _begin_char_;
	if (_depth_ == val_.size())
	{
		return _data_;
	}
	else if (_son_[k] == NULL)
	{
		return NULL;
	}
	else
		return _son_[k]->query(val_);
}

void trie_node::get_all_data(trie_node *now, vector<data> &ans)
{
	if (this->_data() != NULL)
	{
		ans.push_back(*(this->_data()));
	}
	for (int i = 0; i < 26; i++)
	{
		if (_son_[i] == NULL)
			continue;
		_son_[i]->get_all_data(_son_[i], ans);
	}
}

vector<data> trie_node::all_data()
{
	vector<data> ans;
	get_all_data(this, ans);
	return ans;
}

data *trie_node::_data()
{
	return _data_;
}

contact::contact(string name_, vector<string> lable_, vector<char> begin_char_)
{
	_name_ = name_;
	_lable_ = lable_;
	_begin_char_ = begin_char_;
	int num_info = begin_char_.size();
	for (int i = 0; i < num_info; i++)
	{
		trie_node *new_trie = new trie_node(NULL, 0, 0, i, begin_char_[i]);
		_data_.push_back(new_trie);
	}
}

void contact::name(string name_)
{
	_name_ = name_;
}

string contact::name()
{
	return _name_;
}

void contact::insert(vector<string> info_, vector<string> lable_)
{
	data *tmp = _data_[0]->query(info_[0]);
	if (tmp != NULL)
		erase(tmp->info());
	tmp = new data(info_);
	for (int i = 0; i < _data_.size(); i++)
	{
		_data_[i]->insert(tmp, info_[i]);
	}
}
void contact::erase(vector<string> info_)
{
	data *tmp = _data_[0]->query(info_[0]);
	delete tmp;
	for (int i = 0; i < _data_.size(); i++)
	{
		_data_[i]->erase(info_[i]);
	}
}
void contact::modify(data *data_)
{
}
vector<data> contact::show(int type_) //0为按姓名排序，1为按号码排序
{
	return _data_[type_]->all_data();
}
data *contact::query(string val_key_, int type_)
{
	data *tmp = _data_[type_]->query(val_key_);
	return tmp;
}