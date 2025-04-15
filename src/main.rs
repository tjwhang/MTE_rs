use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
    process::Command
};
use image::GenericImageView;
use reqwest::blocking::get;
//use image::{GenericImageView, ImageReader};
use pdfium_render::prelude::*;
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    display_title_screen();


    print!("📂 URL 목록 txt 파일 경로를 입력하세요 (예: urls.txt): ");
    io::Write::flush(&mut io::stdout())?;

    let mut url_file = String::new();
    io::stdin().read_line(&mut url_file)?;

    let url_file = url_file.trim();
    let img_dir_name = format!("dlImages_{}", url_file);
    let img_dir = Path::new(&img_dir_name);

    fs::create_dir_all(&img_dir)?;

    // URL 읽기
    println!("🔗 URL 읽어들이는 중...");
    let urls: Vec<String> = io::BufReader::new(File::open(url_file)?)
        .lines()
        .filter_map(Result::ok)
        .collect();

    println!("⬇️ 총 {} 개의 이미지 다운로드를 시작합니다...", urls.len());

    let mut images = Vec::new();

    if img_dir.read_dir()?.next().is_some() {
        println!("🗂️ 대상 폴더가 비어 있지 않습니다. 다운로드 단계를 건너뜁니다.");
        for entry in img_dir.read_dir()? {
            let entry = entry?;
            if entry.path().is_file() {
                images.push(entry.path());
            }
        }
    } else {
        // for (i, url) in urls.iter().enumerate() {
        //     let filename = img_dir.join(format!("page_{:03}.png", i + 1));
        //     let mut retries = 3;
        //     let mut success = false;

        //     while retries > 0 && !success {
        //         match get(url) {
        //             Ok(response) => match response.bytes() {
        //                 Ok(bytes) => {
        //                     fs::write(&filename, &bytes)?;
        //                     println!("✅ {} 다운로드 완료", filename.display());
        //                     success = true;
        //                 }
        //                 Err(e) => {
        //                     println!("❌ 바이트 읽기 실패: {}. 재시도 중... (남은 시도: {})", e, retries - 1);
        //                 }
        //             },
        //             Err(e) => {
        //                 println!("❌ 다운로드 실패: {}. 재시도 중... (남은 시도: {})", e, retries - 1);
        //             }
        //         }
        //         retries -= 1;
        //     }

        //     if !success {
        //         println!("❌ {} 다운로드 실패. 건너뜁니다.", url);
        //     }
        //     images.push(filename);
        // }

        // 병렬 다운로드
        println!("❕ 빠른 다운로드를 위해 병렬적으로 작업이 수행됩니다.\n\t로그가 순차적으로 출력되지 않거나 누락될 수 있습니다.\n작업 자체는 정상적으로 처리되었으니 걱정 마시기 바랍니다.\n");
        images = urls.par_iter()
            .enumerate()
            .map(|(i, url)| {
                let filename = img_dir.join(format!("page_{:03}.png", i + 1));
                let mut retries = 3;
                let mut success = false;

                while retries > 0 && !success {
                    match get(url) {
                        Ok(response) => match response.bytes() {
                            Ok(bytes) => {
                                if let Err(e) = fs::write(&filename, &bytes) {
                                    println!("❌ 파일 저장 실패: {}", e);
                                } else {
                                    println!("✅ {} 다운로드 완료", filename.display());
                                    success = true;
                                }
                            }
                            Err(e) => {
                                println!("❌ 바이트 읽기 실패: {}. 재시도 중... (남은 시도: {})", e, retries - 1);
                            }
                        },
                        Err(e) => {
                            println!("❌ 요청에 응답하지 않음: {}. 재시도 중... (남은 시도: {})", e, retries - 1);
                        }
                    }
                    retries -= 1;
                }

                if !success {
                    println!("❌ {} 다운로드 실패. 건너뜁니다.", url);
                }

                filename
            })
            .collect();
    }

    images.sort(); 
    println!("📜 파일 벡터 정렬 완료");
    println!("\t정렬 목록(vec): {:?}", &images);

    // Pdfium 초기화
    println!("📜 pdfium.dll 초기화");
    let pdfium = Pdfium::default();
    let mut doc = pdfium.create_new_pdf()?;
    println!("💭 메모리에 PDF 문서 생성 완료");

    for file in &images {
        // let img = ImageReader::open(file)?.decode()?;
    
        let img = image::open(file)?;
        let (width, height) = img.dimensions();

        let page_width = PdfPagePaperSize::a4().width().value; // Points
        let page_height = PdfPagePaperSize::a4().height().value; // Points

        let scale_x = page_width / width as f32;
        let scale_y = page_height / height as f32;
        let scale = scale_x.min(scale_y); // Use the smaller scale to maintain aspect ratio

        let scaled_width = (width as f32) * scale;
        let scaled_height = (height as f32) * scale;
        let offset_x = (page_width - scaled_width) / 2.0; // Center horizontally
        let offset_y = (page_height - scaled_height) / 2.0; // Center vertically

    let mut page = doc.pages_mut().create_page_at_end(PdfPagePaperSize::a4())?;

    let mut image_object = PdfPageImageObject::new_with_width(
        &doc,
        &img,
        PdfPoints::new(scaled_width),
    )?;

    image_object.translate(PdfPoints::new(offset_x), PdfPoints::new(offset_y))?;

    page.objects_mut().add_image_object(image_object)?;

        println!("✅ {} Append 완료", file.display());
    }

    // PDF 저장
    println!("🛞 PDF 병합 및 저장 중...\n\t잠시만 기다려 주세요.");
    let pdf_path = format!("{}_output.pdf", url_file);
    doc.save_to_file(&pdf_path)?;
    println!("\n✅ PDF 저장 완료\n\t파일명: {}", pdf_path);
    println!("\t🖼️ 이미지를 저장한 임시 폴더는 자동 삭제되지 않습니다. 삭제를 원한다면 다음 폴더를 삭제하세요: {}", img_dir.display());

    println!("\n👍 작업 완료! 아무 키나 눌러 종료하세요...");
    Command::new("cmd.exe").arg("/c").arg("pause").status()?;

    Ok(())
}

fn display_title_screen() {
    let width = 65; 
    let center = |text: &str| format!("{:^width$}", text, width = width);

    println!("{}", center(r#"
 
        ░▒▓██████████████▓▒░▒▓████████▓▒░▒▓████████▓▒░ 
        ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓█▓▒░        
        ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓█▓▒░        
        ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓██████▓▒░   
        ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓█▓▒░        
        ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓█▓▒░        
        ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓████████▓▒░ 
    "#));
    
    println!("{}", center("M.T.E (Mother Tongue Exporter)"));
    println!("{}", center("만든 놈: tjwhang"));
    println!();
    println!("{}", center("NOT 뭉탱이"));
    println!("{}", center("---------------------------------------------------------------"));
    println!();
}