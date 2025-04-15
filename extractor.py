import subprocess
import sys

# 라이브러리 자동 설치
def install(package):
    subprocess.check_call([sys.executable, "-m", "pip", "install", package])

try:
    from PIL import Image
except ImportError:
    print("📦 Pillow 라이브러리 설치 중...")
    install("Pillow")
    from PIL import Image

try:
    import requests
except ImportError:
    print("📦 requests 라이브러리 설치 중...")
    install("requests")
    import requests

from io import BytesIO
from pathlib import Path

URL_FILE = input("📂 txt 파일 경로를 입력하세요 (예: urls.txt): ").strip()
IMG_DIR = Path(f"dlImages_{URL_FILE}")
IMG_DIR.mkdir(exist_ok=True)

image_files = []

# 이미지 다운로드
with open(URL_FILE, "r") as f:
    urls = [line.strip() for line in f if line.strip()]

print(f"URL 읽어들이기 완료")
print(f"🔗 총 {len(urls)} 개의 이미지 다운로드를 시작합니다...")

if any(IMG_DIR.iterdir()):
    print(f"⚠️ 대상 폴더가 비어있지 않습니다. 다운로드를 건너뜁니다. 경로: {IMG_DIR}")
    for i, url in enumerate(urls, 1):
        filename = IMG_DIR / f"page_{i:03}.png"
        if filename.exists():
            image_files.append(filename)
            print(f"✅ {filename.name} 이미 존재하여 목록에 추가됨")
else:
    for i, url in enumerate(urls, 1):
        filename = IMG_DIR / f"page_{i:03}.png"
        try:
            response = requests.get(url)
            response.raise_for_status()
            with open(filename, "wb") as f:
                f.write(response.content)
            image_files.append(filename)
            print(f"✅ {filename.name} 다운로드 완료")
        except Exception as e:
            print(f"❌ {filename.name} 다운로드 실패!\n오류: {e}")

# PDF 생성
print("\n📄 PDF로 변환 중...")
images = []
for file in image_files:
    try:
        img = Image.open(file).convert("RGB")
        images.append(img)
        print(f"✅ {file.name} Append 성공")
    except Exception as e:
        print(f"⚠️ {file.name} 열기 실패!\n오류: {e}")

if images:
    pdf_path = f"{URL_FILE}_output.pdf"
    images[0].save(pdf_path, save_all=True, append_images=images[1:])
    print(f"\n✅ PDF 저장 완료: {pdf_path}")
else:
    print("❌ PDF로 저장할 이미지가 없습니다.")

print(f"\n📂 다운로드된 이미지는 자동으로 삭제되지 않습니다.\n\t삭제를 원하신다면 다음 폴더를 지우세요. 경로: {IMG_DIR}")

