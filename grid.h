#ifndef GRID
#define GRID
struct Grid{
	int r,c;
	char*lmn;
};
struct Grid*mkgrid(int r,int c);
char*gridrc(struct Grid*g,int r,int c);
void rmgrid(struct Grid*g);
void printgrid(struct Grid*g);
#endif
