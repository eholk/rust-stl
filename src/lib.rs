use std::io::{IoResult};

struct Triangle {
    normal: [f32, ..3],
    v1: [f32, ..3],
    v2: [f32, ..3],
    v3: [f32, ..3],
    attr_byte_count: u16
}

static EMPTY_TRIANGLE : Triangle = Triangle {
    normal: [0f32, 0f32, 0f32],
    v1: [0f32, 0f32, 0f32],
    v2: [0f32, 0f32, 0f32],
    v3: [0f32, 0f32, 0f32],
    attr_byte_count: 0
};

struct BinaryStlHeader {
    header: [u8, ..80],
    num_triangles: u32
}

pub struct BinaryStlFile {
    header: BinaryStlHeader,
    triangles: Vec<Triangle>
}

fn read_point<T: Reader>(input: &mut T) -> IoResult<[f32, ..3]> {
    let x1 = try!(input.read_le_f32());
    let x2 = try!(input.read_le_f32());
    let x3 = try!(input.read_le_f32());
    
    Ok([x1, x2, x3])
}

fn read_triangle<T: Reader>(input: &mut T) -> IoResult<Triangle> {
    let normal = try!(read_point(input));
    let v1 = try!(read_point(input));
    let v2 = try!(read_point(input));
    let v3 = try!(read_point(input));
    let attr_count = try!(input.read_le_u16());

    Ok(Triangle { normal: normal,
                  v1: v1, v2: v2, v3: v3,
                  attr_byte_count: attr_count })
}

pub fn read_stl<T: Reader>(input: &mut T) -> IoResult<BinaryStlFile> {
    let mut header = BinaryStlHeader { header: [0u8, ..80],
                                       num_triangles: 0 };

    // read the header
    try!(input.read_at_least(header.header.len(), &mut header.header));

    // TODO: check the header to make sure whether this is a binary
    // STL file.

    header.num_triangles = try!(input.read_le_u32());

    let mut triangles = Vec::new();
    for _ in range(0, header.num_triangles) {
        triangles.push(try!(read_triangle(input)));
    }

    Ok(BinaryStlFile {
        header: header,
        triangles: triangles
    })
}

#[test]
fn it_works() {
}
