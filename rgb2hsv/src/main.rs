use std::env;

// 正規化する必要があるので浮動小数点数で格納する
struct RGB {
    red: f64,
    green: f64,
    blue: f64
}

#[derive(Debug)]
struct HSV {
    h: f64,
    s: f64,
    v: f64
}

impl RGB {
    fn to_hsv(&self) -> HSV {
        let normalized_rgb = RGB {
            red: self.red / 255.0,
            green: self.green / 255.0,
            blue: self.blue / 255.0,
        };
        let max = if normalized_rgb.red > normalized_rgb.green {
            if normalized_rgb.red > normalized_rgb.blue {
                normalized_rgb.red
            } else {
                normalized_rgb.blue
            }
        } else {
            if normalized_rgb.green > normalized_rgb.blue {
                normalized_rgb.green
            } else {
                normalized_rgb.blue
            }
        };
        let min = if normalized_rgb.red < normalized_rgb.green {
            if normalized_rgb.red < normalized_rgb.blue {
                normalized_rgb.red
            } else {
                normalized_rgb.blue
            }
        } else {
            if normalized_rgb.green < normalized_rgb.blue {
                normalized_rgb.green
            } else {
                normalized_rgb.blue
            }
        };
        let h = if min != max {
            if min == normalized_rgb.blue {
                60.0 * (normalized_rgb.green - normalized_rgb.red) / (max - min) + 60.0
            } else if min == normalized_rgb.red {
                60.0 * (normalized_rgb.blue - normalized_rgb.green) / (max - min) + 180.0
            } else {
                60.0 * (normalized_rgb.red - normalized_rgb.blue) / (max - min) + 300.0
            }
        } else {
            0.0
        };

        HSV {
            h,
            s: (max - min) * 100.0 / max,
            v: max * 100.0
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // HEXの先頭に#が来る可能性があるので削除しておく
    let hex_color = String::from(&args[1]).replace("#", "");
    // とりあえず適当に2文字ずつ抽出して10進数に変換
    let decimal_rgb = RGB {
        red: i64::from_str_radix(&hex_color[0..2], 16).unwrap() as f64,
        green: i64::from_str_radix(&hex_color[2..4], 16).unwrap() as f64,
        blue: i64::from_str_radix(&hex_color[4..6], 16).unwrap() as f64,
    };
    println!("{:#?}", decimal_rgb.to_hsv());
    
}
