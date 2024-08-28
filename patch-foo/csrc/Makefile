# 编译器
CC = gcc

# 编译器选项
CFLAGS = -Wall -g

# 目标可执行文件
TARGET = inject

# 源文件
SRCS = inject.c

# 对应的对象文件
OBJS = $(SRCS:.c=.o)

# 规则：默认目标
all: $(TARGET) add

# 规则：编译目标
$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $(TARGET) $(OBJS)
add:
	g++ -o add add.cpp
# 规则：生成对象文件
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# 规则：清理编译生成的文件
clean:
	rm -f $(OBJS) $(TARGET)
	rm -rf add
