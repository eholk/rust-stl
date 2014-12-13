struct Triangle {
    normal: [f32, ..3],
    v1: [f32, ..3],
    v2: [f32, ..3],
    v3: [f32, ..3],
    attr_byte_count: u16
}

struct BinaryStlHeader {
    header: [u8, ..80],
    num_triangles: u32
}

struct BinaryStlFile {
    header: BinaryStlHeader,
    triangles: [Triangle]
}

#[test]
fn it_works() {
}
