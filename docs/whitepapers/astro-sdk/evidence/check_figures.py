from pathlib import Path
from zipfile import ZipFile
from pypdf import PdfReader
from PIL import Image,ImageChops,ImageStat
import io,json,hashlib
P=Path(__file__).resolve().parent.parent
sources=[P/'figures'/n for n in ['01-kien-truc.png','03-cong-thuc.png','02-luong-xu-ly.png']]
with ZipFile(P/'Paper Astro SDK 7MLabs.docx') as z:
 media=[z.read(n) for n in z.namelist() if n.startswith('word/media/')]
 assert len(media)==3
 for f in sources: assert f.read_bytes() in media
pdf=PdfReader(P/'Paper Astro SDK 7MLabs.pdf')
images=[]
for i,p in enumerate(pdf.pages):
 for operands,operator in p.get_contents().operations:
  if operator==b'Do' and p['/Resources']['/XObject'][operands[0]].get('/Subtype')=='/Image':
   images.append((i+1,p.images[operands[0]].image.convert('RGB')))
assert len(images)==3
result=[]
for f,(page,im) in zip(sources,images):
 source=Image.open(f).convert('RGB').resize(im.size)
 error=sum(ImageStat.Stat(ImageChops.difference(source,im)).mean)/3
 assert error < 5,(f,error)
 result.append({'figure':f.name,'pdfPage':page,'pdfSize':im.size,'meanPixelDifference':error,'sourceSha256':hashlib.sha256(f.read_bytes()).hexdigest()})
r=json.loads((P/'evidence/review-v1.1/results.json').read_text())
assert r['totalInputBytes']==20014085 and r['totalOutputBytes']==11554591
assert r['totalInputBytes']-r['totalOutputBytes']==8459494
assert round(100*r['totalOutputBytes']/r['totalInputBytes'],2)==57.73
(P/'evidence/figure-check.json').write_text(json.dumps(result,ensure_ascii=False,indent=2))
print(json.dumps(result,ensure_ascii=False,indent=2))
