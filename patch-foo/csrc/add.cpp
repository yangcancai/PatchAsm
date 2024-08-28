#include <stdio.h>
#include <string.h>
#include<iostream>

    char c = '1';
class Person{
    public:
    int id;
    void set_name(char* n);
    char* get_name();
    private:
    char name[20];
};
void Person::set_name(char* n){
    snprintf(name, sizeof(name), n);
}
char* Person::get_name(){
    return name;
}
Person *p1 = new Person;
int add(int a, int b) {
    return a + b;
}
int main(int argc, char *argv[]) {
    // 将命令行参数从字符串转换为整数
    int a = 10;
    
    p1->id = 1;
    p1->set_name("John Doe");
    while (c != 48)
    {
        int res = add(1, 2);
        printf("res = %d, c=%d, a=%d, person.id=%d, person.name=%s, addr = %#018llx\n", res,c, a,p1->id,p1->get_name(), p1);
        char i = getchar();
        if (i != 10 && i >= '0' && i <= '9' || i >= 'a' && i <= 'z'){
            c = i;
        }
    }
    delete p1;
    return 0;
}
 