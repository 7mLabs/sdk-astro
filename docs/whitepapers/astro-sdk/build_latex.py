from pathlib import Path
import re,shutil,json
from equations import EQUATIONS
P=Path(__file__).parent;L=P/'latex';(L/'images').mkdir(exist_ok=True)
for f in (P/'figures').glob('*.png'):shutil.copy2(f,L/'images'/f.name)
# Record the ordered content units so exports can be checked independently of pagination.
units=[]
def esc(t):
 table={'\\':r'\textbackslash{}','&':r'\&','%':r'\%','$':r'\$','#':r'\#','_':r'\_','{':r'\{','}':r'\}','~':r'\textasciitilde{}','^':r'\textasciicircum{}'}
 t=''.join(table.get(c,c) for c in t)
 t=re.sub(r'([BCRw])\\_([ij])',r'\(\1_{\2}\)',t)
 # Long code identifiers are allowed to break without changing visible text.
 t=re.sub(r'[a-f0-9]{40}',lambda m:r'\allowbreak{}'.join(m.group()[i:i+10] for i in range(0,40,10)),t)
 return t.replace('≡',r'\(\equiv\)').replace('--',r'-\kern0pt-')

def text(t):
 chunks=re.split(r'(https?://\S+)',t)
 return ''.join(r'\url{'+c+'}' if c.startswith(('https://','http://')) else esc(c).replace('/',r'/\allowbreak{}') for c in chunks)

header=r'''\documentclass[11pt,a4paper]{article}
\usepackage[margin=1in]{geometry}
\usepackage{fontspec}
\setmainfont{texgyretermes-regular.otf}[BoldFont=texgyretermes-bold.otf,ItalicFont=texgyretermes-italic.otf,BoldItalicFont=texgyretermes-bolditalic.otf]
\usepackage{amsmath,amssymb}
\usepackage{graphicx,longtable,array,booktabs}
\usepackage{titlesec,needspace}
\usepackage[hidelinks,unicode]{hyperref}
\usepackage{xurl}
\urlstyle{same}
\setlength{\parindent}{16pt}
\setlength{\parskip}{0pt}
\setlength{\emergencystretch}{3em}
\hyphenpenalty=10000
\exhyphenpenalty=10000
\widowpenalty=10000
\clubpenalty=10000
\titleformat{\section}{\normalfont\fontsize{12}{14}\bfseries}{}{0pt}{}
\titleformat{\subsection}{\normalfont\fontsize{11}{13}\bfseries}{}{0pt}{}
\titlespacing*{\section}{0pt}{12pt}{5pt}
\titlespacing*{\subsection}{0pt}{9pt}{4pt}
\setlength{\LTpre}{5pt}
\setlength{\LTpost}{5pt}
\renewcommand{\arraystretch}{1.18}
\hypersetup{pdftitle={Astro SDK và khả năng bảo toàn dữ liệu thiên văn},pdfauthor={7MLabs},pdfsubject={Bản thảo 1.4 — nghiên cứu trường hợp phần mềm}}
\begin{document}
\fontsize{11}{12.7}\selectfont
'''
out=[header];lines=(P/'whitepaper.md').read_text().splitlines();i=0;inrefs=False
while i<len(lines):
 ln=lines[i].strip();i+=1
 if not ln:continue
 if ln.startswith('{{EQ_'):
  key=ln[5:-2];num,latex,_=EQUATIONS[key]
  out.append(r'\begin{equation}'+latex+r'\label{eq:'+key+r'}\end{equation}')
  units.append({'kind':'equation','id':key,'number':num});continue
 if ln.startswith('!['):
  m=re.fullmatch(r'!\[(.*?)\]\((.*?)\)',ln)
  while i<len(lines) and not lines[i].strip():i+=1
  caption=lines[i].strip();assert caption.startswith('Hình ');i+=1
  out.append(r'\par\medskip\noindent\begin{minipage}{\linewidth}\centering'+'\n'+r'\includegraphics[width=\linewidth]{images/'+Path(m[2]).name+'}\n'+r'\par\smallskip\raggedright{\fontsize{10}{12}\selectfont\itshape '+text(caption)+r'\par}\end{minipage}\par\medskip')
  units.append({'kind':'text','text':caption});continue
 if ln.startswith('|'):
  block=[ln]
  while i<len(lines) and lines[i].startswith('|'):block.append(lines[i]);i+=1
  rows=[[c.strip() for c in row.strip('|').split('|')] for row in block if not re.match(r'^\|[\s|:-]+\|$',row)]
  n=len(rows[0]);numeric=rows[0][0] in ['Mẫu','Trường hợp','Nhóm']
  widths=([.37,.22,.22,.19] if n==4 else ([.48,.22,.30] if numeric else [.21,.39,.40])) if n>2 else ([.78,.22] if numeric else [.29,.71])
  # Table width including padding exactly equals text width.
  spec=''.join(('>{\\raggedleft\\arraybackslash}' if numeric and j>0 else '>{\\raggedright\\arraybackslash}')+'p{\\dimexpr '+str(w)+'\\linewidth-2\\tabcolsep\\relax}' for j,w in enumerate(widths))
  h=' & '.join(r'\textbf{'+text(c)+'}' for c in rows[0])+r' \\ \midrule'
  out.append(r'{\fontsize{10}{12}\selectfont\begin{longtable}{'+spec+'}\n'+r'\toprule '+h+'\n'+r'\endfirsthead\toprule '+h+r'\endhead\bottomrule\endfoot'+'\n')
  out.extend(' & '.join(text(c) for c in row)+r' \\' for row in rows[1:]);out.append(r'\end{longtable}}')
  for row in rows:
   for c in row:units.append({'kind':'text','text':c})
  continue
 units.append({'kind':'text','text':ln.lstrip('# ') if ln.startswith('#') else ln})
 if ln.startswith('# '):out.append(r'\begin{center}{\fontsize{14}{17}\selectfont\bfseries '+text(ln[2:])+r'\par}\end{center}')
 elif ln.startswith('7MLabs ·'):out.append(r'\begin{center}\fontsize{10}{12}\selectfont '+text(ln)+r'\end{center}')
 elif ln.startswith('### '):out.append(r'\subsection*{'+text(ln[4:])+'}')
 elif ln.startswith('## '):
  title=ln[3:];inrefs=title=='Tài liệu tham khảo';out.append(r'\section*{'+text(title)+'}')
 elif re.match(r'^Bảng \d+\.',ln):out.append(r'\Needspace{6\baselineskip}\par\medskip\noindent{\fontsize{10}{12}\selectfont\itshape '+text(ln)+r'\par}\nopagebreak')
 elif inrefs:out.append(r'{\fontsize{10}{11}\selectfont\raggedright\hangindent=18pt\hangafter=1\noindent '+text(ln)+r'\par}\vspace{1pt}')
 elif ln.startswith('Chạy từ thư mục gốc'):out.append(r'{\raggedright '+text(ln)+r'\par}')
 else:out.append(text(ln)+'\n\n')
out.append(r'\end{document}')
(L/'main.tex').write_text('\n'.join(out));(P/'evidence/content-units-v1.4.json').write_text(json.dumps(units,ensure_ascii=False,indent=2))
print('LaTeX source written',len(units),'content units')
