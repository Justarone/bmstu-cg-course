use super::prelude::*;

#[allow(non_upper_case_globals)]
static mut z_buffer: [[f64; constants::WIDTH]; constants::HEIGHT] =
[[f64::MIN; constants::WIDTH]; constants::HEIGHT];
#[allow(non_upper_case_globals)]
static mut color_buffer: [[u32; constants::WIDTH]; constants::HEIGHT] =
[[constants::DEFAULT_COLOR; constants::WIDTH]; constants::HEIGHT];

pub unsafe fn clear_buffers() {
    for line in z_buffer.iter_mut() {
        for pixel in line.iter_mut() {
            *pixel = constants::MIN_Z;
        }
    }

    for line in color_buffer.iter_mut() {
        for color in line.iter_mut() {
            *color = constants::DEFAULT_COLOR;
        }
    }
}

pub unsafe fn transform_and_add(
    points_and_normals: &(Vec<Vec<Point3d>>, Vec<Vec<Point3d>>),
    matrix: &Matrix4,
    light_source: Vec3d,
    color: u32,
) {
    let (points_groups, normals_groups) = points_and_normals;

    for (points, normals) in points_groups.iter().zip(normals_groups.iter()) {
        let (p1, p2) = (
            transform_and_normalize(points[0], normals[0], matrix),
            transform_and_normalize(points[1], normals[1], matrix),
        );
        let mut current_window = [p1, p2, (Point3d::default(), Vec3d::default())];

        for (change_index, (&new_point, &new_normal)) in (2..)
            .map(|elem| elem % 3)
                .zip(points.iter().skip(2).zip(normals.iter().skip(2)))
                {
                    current_window[change_index] = transform_and_normalize(new_point, new_normal, matrix);
                    if check_pos_all(current_window.iter().map(|elem| elem.0))
                        && check_normals_all(current_window.iter().map(|elem| elem.1))
                    {
                        // no way to build slice from iterator :(
                        let points = [
                            current_window[0].0,
                            current_window[1].0,
                            current_window[2].0,
                        ];
                        let normals = [
                            current_window[0].1,
                            current_window[1].1,
                            current_window[2].1,
                        ];
                        add_polygon(points, normals, &light_source, color);
                    }
                }
    }
}

pub unsafe fn flush(pb: Pixbuf) {
    for (i, line) in color_buffer.iter().enumerate() {
        for (j, pixel) in line.iter().enumerate() {
            pb.put_pixel(
                j as u32,
                i as u32,
                (pixel >> 24) as u8,
                (pixel >> 16 & 0xFF) as u8,
                (pixel >> 8 & 0xFF) as u8,
                (pixel & 0xFF) as u8,
            );
        }
    }
}

unsafe fn add_polygon(
    points: [Point3d; 3],
    mut normals: [Vec3d; 3],
    light_source: &Vec3d,
    color: u32,
) {
    let mut int_points = [
        IntYPoint3d::from(points[0]),
        IntYPoint3d::from(points[1]),
        IntYPoint3d::from(points[2]),
    ];
    sort_by_y(&mut int_points, &mut normals);
    let brightnesses = find_brightnesses(normals, light_source);
    let sections = divide_on_sections(int_points, brightnesses);
    process_sections(sections, color);
}

unsafe fn process_sections(mut sections: [Section; 4], color: u32) {
    for pair in sections.chunks_mut(2) {
        if pair[0].x_start > pair[1].x_start {
            continue;
        }

        if pair[0].y_start < 0 {
            let diff = (-pair[0].y_start) as f64;
            for sec in pair.iter_mut() {
                sec.x_start += diff * sec.x_step;
                sec.br_start += diff * sec.br_step;
                sec.z_start += diff * sec.z_step;
            }
            pair[0].y_start = 0;
        }

        for y in (pair[0].y_start..=pair[0].y_end).filter(|&elem| elem < constants::HEIGHT as i16)
            .map(|y| y as usize)
        {
            let x_from = f64::round(pair[0].x_start) as usize;
            let x_to = f64::round(pair[1].x_start) as usize;
            let diff_x = (x_to - x_from) as f64;

            let mut br = pair[0].br_start;
            let br_diff = (pair[1].br_start - br) / diff_x;
            let mut z = pair[0].z_start;
            let z_diff = (pair[1].z_start - z) / diff_x;

            for x in (x_from..=x_to).filter(|&x| x < constants::WIDTH) {
                if z > z_buffer[y][x] {
                    z_buffer[y][x] = z;
                    put_color(x, y, color, br);
                }

                br += br_diff;
                z += z_diff;
            }

            for sec in pair.iter_mut() {
                sec.x_start += sec.x_step;
                sec.br_start += sec.br_step;
                sec.z_start += sec.z_step;
            }
        }
    }
}

unsafe fn put_color(x: usize, y: usize, color: u32, br: f64) {
    let (r, g, b, a) = (
        (color >> 24) as f64 * br,
        (color >> 16 & 0xFF) as f64 * br,
        (color >> 8 & 0xFF) as f64 * br,
        (color & 0xFF) as u32,
    );
    let (r, g, b, a) = (
        (f64::round(r) as u32) << 24,
        (f64::round(g) as u32) << 16,
        (f64::round(b) as u32) << 8,
        a,
    );
    let color = r + g + b + a;
    color_buffer[y][x] = color;
}

unsafe fn sort_by_y(int_points: &mut [IntYPoint3d; 3], normals: &mut [Vec3d; 3]) {
    for (&i, &j) in [0, 0, 1].iter().zip([2, 1, 2].iter()) {
        let condition = {
            let (a, b) = (&int_points[i], &int_points[j]);
            a.y > b.y || a.y == b.y && a.x > b.x
        };
        if condition {
            int_points.swap(i, j);
            normals.swap(i, j);
        }
    }
}

unsafe fn find_brightnesses(normals: [Vec3d; 3], light_source: &Vec3d) -> [f64; 3] {
    [
        constants::ZERO_BRIGHTNESS
            + constants::BRIGHTNESS_RANGE * (normals[0].scalar_mul(light_source)),
            constants::ZERO_BRIGHTNESS
                + constants::BRIGHTNESS_RANGE * (normals[1].scalar_mul(light_source)),
                constants::ZERO_BRIGHTNESS
                    + constants::BRIGHTNESS_RANGE * (normals[2].scalar_mul(light_source)),
    ]
}

unsafe fn divide_on_sections(int_points: [IntYPoint3d; 3], brightnesses: [f64; 3]) -> [Section; 4] {
    if int_points[0].y == int_points[2].y {
        return [
            Section::new(
                &int_points[0],
                &int_points[2],
                brightnesses[0],
                brightnesses[2],
            ),
            Section::new(
                &int_points[2],
                &int_points[0],
                brightnesses[2],
                brightnesses[0],
            ),
            Section::new(
                &int_points[2],
                &int_points[0],
                brightnesses[2],
                brightnesses[0],
            ),
            Section::new(
                &int_points[0],
                &int_points[2],
                brightnesses[0],
                brightnesses[2],
            ),
            ];
    };

    let midpoint2 = find_midpoint2(&int_points[0], &int_points[2], int_points[1].y);
    let midbrightness = brightnesses[0]
        + (brightnesses[2] - brightnesses[0])
        * ((int_points[1].y - int_points[0].y) as f64
            / (int_points[2].y - int_points[0].y) as f64);

    if midpoint2.x > int_points[1].x {
        [
            Section::new(
                &int_points[0],
                &int_points[1],
                brightnesses[0],
                brightnesses[1],
            ),
            Section::new(&int_points[0], &midpoint2, brightnesses[0], midbrightness),
            Section::new(
                &int_points[1],
                &int_points[2],
                brightnesses[1],
                brightnesses[2],
            ),
            Section::new(&midpoint2, &int_points[2], midbrightness, brightnesses[2]),
        ]
    } else {
        [
            Section::new(&int_points[0], &midpoint2, brightnesses[0], midbrightness),
            Section::new(
                &int_points[0],
                &int_points[1],
                brightnesses[0],
                brightnesses[1],
            ),
            Section::new(&midpoint2, &int_points[2], midbrightness, brightnesses[2]),
            Section::new(
                &int_points[1],
                &int_points[2],
                brightnesses[1],
                brightnesses[2],
            ),
        ]
    }
}

fn find_midpoint2(min: &IntYPoint3d, max: &IntYPoint3d, mid_y: i16) -> IntYPoint3d {
    let mult = if max.y == min.y {
        1.0
    } else {
        (mid_y - min.y) as f64 / (max.y - min.y) as f64
    };
    IntYPoint3d {
        x: min.x + (max.x - min.x) * mult,
        y: mid_y,
        z: min.z + (max.z - min.z) * mult,
    }
}

fn check_pos_all<Iter>(mut points: Iter) -> bool
where
    Iter: Iterator<Item = Point3d> + Clone,
{
    let all_left = points.clone().all(|p| p.x < 0_f64);
    let all_right = points.clone().all(|p| p.x >= constants::WIDTH as f64);
    let all_down = points.clone().all(|p| p.y < 0_f64);
    let all_up = points.all(|p| p.y >= constants::HEIGHT as f64);

    !(all_left || all_right || all_down || all_up)
}

#[allow(dead_code)]
fn check_normals_all<Iter>(mut normals: Iter) -> bool
where
    Iter: Iterator<Item = Vec3d>,
{
    if let Some(first) = normals.next() {
        let mut res = first.z >= constants::NEGATIVE_Z_PROJECTION;
        for norm in normals {
            res = res || norm.z >= constants::NEGATIVE_Z_PROJECTION;
        }
        res
    } else {
        false
    }
}

#[allow(dead_code)]
fn check_normals_all_sum<Iter>(mut normals: Iter) -> bool
where
    Iter: Iterator<Item = Vec3d>,
{
    if let Some(first) = normals.next() {
        let mut res = first;
        for norm in normals {
            res.add_assign(&norm);
        }
        res.z > constants::NEGATIVE_Z_PROJECTION
    } else {
        false
    }
}

fn transform_and_normalize(
    mut point: Point3d,
    mut norm_point: Point3d,
    matrix: &Matrix4,
) -> (Point3d, Vec3d) {
    matrix.apply_to_point(&mut point);
    matrix.apply_to_point(&mut norm_point);
    let mut norm_vec = Vec3d::from_pts(&point, &norm_point);
    norm_vec.normalize();

    (point, norm_vec)
}
