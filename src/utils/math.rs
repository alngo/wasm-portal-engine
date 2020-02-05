// #define vxs(x0,y0, x1,y1)	((x0)*(y1) - (x1)*(y0))   // vxs: Vector cross product
// // Overlap:  Determine whether the two number ranges overlap.
// #define Overlap(a0,a1,b0,b1) (min(a0,a1) <= max(b0,b1) && min(b0,b1) <= max(a0,a1))
// // IntersectBox: Determine whether two 2D-boxes intersect.
// #define IntersectBox(x0,y0, x1,y1, x2,y2, x3,y3) (Overlap(x0,x1,x2,x3) && Overlap(y0,y1,y2,y3))
// // PointSide: Determine which side of a line the point is on. Return value: <0, =0 or >0.
// #define PointSide(px,py, x0,y0, x1,y1) vxs((x1)-(x0), (y1)-(y0), (px)-(x0), (py)-(y0))
// // Intersect: Calculate the point of intersection between two lines.
// #define Intersect(x1,y1, x2,y2, x3,y3, x4,y4) ((struct xy) { \
// 	vxs(vxs(x1,y1, x2,y2), (x1)-(x2), vxs(x3,y3, x4,y4), (x3)-(x4)) / vxs((x1)-(x2), (y1)-(y2), (x3)-(x4), (y3)-(y4)), \
// 	vxs(vxs(x1,y1, x2,y2), (y1)-(y2), vxs(x3,y3, x4,y4), (y3)-(y4)) / vxs((x1)-(x2), (y1)-(y2), (x3)-(x4), (y3)-(y4)) })

pub fn vector_cross_product(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    ((x0 * y1) - (x1 * y0))
}

pub fn overlap(a0, a1, b0, b1) -> bool {
    (cmp::min(a0, a1) < cmp::max(b0, b1)) && (cmp::min(b0, b1) <= cmp::max(a0, a1))
}

pub fn IntersectBox() {
}
