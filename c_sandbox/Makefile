all: kqueue_example
epoll_example: epoll_example.c
	gcc -Wall -Werror -o $@ epoll_example.c
kqueue_example: kqueue_example.c
	gcc -Wall -Werror -o $@ kqueue_example.c
clean:
	@rm -v epoll_example kqueue_example