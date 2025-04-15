(async () => {
    const divs = [...document.querySelectorAll('div[id^="book_img_"]')];
    divs.sort((a, b) => {
      const getId = el => parseInt(el.id.replace('book_img_', ''));
      return getId(a) - getId(b);
    });
  
    const urls = divs.map(div => {
      const img = div.querySelector('img');
      return img ? img.src : null;
    }).filter(Boolean);

    console.log("총 " + urls.length + " 개 이미지 URL 추출 완료");
    console.log(urls.join('\n'));
  
    const blob = new Blob([urls.join('\n')], { type: 'text/plain' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = 'image_urls.txt';
    document.body.appendChild(a);
    a.click();
    a.remove();

    
  
    console.log(`✅ ${urls.length} 개 이미지 URL을 image_urls.txt로 저장했습니다. 위치는 브라우저 기본 다운로드 위치(또는 다른 이름으로 저장 프롬프트)`);
  })();
  