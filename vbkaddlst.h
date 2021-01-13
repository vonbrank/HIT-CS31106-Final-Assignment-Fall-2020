#ifndef VBKADDLST_H
#define VBKADDLST_H

#include <iostream>
#include <cstdio>
#include <conio.h>
#include <ctime>
#include <windows.h>
#include <vector>
#include <string>
#include <cstring>
#include <cstdlib>
using namespace std;

class data
{
private:
	vector<string> _info_;	   //每个类型对应的值
	vector<string> _lable_;	   //每个数据的类型（标签）
	vector<char> _begin_char_; //每个数据类型的起始字符
							   //1:name, 2:phone_number, 3:...
public:
	data(vector<string> info_);
	string info(int lable_);
	vector<string> info();
	void info(string info_, int lable_);
	char begin_char(int lable_);
};

class trie_node	//字典树类
{
private:
	trie_node *_son_[26];
	trie_node *_fa_;
	char _freq_;
	char _begin_char_;
	data *_data_;
	int _depth_;
	int _type_;
	char to_lower(char ch_);

public:
	trie_node(trie_node *fa_, char freq_, int depth_, int type_, char begin_char_);
	void insert(data *data_, string val_);
	void erase(string val_);
	data *query(string val_);
	void get_all_data(trie_node *now, vector<data> &ans);
	vector<data> all_data();
	data *_data();
};

class contact	//通讯录类
{
private:
	vector<trie_node *> _data_;
	vector<string> _lable_; //每个数据的类型（标签）
	vector<char> _begin_char_;
	string _name_;

public:
	contact(string name_, vector<string> lable_, vector<char> begin_char_); //初始化通讯录
	void name(string name_);												//修改通讯录名称
	string name();															//返回通讯录名称
	void insert(vector<string> info_, vector<string> lable_);				//插入一个联系人
	void erase(vector<string> info_);										//删除一个联系人
	void modify(data *data_);												//修改联系人
	vector<data> show(int type_);											//0为按姓名排序，1为按号码排序
	data *query(string val_key_, int type_);								//查询一个联系人是否存在
};

class list;

class item
{
private:
	int _val_;				  //item的值
	list *_son_;			  //item的子目录列表
	vector<string> _val_key_; //item每一个键值
	string _name_;			  //item的名字
	string _explain_;		  //item的描述
public:
	item(list *son_, vector<string> val_key_, string name_, string explain_);
	int val();
	void val(int val_);
	list *son();
	void son(list *son_);
	//	list* son_();
	//	void son_(list* son_);
	string val_key();
	string val_key(int val_);
	vector<string> all_val_key();
	void val_key(string val_key_, int val_);
	void val_key(vector<string> val_key_);
	string name();
	void name(string name_);
	string explain();
	void explain(string explain_);
};

class list
{
private:
	bool _const_; //是否是不可删除列表
	list *_fa_;
	vector<item *> _item_; //列表下条目的指针
	string _name_;		   //列表名称
public:
	list(bool const_, list *fa_, vector<item *> item_, string name_);
	list *fa();
	void fa(list *fa_);
	void add_item(item *item_); //为列表添加一个条目
	item *list_item(int val_);	//返回第val_个条目的指针
	vector<item *> list_item(); //返回所有条目的指针
	string name();				//返回list的名称
	void name(string name_);	//将list的名称修改为name_
	void delete_element();
};

class UI	//图形界面类
{
private:
	//系统参数
	list *_root_;			 //根目录指针
	list *_contact_op_;		 //通讯录操作菜单指针
	list *_contact_item_op_; //显示单个联系人操作列表的临时指针
	list *_now_list_;		 //当前所在目录的指针
	//int _num_item_;	//条目数量
	item *_now_item_;	//当前选中条目
	int _now_item_loc_; //当前选择条目在此目录的位置
	//item* _now_item_fa_;	//当前所在目录
	//vector<item*>_item_address_;	//条目标签对应的地址
	char _map_[512][512]; //输出画面

	//渲染参数
	bool _V_SYNC_ON_;		//是否启用垂直同步
	int _refresh_rate_val_; //刷新率值
	int _upper_bound_;		//当前列表可见条目上限
	int _lower_bound_;		//当前列表可见条目下限
	int _height_, _width_;	//界面的宽与高
	int _cursor_loc_;		//光标位置
	int _margin_up_;		//页边距
	int _margin_down_;
	int _margin_left_;
	int _margin_right_;
	//系统指针
	item *_resolution_;		//分辨率参数指针
	item *_V_SYNC_;			//垂直同步参数指针
	item *_refresh_rate_;	//刷新率参数指针
	item *_developer_mode_; //开发者选项指针
	//操作指针
	item *_new_address_list_;  //新建通讯录条目指针
	item *_load_address_list_; //加载通讯录条目指针
	//系统变量
	vector<contact *> _address_list_;
	contact *_now_address_list_;
	int _num_address_list_;
	//生成列表指针
	list *_address_list_tmp_;	   //显示通讯录本身的临时指针
	list *_address_list_list_tmp_; //显示通讯录存档列表的临时指针

	//功能条目指针
	item *_show_address_list_;
	item *_add_contact_;
	item *_rename_address_list_;
	item *_delete_address_list_;

	item *_modify_contact_;
	item *_delete_contact_;

public:
	UI();
	void resolution();		//同步分辨率设置
	void refresh_rate();	//同步刷新率设置
	int refresh_rate_val(); //返回刷新率的值
	void V_SYNC();			//同步垂直同步设置
	bool V_SYNC_ON();		//返回垂直同步是否启用
	bool developer_mode();	//返回开发者选项是否启用
	void list_range();												   //同步列表渲染范围
	void init_map();												   //初始化帧
	void map_str_cpy(char *a, int s1, int len, const char *b, int s2); //向图像中复制数据
	void operation();												   //操作函数
	void add_contact();												   //新建通讯录
	contact *create_address_list();	//新建一个通讯录，并返回其指针
	void render_menu();	//将当前列表渲染到画布上
	void print();	//打印画布
	void fscan_address_list();	//从文件读取通讯录
	void fprint_address_list();	//将所有通讯录写入文件
	void system_debug();	//输出关键信息进行调试
};

#endif