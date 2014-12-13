use std::io::{BufReader, IoResult};

struct Triangle {
    normal: [f32, ..3],
    v1: [f32, ..3],
    v2: [f32, ..3],
    v3: [f32, ..3],
    attr_byte_count: u16
}

fn point_eq(lhs: [f32, ..3], rhs: [f32, ..3]) -> bool {
    lhs[0] == rhs[0] && lhs[1] == rhs[1] && lhs[2] == rhs[2]
}

impl PartialEq for Triangle {
    fn eq(&self, rhs: &Triangle) -> bool {
        point_eq(self.normal, rhs.normal)
            && point_eq(self.v1, rhs.v1)            
            && point_eq(self.v2, rhs.v2)            
            && point_eq(self.v3, rhs.v3)
            && self.attr_byte_count == rhs.attr_byte_count
    }
}

impl Eq for Triangle {}

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

fn write_point<T: Writer>(out: &mut T, p: [f32, ..3]) -> IoResult<()> {
    for x in p.iter() {
        try!(out.write_le_f32(*x));
    }
    Ok(())
}

pub fn write_stl<T: Writer>(out: &mut T, stl: &BinaryStlFile) -> IoResult<()> {
    assert!(stl.header.num_triangles as uint == stl.triangles.len());

    //write the header.
    try!(out.write(&stl.header.header));
    try!(out.write_le_u32(stl.header.num_triangles));
    
    // write all the triangles
    for t in stl.triangles.iter() {
        try!(write_point(out, t.normal));
        try!(write_point(out, t.v1));
        try!(write_point(out, t.v2));
        try!(write_point(out, t.v3));
        try!(out.write_le_u16(t.attr_byte_count));
    }

    Ok(())
}

#[test]
fn write_read() {
    // Make sure we can write and read a simple file.
    let file = BinaryStlFile {
        header: BinaryStlHeader { header: [0u8, ..80],
                                  num_triangles: 1 },
        triangles: vec![Triangle { normal: [0f32, 1f32, 0f32],
                                   v1: [0f32, 0f32, 0f32],
                                   v2: [0f32, 0f32, 1f32],
                                   v3: [1f32, 0f32, 1f32],
                                   attr_byte_count: 0 }]
    };

    let mut buffer = Vec::new();

    match write_stl(&mut buffer, &file) {
        Ok(_) => (),
        Err(_) => panic!()
    }

    match read_stl(&mut BufReader::new(buffer.as_slice())) {
        Ok(stl) => {
            assert!(stl.header.num_triangles == file.header.num_triangles);
            assert!(stl.triangles.len() == 1);
            assert!(stl.triangles[0] == file.triangles[0])
        },
        Err(_) => panic!()
    }
}
