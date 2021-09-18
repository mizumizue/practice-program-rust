use std::thread;
use std::sync::{Arc};
use std::any::Any;

fn main() {
    manipulate_str()
}

fn manipulate_str() {
    let target = String::from("hoge/foo/bar");

    // 区切り文字で分割する
    for item in target.split("/").into_iter() {
        println!("{}", item.to_string())
    }

    // 最後のItemを取り出す
    println!("{}", target.split("/").last().unwrap())
}

fn parallel_download_image(urls: Vec<String>, save_dir: String) -> Result<(), Box<dyn Any + Send + 'static>> {
    std::fs::create_dir_all(save_dir.clone()).unwrap();

    let mut handles = vec![];
    for url in urls {
        let dir = Arc::new(save_dir.clone());
        handles.push(thread::spawn(move || {
            let url2 = url.clone();
            let file_name = url2.split("/").last().unwrap();
            download_image(url, format!("{}/{}", dir.as_str().to_string(), file_name))
        }));
    }
    for h in handles {
        let _ = h.join()?;
    }
    Ok(())
}

fn download_image(url: String, save_path: String) -> Result<Vec<i32>, String> {
    let b = reqwest::blocking::get(url.as_str())
        .unwrap()
        .bytes()
        .unwrap();

    image::load_from_memory(&b)
        .unwrap()
        .save(save_path)
        .unwrap();

    Ok(res)
}
