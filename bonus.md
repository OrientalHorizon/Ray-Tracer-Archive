# Advanced features 记录

### 多线程

采取的思路是按采样分线程。我的电脑 cpu 是 16 threads 的，按照学长的建议开了 18 个线程。有些图采样次数为 100，我将它改成 108 次。

Code:

```rust 
let thread_num: u32 = 18;
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let pixel = img.get_pixel_mut(i, IMAGE_HEIGHT - j - 1);
            let mut pixel_color: Color3 = Color3::construct(&[0.0, 0.0, 0.0]);

            let mut handles: Vec<thread::JoinHandle<()>> = Vec::new(); // 各个线程
            let mut recv: Vec<mpsc::Receiver<Color3>> = Vec::new(); // 各个接收器的数组
            for _s in 0..thread_num {
                let (tx, rx) = mpsc::channel();
                recv.push(rx);
                let cam = cam.clone();
                let world = world.clone();
                let max_depth = MAX_DEPTH;
                let image_width = IMAGE_WIDTH;
                let image_height = IMAGE_HEIGHT;
                let i_f64 = i as f64;
                let j_f64 = j as f64;
                let lights_ptr = lights_ptr.clone();

                let handle = thread::spawn(move || {
                    for _t in 0..(SAMPLES_PER_PIXEL / thread_num) {
                        let u: f64 = (i_f64 + random_double()) / (image_width - 1) as f64;
                        let v: f64 = (j_f64 + random_double()) / (image_height - 1) as f64;
                        let r: Ray = cam.get_ray(u, v);
                        tx.send(ray_color(
                            &r,
                            &background,
                            &world,
                            lights_ptr.clone(),
                            max_depth,
                        ))
                        .unwrap();
                    }
                });
                handles.push(handle);
            }
            let mut cnt = 0u32;
            // 第一层遍历：遍历接收者
            for rec in recv {
                // 第二层遍历：遍历接收者收到的数据
                for received in rec {
                    pixel_color += received;
                    cnt += 1;
                }
            }
            for thread in handles {
                thread.join().unwrap();
            }
            assert_eq!(cnt, SAMPLES_PER_PIXEL);
            let rgb: [u8; 3] = write_color(&pixel_color, SAMPLES_PER_PIXEL);
            *pixel = image::Rgb(rgb);
            progress.inc(1);
        }
    }
    progress.finish();
```

### 替换 `Arc` 和 `dyn`

### 边缘检测

采用 Laplace 和 Sobel 算子。

核心代码：

```rust
// Laplace
for i in 0..width {
    for j in 0..height {
        if i == 0 || i == width - 1 || j == 0 || j == height - 1 {
            gen[i][j] = 0f64;
            continue;
        }

        let upleft = original[i - 1][j - 1];
        let up = original[i - 1][j];
        let upright = original[i - 1][j + 1];
        let left = original[i][j - 1];
        let center = original[i][j];
        let right = original[i][j + 1];
        let downleft = original[i + 1][j - 1];
        let down = original[i + 1][j];
        let downright = original[i + 1][j + 1];
        let sum =
            upleft + up + upright + left + right + downleft + down + downright - 8.0 * center;
        gen[i][j] = sum;
    }
}

// Sobel
let sobel_x: [[f64; 3]; 3] = { [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]] };
let sobel_y: [[f64; 3]; 3] = { [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]] };
for i in 0..width {
    for j in 0..height {
        if i == 0 || i == width - 1 || j == 0 || j == height - 1 {
            gen[i][j] = 0f64;
            continue;
        }
        let mut gx = 0.0;
        for k in 0..3 {
            for l in 0..3 {
                gx += sobel_x[k][l] * original[i + k - 1][j + l - 1];
            }
        }
        let mut gy = 0.0;
        for k in 0..3 {
            for l in 0..3 {
                gy += sobel_y[k][l] * original[i + k - 1][j + l - 1];
            }
        }
        let ans = (gx * gx + gy * gy).sqrt();
        // println!("{ans}");
        if ans > 58.0 {
            gen[i][j] = 0.0;
        } else {
            gen[i][j] = original[i][j];
            // println!("fuck");
        }
    }
}
```

仓库地址：<https://github.com/OrientalHorizon/Edge-Detection>

Laplace 效果图：![Laplace](https://github.com/OrientalHorizon/Edge-Detection/blob/main/output/gen-lap.jpg)

Sobel 效果图：![Sobel](https://github.com/OrientalHorizon/Edge-Detection/blob/main/output/gen-sobel.jpg)