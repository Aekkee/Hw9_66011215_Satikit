use rand::Rng; 
use csv::Writer; 
use std::fs::File; 
use std::io::Read;
use std::io::Write;

fn main() {
    let list_layer = gen_obj_layer_list(4);
    save_list_to_csv(5, "list.csv");
    let data_file = File::open("list.csv").unwrap();
    avg_csv_to_html_with_min_max(data_file, "minmax.html")
    // println!("{:?}", list_layer);
    // println!("{:?}", cal_average_area(list_layer));
}

#[derive(Debug)]
struct Circle {
    x : f32,
    y : f32,
    r : f32
}

#[derive(Debug)]
struct Layer {
    name: String,
    color: String,
    objects: Vec<Circle>
}

// 1.1
fn gen_obj_layer_list(n: i32) -> Vec<Layer> {
    let mut rng = rand::thread_rng();
    let mut result: Vec<Layer> = Vec::new(); 
    for i in 1..=n {
        let mut objects: Vec<Circle> = Vec::new();
        let nc = rng.gen_range(20 ..= 50);
        for _ in 0..nc {
            objects.push(Circle{
                x: rng.gen_range(-100.0..=100.0),
                y: rng.gen_range(-100.0..=100.0),
                r: rng.gen_range(-10.0..=20.0)
            })
        }
        result.push( Layer{
            name: format!("Layer {}", i),
            color: format!("#{}", rng.gen_range(10000000 ..= 99999999)),
            objects
        })
    }
    result
}

#[test]
fn test_gen_obj_layer_list(){
    let layer1 = gen_obj_layer_list(5);
    let mut no = 1;
    for i in layer1 {
        for p in i.objects {
            assert!(p.x <= 100. && p.x >= -100. && p.y <= 100. && p.y >= -100. && p.r <= 20. && p.r >= -10.)
        }
        assert_eq!(i.name, format!("Layer {no}"));
        no += 1
    }
}

// 1.2
fn cal_average_area(layers: &Vec<Layer>) -> Vec<(String, f32)> {
    let mut result: Vec<(String, f32)> = Vec::new();
    if layers.is_empty() {
        return Vec::new()
    }
    for i in layers {
        let total: f32 = i.objects.iter().map(|x| x.r.powf(2.)*3.14).sum();
        let num: f32 = i.objects.len() as f32;
        result.push((i.name.to_string(), total/ num))
    }
    result
}

#[test]
fn test_cal_avg_area()  {
    let layer = gen_obj_layer_list(5);
    let avg_layer = cal_average_area(&layer);
    let mut no = 1;
    for i in avg_layer.iter().enumerate() {
        let total: f32 = layer[i.0].objects.iter().map(|x| x.r.powf(2.)*3.14).sum();
        let num: f32 = layer[i.0].objects.len() as f32;
        let avg = total/ num;
        assert_eq!(i.1.1, avg);
        assert_eq!(i.1.0, format!("Layer {no}"));
        no += 1
    }
}

// 2.1
fn save_list_to_csv(n: i32, output_file: &str) {
    let file_path = std::path::Path::new(output_file);
    let mut wtr = csv::Writer::from_path(file_path).unwrap();
    let _ = wtr.write_record(&["name", "color", "circle"]);
    let list = gen_obj_layer_list(n);
    for i in list {
        let _ = wtr.write_record(&[i.name, i.color, i.objects.iter().map(|c| format!("{}, {}, {},", c.x, c.y, c.r)).collect()]);
    }
}

// 2.2
fn csv_to_avg_csv(reader: impl Read, output_file: &str){
    let mut layer_list: Vec<Layer> = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    rdr.records().next();
    for result in rdr.records() {
        let mut objects: Vec<Circle> = Vec::new();
        let record = result.unwrap();
        let name = record[0].to_string();
        let color = record[1].to_string();
        let vecc: Vec<&str> = record[2].split(",").collect();
        for i in 0..vecc.len()/3-1 {
            let index = i*3;
            objects.push(Circle { x: vecc[index].parse().unwrap_or(0.), y: vecc[index+1][1..].parse().unwrap_or(0.), r: vecc[index+2][1..].parse().unwrap_or(0.) })
        }
        layer_list.push(Layer{name, color, objects});
    }
    let avg_list = cal_average_area(&layer_list);
    let file_path = std::path::Path::new(output_file);
    let mut wtr = Writer::from_path(file_path).unwrap();
    let _ = wtr.write_record(&["name", "average area"]);
    for i in avg_list {
        let _ = wtr.write_record(&[i.0, i.1.to_string()]);
    }
}

// 3.1
#[allow(unused_must_use)]
fn avg_csv_to_html(reader: impl Read, output_file: &str) {
    let mut file = File::create(output_file).expect("Failed");
    let mut layer_list: Vec<Layer> = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    rdr.records().next();
    for result in rdr.records() {
        let mut objects: Vec<Circle> = Vec::new();
        let record = result.unwrap();
        let name = record[0].to_string();
        let color = record[1].to_string();
        let vecc: Vec<&str> = record[2].split(",").collect();
        for i in 0..vecc.len()/3-1 {
            let index = i*3;
            objects.push(Circle { x: vecc[index].parse().unwrap_or(0.), y: vecc[index+1][1..].parse().unwrap_or(0.), r: vecc[index+2][1..].parse().unwrap_or(0.) })
        }
        layer_list.push(Layer{name, color, objects});
    }
    let avg_list: Vec<(String, f32)> = cal_average_area(&layer_list);
    file.write(b"<style>");
    file.write(b"\ntable, td, th {\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}\n</style>\n");
    file.write(b"<table>\n");
    file.write(b"\t<tr>\n\t\t<th>name</th>\n\t\t<th>average area</th>\n\t</tr>");
    for i in  avg_list{
        file.write(
            format!(
                "\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>",
                i.0,
                i.1
            )
            .as_bytes(),
        );
    }
    file.write(b"\n</table>");
}

// 3.2
#[allow(unused_must_use)]
fn avg_csv_to_html_with_min_max(reader: impl Read, output_file: &str) {
    let mut file = File::create(output_file).expect("Failed");
    let mut layer_list: Vec<Layer> = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    rdr.records().next();
    for result in rdr.records() {
        let mut objects: Vec<Circle> = Vec::new();
        let record = result.unwrap();
        let name = record[0].to_string();
        let color = record[1].to_string();
        let vecc: Vec<&str> = record[2].split(",").collect();
        for i in 0..vecc.len()/3-1 {
            let index = i*3;
            objects.push(Circle { x: vecc[index].parse().unwrap_or(0.), y: vecc[index+1][1..].parse().unwrap_or(0.), r: vecc[index+2][1..].parse().unwrap_or(0.) })
        }
        layer_list.push(Layer{name, color, objects});
    }
    let mut avg_list: Vec<(String, f32, f32, f32)> = Vec::new();
    for i in layer_list {
        let total: f32 = i.objects.iter().map(|x| x.r.powf(2.)*3.14).sum();
        let num: f32 = i.objects.len() as f32;
        let mut sort_list: Vec<f32> = i.objects.iter().map(|x| x.r.powf(2.)*3.14).collect();
        sort_list.sort_by(|x, y| x.partial_cmp(&y).unwrap());
        avg_list.push((i.name, total/ num, sort_list[0], sort_list[sort_list.len()-1]))
    }
    file.write(b"<style>");
    file.write(b"\ntable, td, th {\n\tborder: 1px solid #000000;\n\tborder-collapse: collapse;\n}\n</style>\n");
    file.write(b"<table>\n");
    file.write(b"\t<tr>\n\t\t<th>name</th>\n\t\t<th>average area</th>\n\t\t<th>minimum</th>\n\t\t<th>maximum</th>\n\t</tr>");
    for i in  avg_list{
        file.write(
            format!(
                "\t<tr>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t\t<td>{}</td>\n\t</tr>",
                i.0,
                i.1,
                i.2,
                i.3
            )
            .as_bytes(),
        );
    }
    file.write(b"\n</table>");
}