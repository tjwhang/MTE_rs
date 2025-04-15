# MTE_rs
MotherTongue Extractor RuSt. Not 뭉탱이.

## 개요
마더텅 서버에 저장된 각 교재 페이지의 링크를 나열한 txt 파일을 입력하여 pdf를 만들어내는 프로그램

## 사용 방법
1. `console.js`를 통해 페이지 이미지 링크 enumerate
2. Python: `run.bat` 실행, Rust: `run.exe` 실행 (단, 파이썬의 경우 파이썬이 당연히 설치되어 있어야 하며, 러스트의 경우 pdfium.dll이 PATH 또는 실행 디렉터리에 필요)
3. txt 파일 입력

## 기능
별거 없음
txt 파일 입력받아서 링크 타고 이미지 다운로드 받은 후 메모리에다 때려넣어놓고 PDF로 합치는 간단한 프로그램

## 변경사항
- 0.1.0: 초기 `extractor.py`, `console.js`
- 0.1.1: 마이너한 오류 수정, 서버가 요청에 응답 없을 시 재시도 요청 보내도록 추가
- 0.1.2: Rust로 코드 재작성, 속도 향상
- 0.1.3: 병렬 다운로드로 속도 대폭 향상
- 0.1.4: PDF crate를 `printpdf`에서 `pdfium-render`로 변경, 코드 감축 및 속도 증가
- 0.1.5: 병렬 다운로드로 PDF 페이지 순서가 섞이던 문제 수정
