use super::prelude::*;
use log::{ trace, debug };
use termion::{ color, style };
use itertools::Itertools;

static mut z_buffer: [[f64; constants::WIDTH]; constants::HEIGHT] =
    [[f64::MIN; constants::WIDTH]; constants::HEIGHT];
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


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_upper_case_globals)]
pub unsafe fn transform_and_flush(points_and_normals: &(Vec<Vec<Point3d>>, Vec<Vec<Point3d>>),
                                  matrix: &Matrix4, pb: Pixbuf, light_source: Vec3d, color: u32 /* Color maybe?! */) {
    let (points_groups, normals_groups) = points_and_normals;

    for (points, normals) in points_groups.iter().zip(normals_groups.iter()) {
        let (p1, p2) = (transform_and_normalize(points[0], normals[0], matrix), transform_and_normalize(points[1], normals[1], matrix));
        let mut current_window = [p1, p2, (Point3d::default(), Vec3d::default())];

        for (change_index, (&new_point, &new_normal)) in (2..).map(|elem| elem % 3).zip(points.iter().skip(2).zip(normals.iter().skip(2))) {
            *current_window.get_unchecked_mut(change_index) = transform_and_normalize(new_point, new_normal, matrix);

            if check_pos(&current_window[0].0, &current_window[1].0, &current_window[2].0) && 
                check_normals(&current_window[0].1, &current_window[1].1, &current_window[2].1) {
                // no way to build it from iterator
                let points = [current_window[0].0, current_window[1].0, current_window[2].0];
                let normals = [current_window[0].1, current_window[1].1, current_window[2].1];
                add_polygon(points, normals, &light_source, color);
            }
        }
    }

    flush(pb);
}

unsafe fn flush(pb: Pixbuf) {
    for (i, line) in color_buffer.iter().enumerate() {
        for (j, pixel) in line.iter().enumerate() {
            pb.put_pixel(j as u32, i as u32, (pixel >> 24) as u8, (pixel >> 16 & 0xFF) as u8,
                (pixel >> 8 & 0xFF) as u8, (pixel & 0xFF) as u8);
        }
    }
}

unsafe fn add_polygon(points: [Point3d; 3], mut normals: [Vec3d; 3], light_source: &Vec3d, color: u32) {
    //trace!("Trace ((float y) points before sorting): {:?}", points);
    let mut int_points = [IntYPoint3d::from(points[0]), IntYPoint3d::from(points[1]), IntYPoint3d::from(points[2])];
    sort_by_y(&mut int_points, &mut normals);
    //trace!("Trace ((int y) points after sorting): {:?}", int_points);
    let brightnesses = find_brightnesses(normals, light_source);
    let sections = divide_on_sections(int_points, brightnesses);
    process_sections(sections, color);
    //debug!("COLOR_BUFFER:\n{}", color_buffer[5][0]);
}

unsafe fn process_sections(mut sections: [Section; 4], color: u32) {
    for pair in sections.chunks_mut(2) {
        if pair[0].x_start > pair[1].x_start {
            continue;
        }
        for y in (pair[0].y_start as usize)..=(pair[0].y_end as usize) {
            //debug!("{}Inside outer loop{}", color::Fg(color::Yellow), style::Reset);
            let x_from = f64::round(pair[0].x_start) as usize;
            let x_to = f64::round(pair[1].x_start) as usize;
            let diff_x = (x_to - x_from) as f64;

            let mut br = pair[0].br_start;
            let br_diff = (pair[1].br_start - br) / diff_x;
            let mut z = pair[0].z_start;
            let z_diff = (pair[1].z_start - z) / diff_x;

            for x in x_from..=x_to {
                //debug!("{}Inside inner loop{}", color::Fg(color::Green), style::Reset);
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
    //debug!("br = {}", br);
    let (r, g, b, a) = ((color >> 24) as f64 * br, (color >> 16 & 0xFF) as f64 * br, 
        (color >> 8 & 0xFF) as f64 * br, (color & 0xFF) as u32);
    let (r, g, b, a) = ((f64::round(r) as u32) << 24, (f64::round(g) as u32) << 16, (f64::round(b) as u32) << 8, a);
    let color = r + g + b + a;
    *color_buffer.get_unchecked_mut(y).get_unchecked_mut(x) = color;
}

unsafe fn sort_by_y(int_points: &mut [IntYPoint3d; 3], normals: &mut [Vec3d; 3]) {
    for (&i, &j) in [0, 0, 1].iter().zip([2, 1, 2].iter()) {
        let condition = {
            let (a, b) = (int_points.get_unchecked(i), int_points.get_unchecked(j));
            a.y > b.y || a.y == b.y && a.x > b.x
        };
        if condition {
            int_points.swap(i, j);
            normals.swap(i, j);
        }
    }
}

unsafe fn find_brightnesses(normals: [Vec3d; 3], light_source: &Vec3d) -> [f64; 3] {
    [0.8 + 0.5 * (normals.get_unchecked(0).scalar_mul(light_source)), 0.8 + 0.5 * (normals.get_unchecked(1).scalar_mul(light_source)),
        0.8 + 0.5 * (normals.get_unchecked(2).scalar_mul(light_source))]
}

unsafe fn divide_on_sections(int_points: [IntYPoint3d; 3], brightnesses: [f64; 3]) -> [Section; 4] {
    //trace!("Trace (points to make sections): {:?}", int_points);
    // one line: that's not a general case, so we need to iterate from min.x to max.x (FIXME: now
    // it's terrible, because I need only one outer iteration over one line, and in this case I
    // have to have 2)
    if int_points.get_unchecked(0).y == int_points.get_unchecked(2).y {
        //trace!("{}Trace: One line case! Higher you may see coordinates...{}",
            //color::Fg(color::Red), style::Reset);
        return [Section::new(int_points.get_unchecked(0), int_points.get_unchecked(2), *brightnesses.get_unchecked(0), *brightnesses.get_unchecked(2)),
                Section::new(int_points.get_unchecked(2), int_points.get_unchecked(0), *brightnesses.get_unchecked(2), *brightnesses.get_unchecked(0)),
                Section::new(int_points.get_unchecked(2), int_points.get_unchecked(0), *brightnesses.get_unchecked(2), *brightnesses.get_unchecked(0)),
                Section::new(int_points.get_unchecked(0), int_points.get_unchecked(2), *brightnesses.get_unchecked(0), *brightnesses.get_unchecked(2))]
    };

    let midpoint2 = find_midpoint2(&int_points.get_unchecked(0), &int_points.get_unchecked(2), int_points.get_unchecked(1).y);
    let midbrightness = brightnesses.get_unchecked(0) + (brightnesses.get_unchecked(2) - brightnesses.get_unchecked(0)) *
        ((int_points.get_unchecked(1).y - int_points.get_unchecked(0).y) as f64 /
         (int_points.get_unchecked(2).y - int_points.get_unchecked(0).y) as f64);

    if midpoint2.x > int_points.get_unchecked(1).x {
        [Section::new(int_points.get_unchecked(0), int_points.get_unchecked(1), *brightnesses.get_unchecked(0), *brightnesses.get_unchecked(1)),
            Section::new(int_points.get_unchecked(0), &midpoint2, *brightnesses.get_unchecked(0), midbrightness), 
            Section::new(int_points.get_unchecked(1), int_points.get_unchecked(2), *brightnesses.get_unchecked(1), *brightnesses.get_unchecked(2)),
            Section::new(&midpoint2, int_points.get_unchecked(2), midbrightness, *brightnesses.get_unchecked(2))]
    } else {
        [Section::new(int_points.get_unchecked(0), &midpoint2, *brightnesses.get_unchecked(0), midbrightness), 
            Section::new(int_points.get_unchecked(0), int_points.get_unchecked(1), *brightnesses.get_unchecked(0), *brightnesses.get_unchecked(1)),
            Section::new(&midpoint2, int_points.get_unchecked(2), midbrightness, *brightnesses.get_unchecked(2)),
            Section::new(int_points.get_unchecked(1), int_points.get_unchecked(2), *brightnesses.get_unchecked(1), *brightnesses.get_unchecked(2))]
    }
}

fn find_midpoint2(min: &IntYPoint3d, max: &IntYPoint3d, mid_y: u16) -> IntYPoint3d {
    let mult = if max.y == min.y { 1.0 } else { (mid_y - min.y) as f64 / (max.y - min.y) as f64 };
    IntYPoint3d {
        x: min.x + (max.x - min.x) * mult,
        y: mid_y,
        z: min.z + (max.z - min.z) * mult,
    }
}

fn check_pos(p1: &Point3d, p2: &Point3d, p3: &Point3d) -> bool {
    let all_left = p1.x < 0_f64 && p2.x < 0_f64 && p3.x < 0_f64;
    let all_right = p1.x > constants::WIDTH as f64 && p2.x > constants::WIDTH as f64 && p3.x > constants::WIDTH as f64;
    let all_down = p1.y < 0_f64 && p2.y < 0_f64 && p3.y < 0_f64;
    let all_up = p1.y > constants::HEIGHT as f64 && p2.y > constants::HEIGHT as f64 && p3.y > constants::HEIGHT as f64;

    !(all_left || all_right || all_down || all_up)
}

fn check_normals(n1: &Vec3d, n2: &Vec3d, n3: &Vec3d) -> bool {
    let res = n1.add(n2);
    let res = res.add(n3);

    res.z > 0_f64
}

fn transform_and_normalize(mut point: Point3d, mut norm_point: Point3d, matrix: &Matrix4) -> (Point3d, Vec3d) {
    matrix.apply_to_point(&mut point);
    matrix.apply_to_point(&mut norm_point);
    let mut norm_vec = Vec3d::from_pts(&point, &norm_point);
    norm_vec.normalize();

    (point, norm_vec)
}
