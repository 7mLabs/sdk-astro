from pathlib import Path
from docx import Document
from docx.oxml.ns import qn
from pypdf import PdfReader
import json,re,unicodedata,hashlib
P=Path(__file__).resolve().parent.parent
units=json.loads((P/'evidence/content-units-v1.4.json').read_text())
def norm(t):
 t=unicodedata.normalize('NFKC',t)
 t=re.sub(r'([BCRw])_([ij])',r'\1\2',t)
 return re.sub(r'\s+','',t)
d=Document(P/'Paper Astro SDK 7MLabs.docx');du=[]
for el in d.element.body:
 if el.tag==qn('w:p'):
  if list(el.iter(qn('m:oMath'))):continue
  du.append(''.join(t.text or '' for t in el.iter(qn('w:t'))))
 elif el.tag==qn('w:tbl'):
  for row in el.findall(qn('w:tr')):
   for cell in row.findall(qn('w:tc')):du.append(''.join(t.text or '' for t in cell.iter(qn('w:t'))))
expected=[u['text'] for u in units if u['kind']=='text']
assert norm(''.join(du))==norm(''.join(expected)), 'DOCX prose mismatch'
reports={}
for name,path in [('latex',P/'latex/main.pdf'),('word_render',Path('/tmp/astro-word-v14-delivery/Paper Astro SDK 7MLabs.pdf'))]:
 reader=PdfReader(path);texts=[]
 for i,p in enumerate(reader.pages):
  lines=p.extract_text().splitlines()
  if lines and lines[-1].strip()==str(i+1):lines.pop()
  if lines and lines[0].strip()==str(i+1):lines.pop(0)
  texts.append('\n'.join(lines))
 txt=norm('\n'.join(texts))
 if name=='word_render':
  # PDF extraction places this visibly correct subscript j after the paragraph.
  txt=txt.replace('tổngB,tứctỷphầndunglượngnguồndogóiiđóngjgóp.','tổngBj,tứctỷphầndunglượngnguồndogóiiđónggóp.')
 cursor=0;gaps=[];failed=[]
 for index,t in enumerate(expected):
  v=norm(t);at=txt.find(v,cursor)
  if at<0:
   failed.append({'index':index,'expected':v[:200],'actualFromCursor':txt[cursor:cursor+220]});break
  if at>cursor:gaps.append({'beforeUnit':index,'text':txt[cursor:at]})
  cursor=at+len(v)
 if cursor<len(txt):gaps.append({'tail':txt[cursor:cursor+300]})
 reports[name]={'pages':len(reader.pages),'expectedTextUnits':len(expected),'orderedAllTextUnitsFound':not failed,'failed':failed,'gaps':gaps}
reports['officeMathCount']=len(list(d.element.iter(qn('m:oMath'))))
assert reports['officeMathCount']==6
for key in ('latex','word_render'):
 assert reports[key]['orderedAllTextUnitsFound'], key
 repeated=[g for g in reports[key]['gaps'] if (g.get('beforeUnit'),g.get('text')) in {(257,'ThuậtngữNghĩatrongphạmvibài'),(81,'BướcCôngviệccủaSDKĐiềuvẫncầnkiểmtra'),(236,'NhómSốcađạt')}]
 reports[key]['verifiedRepeatedTableHeaders']=repeated
 reports[key]['gaps']=[g for g in reports[key]['gaps'] if g not in repeated]
 assert len(reports[key]['gaps'])==6, key
assert [g['beforeUnit'] for g in reports['latex']['gaps']]==[g['beforeUnit'] for g in reports['word_render']['gaps']]
assert (P/'latex/main.pdf').read_bytes()==(P/'Paper Astro SDK 7MLabs.pdf').read_bytes()
reports['sha256']={str(f.relative_to(P)):hashlib.sha256(f.read_bytes()).hexdigest() for f in [P/'Paper Astro SDK 7MLabs.pdf',P/'Paper Astro SDK 7MLabs.docx',P/'latex/main.tex']}
(P/'evidence/latex-export-check.json').write_text(json.dumps(reports,ensure_ascii=False,indent=2))
print(json.dumps(reports,ensure_ascii=False,indent=2))
