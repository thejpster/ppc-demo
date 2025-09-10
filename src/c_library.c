/*
Some C code we can use from our Rust code
*/


unsigned long clib_add_two_ulong(unsigned long x, unsigned long y) {
	return x + y;
}

float clib_add_two_float(float x, float y) {
	return x + y;
}

double clib_add_two_double(double x, double y) {
	return x + y;
}

typedef struct data_t {
	unsigned char x;
	double y;
	int z;
} data_t;

int get_int_field(data_t* ptr) {
	return ptr->z;
}

double get_double_field(data_t* ptr) {
	return ptr->y;
}

