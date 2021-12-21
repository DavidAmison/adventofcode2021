
pub type XYZ = (isize, isize, isize);

#[derive(Clone, Copy)]
pub enum Axis {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}

pub fn orient(p: XYZ, axis: Axis, pointing: Axis) -> XYZ {
    // Cross product for
    let third = match (axis, pointing) {
        (Axis::PositiveX, Axis::PositiveY) => Axis::PositiveZ,
        (Axis::PositiveX, Axis::NegativeY) => Axis::NegativeZ,
        (Axis::PositiveX, Axis::PositiveZ) => Axis::NegativeY,
        (Axis::PositiveX, Axis::NegativeZ) => Axis::PositiveY,
        (Axis::NegativeX, Axis::PositiveY) => Axis::NegativeZ,
        (Axis::NegativeX, Axis::NegativeY) => Axis::PositiveZ,
        (Axis::NegativeX, Axis::PositiveZ) => Axis::PositiveY,
        (Axis::NegativeX, Axis::NegativeZ) => Axis::NegativeY,
        (Axis::PositiveY, Axis::PositiveZ) => Axis::PositiveX,
        (Axis::PositiveY, Axis::NegativeZ) => Axis::NegativeX,
        (Axis::PositiveY, Axis::PositiveX) => Axis::NegativeZ,
        (Axis::PositiveY, Axis::NegativeX) => Axis::PositiveZ,
        (Axis::NegativeY, Axis::PositiveZ) => Axis::NegativeX,
        (Axis::NegativeY, Axis::NegativeZ) => Axis::PositiveX,
        (Axis::NegativeY, Axis::PositiveX) => Axis::PositiveZ,
        (Axis::NegativeY, Axis::NegativeX) => Axis::NegativeZ,
        (Axis::PositiveZ, Axis::PositiveX) => Axis::PositiveY,
        (Axis::PositiveZ, Axis::NegativeX) => Axis::NegativeY,
        (Axis::PositiveZ, Axis::PositiveY) => Axis::NegativeX,
        (Axis::PositiveZ, Axis::NegativeY) => Axis::PositiveX,
        (Axis::NegativeZ, Axis::PositiveX) => Axis::NegativeY,
        (Axis::NegativeZ, Axis::NegativeX) => Axis::PositiveY,
        (Axis::NegativeZ, Axis::PositiveY) => Axis::PositiveX,
        (Axis::NegativeZ, Axis::NegativeY) => Axis::NegativeX,
        _ => panic!("INVALID AXIS COMBINATION"),
    };

    let mut p_out = [0, 0, 0];
    for (i, axis) in [axis, pointing, third].iter().enumerate() {
        match axis {
            Axis::PositiveX => p_out[i] = p.0,
            Axis::NegativeX => p_out[i] = -p.0,
            Axis::PositiveY => p_out[i] = p.1,
            Axis::NegativeY => p_out[i] = -p.1,
            Axis::PositiveZ => p_out[i] = p.2,
            Axis::NegativeZ => p_out[i] = -p.2,
        }
    }

    (p_out[0], p_out[1], p_out[2])
}

// /// Return all possible rotations of the point XYZ
// pub fn rotations(p: XYZ) -> Vec<XYZ> {
//     let mut output = Vec::new();
//     output.push((p.0, p.1, p.2));           // +x +y +z
//     output.push((p.0, -p.1, -p.2));         // +x -y -z
//     output.push((p.0, p.2, -p.1));          // +x +z -y
//     output.push((p.0, -p.2, p.1));          // +x -z +y
//     output.push((-p.0, p.1, -p.2));         // -x +y -z
//     output.push((-p.0, -p.1, p.2));         // -x -y +z
//     output.push((-p.0, p.2, p.1));          // -x +z +y
//     output.push((-p.0, -p.2, -p.1));        // -x -z -y
//     output.push((p.1, p.2, p.0));           // +y +z +x
//     output.push((p.1, -p.2, -p.0));         // +y -z -x
//     output.push((p.1, p.0, -p.2));          // +y +x -z
//     output.push((p.1, -p.0, p.2));          // +y -x +z
//     output.push((-p.1, p.2, -p.0));         // -y +z -x
//     output.push((-p.1, -p.2, p.0));         // -y -z +x
//     output.push((-p.1, p.0, p.2));          // -y +x +z
//     output.push((-p.1, -p.0, -p.2));        // -y -x -z
//     output.push((p.2, p.0, p.2));           // +z +x +y
//     output.push((p.2, -p.0, -p.2));         // +z -x -y
//     output.push((p.2, p.1, -p.2));          // +z +y -x
//     output.push((p.2, -p.1, p.2));          // +z -y +x
//     output.push((-p.2, p.0, -p.2));         // -z +x -y
//     output.push((-p.2, -p.0, p.2));         // -z -x +y
//     output.push((-p.2, p.1, p.2));          // -z +y +x
//     output.push((-p.2, -p.1, -p.2));        // -z -y -x

//     output
// }

pub fn rotations(points: &Vec<XYZ>) -> Vec<Vec<XYZ>> {
    let mut output = Vec::new();
    // for (i, p) in points.iter().enumerate() {
    //     // 6 axes
    //     for i in 0..6 {
    //         p = (p.0, p.1, p.2);  // rotate in ? axis
    //         // 4 rotations
    //         for j in 0..4 {

    //         }

    //     }
    // }
    for (i, p) in points.iter().enumerate() {
        if i == 0 {
            output.push(vec!(orient(*p, Axis::PositiveX, Axis::PositiveY)));    // +x +y +z
            output.push(vec!(orient(*p, Axis::PositiveX, Axis::NegativeY)));    // +x -y -z
            output.push(vec!(orient(*p, Axis::PositiveX, Axis::PositiveZ)));    // +x +z -y
            output.push(vec!(orient(*p, Axis::PositiveX, Axis::NegativeZ)));    // +x -z +y
            output.push(vec!(orient(*p, Axis::NegativeX, Axis::PositiveY)));    // -x +y -z
            output.push(vec!(orient(*p, Axis::NegativeX, Axis::NegativeY)));    // -x -y +z
            output.push(vec!(orient(*p, Axis::NegativeX, Axis::PositiveZ)));    // -x +z +y
            output.push(vec!(orient(*p, Axis::NegativeX, Axis::NegativeZ)));    // -x -z -y
            output.push(vec!(orient(*p, Axis::PositiveY, Axis::PositiveX)));    // +y +z +x
            output.push(vec!(orient(*p, Axis::PositiveY, Axis::NegativeX)));    // +y -z -x
            output.push(vec!(orient(*p, Axis::PositiveY, Axis::PositiveZ)));    // +y +x -z
            output.push(vec!(orient(*p, Axis::PositiveY, Axis::NegativeZ)));    // +y -x +z
            output.push(vec!(orient(*p, Axis::NegativeY, Axis::PositiveX)));    // -y +z -x
            output.push(vec!(orient(*p, Axis::NegativeY, Axis::NegativeX)));    // -y -z +x
            output.push(vec!(orient(*p, Axis::NegativeY, Axis::PositiveZ)));    // -y +x +z
            output.push(vec!(orient(*p, Axis::NegativeY, Axis::NegativeZ)));    // -y -x -z
            output.push(vec!(orient(*p, Axis::PositiveZ, Axis::PositiveY)));    // +z +x +y
            output.push(vec!(orient(*p, Axis::PositiveZ, Axis::NegativeY)));    // +z -x -y
            output.push(vec!(orient(*p, Axis::PositiveZ, Axis::PositiveX)));    // +z +y -x
            output.push(vec!(orient(*p, Axis::PositiveZ, Axis::NegativeX)));    // +z -y +x
            output.push(vec!(orient(*p, Axis::NegativeZ, Axis::PositiveY)));    // -z +x -y
            output.push(vec!(orient(*p, Axis::NegativeZ, Axis::NegativeY)));    // -z -x +y
            output.push(vec!(orient(*p, Axis::NegativeZ, Axis::PositiveX)));    // -z +y +x
            output.push(vec!(orient(*p, Axis::NegativeZ, Axis::NegativeX)));    // -z -y -x
        } else {
            output[0].push(orient(*p, Axis::PositiveX, Axis::PositiveY));       // +x +y +z
            output[1].push(orient(*p, Axis::PositiveX, Axis::NegativeY));       // +x -y -z
            output[2].push(orient(*p, Axis::PositiveX, Axis::PositiveZ));       // +x +z -y
            output[3].push(orient(*p, Axis::PositiveX, Axis::NegativeZ));       // +x -z +y
            output[4].push(orient(*p, Axis::NegativeX, Axis::PositiveY));       // -x +y -z
            output[5].push(orient(*p, Axis::NegativeX, Axis::NegativeY));       // -x -y +z
            output[6].push(orient(*p, Axis::NegativeX, Axis::PositiveZ));       // -x +z +y
            output[7].push(orient(*p, Axis::NegativeX, Axis::NegativeZ));       // -x -z -y
            output[8].push(orient(*p, Axis::PositiveY, Axis::PositiveX));       // +y +z +x
            output[9].push(orient(*p, Axis::PositiveY, Axis::NegativeX));       // +y -z -x
            output[10].push(orient(*p, Axis::PositiveY, Axis::PositiveZ));      // +y +x -z
            output[11].push(orient(*p, Axis::PositiveY, Axis::NegativeZ));      // +y -x +z
            output[12].push(orient(*p, Axis::NegativeY, Axis::PositiveX));      // -y +z -x
            output[13].push(orient(*p, Axis::NegativeY, Axis::NegativeX));      // -y -z +x
            output[14].push(orient(*p, Axis::NegativeY, Axis::PositiveZ));      // -y +x +z
            output[15].push(orient(*p, Axis::NegativeY, Axis::NegativeZ));      // -y -x -z
            output[16].push(orient(*p, Axis::PositiveZ, Axis::PositiveY));      // +z +x +y
            output[17].push(orient(*p, Axis::PositiveZ, Axis::NegativeY));      // +z -x -y
            output[18].push(orient(*p, Axis::PositiveZ, Axis::PositiveX));      // +z +y -x
            output[19].push(orient(*p, Axis::PositiveZ, Axis::NegativeX));      // +z -y +x
            output[20].push(orient(*p, Axis::NegativeZ, Axis::PositiveY));      // -z +x -y
            output[21].push(orient(*p, Axis::NegativeZ, Axis::NegativeY));      // -z -x +y
            output[22].push(orient(*p, Axis::NegativeZ, Axis::PositiveX));      // -z +y +x
            output[23].push(orient(*p, Axis::NegativeZ, Axis::NegativeX));      // -z -y -x
        }
    }

    output
}

pub fn rebase(p: XYZ, o: XYZ) -> XYZ {
    (
        p.0 - o.0,
        p.1 - o.1,
        p.2 - o.2,
    )
}