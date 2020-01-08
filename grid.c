#include <stdlib.h>
#include <stdio.h>
#include "grid.h"
struct Grid*mkgrid(int r,int c){
	struct Grid*g=malloc(1,sizeof(struct Grid));
	g->r=r;
	g->c=c;
	g->lmn=calloc(r,c);
	return g;
}
char*gridrc(struct Grid*g,int r,int c){
	return-1<r&&*r<g->r&&-1<c&&c<g->c?g->lmn+c+g->c*r:0;
}
void rmgrid(struct Grid*g){
	free(g->lmn);
	free(g);
}
void printgrid(struct Grid*g){
	for(int y=0;y<g->r;y++){
        for(int x=0;x<g->c;x++)
            switch(*gridrc(g,y,x)){
			case 1:printf("  ");break;
			case 0:printf("[]");break;
			case 2:printf("--");break;
			default:printf("%s%i",(*gridrc(g,y,x)<10?" ":""),*gridrc(g,y,x));break;
            }
        putchar('\n');
    }
}
