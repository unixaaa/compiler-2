// Uses .rs extention for syntax highlighting, but this is not Rust source.
// omit function body to declare prototypes for external linking,"C" linkage optional, otherwise its' a C++ name-mangle with overloaded types..
fn"C" printf(s:str,...)->int;

// stolen from rust: function syntax 'fn <function name>(args) optional return value {body}
// however ommitting return value means infer it, not 'void' 

fn something(f:float){
}

// typeparameter sugar -omitted types get typeparams automatically,
// eg
// template<class A,class B,class F>
//    auto lerp(A a,B b, F f){return (b-a)*f+a;}

fn lerp(a,b,f){(b-a)*f+a};


//  declare a function taking a closure:
//  'funcp' is a variable of function type, 1arg 'int', result 'void'
//  functions declared like this are assumed to be closures
//  represented as a pair of pointers (function*, environment*)
//  raw C like functions are written fn(int)->void


fn take_closure(funcp:(int)->void){
    funcp(10);
}

// struct declarations like Rust.  fieldname:Type,...

struct Foo {
	vx:int, vy:int, vz:int
}

// open overloading like C++; most specific function is matched at callsite
// f:&Foo means parameter 'f' , reference to Foo.. 

fn something_foo(f:&Foo){
	printf("something_foo with 1 arg overloaded\n");
	printf("f.x= %d\n", f.vx);
}
fn something_foo(f:&Foo,x:&Foo){
	printf("something_foo with 2 args overloaded\n");
	printf("f.x= %d,.y= %d,.z= %d\n", f.vx,f.vy,f.vz);
}
fn something(f:&Foo){
	printf("something overloaded with &Foo param\n");
	printf("f.x= %d,.y= %d,.z= %d\n", f.vx, f.vy, f.vz);
}
fn something(f:float,x){
	printf("something overloaded with float param & templated param 'x'\n");
	printf("something(float, auto)\n");
}

// [X,Y] = typeparams like scala TODO we might want to accept both
// for familiarity. 
// [] reads better especially nested, nesting but < > are more widespread
//
// this isn't a union yet, its just  test to show the type-inference
// can handle getting a 'tag' from methods matching type X or Y
// we will probably introduce propper tagged unions like Rust, 
// but also want to use improved inference/templates to implement 'variant' better.
// using rawpointers we could implement any tag/data scheme
// (TODO: max[sizeof[X],sizeof[Y]] operators in template engine..)

struct Union[X,Y]{
	tag:int,
	x:X,y:Y,
};

fn setv[X,Y](u:&Union[X,Y],y:Y)->void{
	printf("setv Y\n");
	u.y=y;
	u.tag=1;
}

fn setv[X,Y](u:&Union[X,Y],x:X)->void{
	printf("setv X\n");
	u.x=x;
	u.tag=0;
}

fn map[X,Y,R](u:&Union[X,Y], fx:(&X)->R,fy:(&Y)->R)->R{
	if u.tag==0 { fx(&u.x)} else{fy(&u.y)}
}

fn main(argc:int,argv:**char)->int{
	printf("example program ./hello.rpp compiled & run by default makefile\n");

	// closure syntax stolen from Rust |args,..|{body...}
	captured_y:=0;
	take_closure(|x|{printf("closure says x=%d y=%d\n",x,captured_y);})

	// LHS=:RHS is temporary syntax to init empty var with given type Union<int,float> 
	//its a hack till we have let u:Union<int,float>.
	// inspiration from rust is to avoid un-init vars altogether using expression syntax.
	// the plan is to stick with rust 'let..', but keep 'go' like := in addition.
	u=:Union[int,float]; 

	// calls to templated functions
	setv(&u,2.0);
	setv(&u,5);
 
	// type inference with polymorphic lambdas
	// we could overload 'map' to supply different combinations of types
	// C++ equivalent doesn't seem to match all template args.
	// as far as i've tried it.. you always need to specify 
	// more params manually

	z:=map(&u,
		|x:&int|{printf("union was set to int %d\n",*x);15},
		|x:&float|{printf("union was set to float %.3f\n",*x);17}
	);
	printf("map union returned z=%d\n",z);

	// C-like for loops minus parens, compulsory {}
	// handles simple cases without needing a whole iterator library..
	// enhanced with expression syntax: break <expr> , else {expr}
	// type of 'value' is infered from the break/else expressions
	// inspired by Rusts nifty inference.

	acc:=0;
	value:=for i:=0,j:=0; i<10; i+=1,j+=10 {
		acc+=i;
		printf("i,j=%d,%d,x=%d\n",i,j,acc);
		if i==5{break 55}
	}else{
		// for..else block called if no 'break'
		printf("loop exit fine\n"); 
		44
	}
	printf("loop return value = %d\n",value);


	// Struct initializers...

	fv:=Foo{vx=13,vy=14,vz=15}; // initialize struct on stack, named fields
	something_foo(&fv,&fv);
	printf("fv.vx=%d\n",fv.vx);

	fv2:=new Foo{vx=23,vy=24,vz=25}; // struct allocate & init
	something_foo(fv2);

	fv3:=new Foo{31,32,33}; // struct initializer, sequential
	something_foo(fv3);

	 // test arrays and ptrs work
	xs=:array[int,512];   // like C++ array<int,512>
	q:=xs[1]; p1:=&xs[1];
	xs[2]=10;
	xs[2]+=400;
	*p1=30;
	p1[3]=77;			// raw ptr offseting (&xs[1])[3] == xs[4]
	z:=5;
	y:=xs[1]+z+xs[2];
	printf("test refs to array:%d %d %d\n",xs[2], *p1,xs[4]);


	// Expression syntax stolen from rust.
	// if..else.. has a return value;more flexible than ternary op
	// because it uses compound statements
	// frees up '?' for other use, eg optional types..(TODO arbitrary operators)
	// last expression in the compound blocks is return value from block

	x1:=if argc<2{printf("argc %d <2",argc);1}else{printf("argc %d >2",argc);2};
	printf("\nHello World %d %d\n", x1, y );

	// Stolen from rust: last statement is a return value. 
	// takes some getting used to but makes semicolons significant and
	// interacts very well with expression syntax generally.

	0
}
