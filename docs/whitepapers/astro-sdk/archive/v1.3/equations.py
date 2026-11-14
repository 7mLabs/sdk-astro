# One formula inventory for LaTeX math and editable Word Office Math.
from docx.oxml import OxmlElement

def run(text):
 r=OxmlElement('m:r');t=OxmlElement('m:t');t.text=text;t.set('{http://www.w3.org/XML/1998/namespace}space','preserve');r.append(t);return r

def frac(a,b):
 f=OxmlElement('m:f')
 for name,txt in [('num',a),('den',b)]:
  e=OxmlElement('m:'+name);e.append(run(txt));f.append(e)
 return f

EQUATIONS={
 'ROUNDTRIP':(1,r'D(E(x))\equiv x\qquad\text{với mọi }x\in X.', ['D(E(x)) ≡ x     với mọi x ∈ X.']),
 'INJECTIVE':(2,r'x\equiv D(E(x))=D(E(y))\equiv y.', ['x ≡ D(E(x)) = D(E(y)) ≡ y.']),
 'REDUCTION':(3,r'\Delta B=B-C,\qquad R=100\frac{B-C}{B}=100\left(1-\frac{C}{B}\right).', ['ΔB = B − C,     R = 100 × ',('B − C','B'),' = 100 × (1 − ',('C','B'),').']),
 'WEIGHTED':(4,r'R_{\mathrm{t\text{ổ}ng}}=100\frac{\sum_{i=1}^{n}(B_i-C_i)}{\sum_{i=1}^{n}B_i}=\sum_{i=1}^{n}w_iR_i,\qquad w_i=\frac{B_i}{\sum_{j=1}^{n}B_j}.', ['R_tổng = 100 × ',('Σᵢ₌₁ⁿ (Bᵢ − Cᵢ)','Σᵢ₌₁ⁿ Bᵢ'),' = Σᵢ₌₁ⁿ wᵢRᵢ,     wᵢ = ',('Bᵢ','Σⱼ₌₁ⁿ Bⱼ'),'.']),
 'WEIGHTED_PROOF':(5,r'\sum_{i=1}^{n}w_iR_i=\sum_{i=1}^{n}\frac{B_i}{\sum_{j=1}^{n}B_j}\,100\frac{B_i-C_i}{B_i}=100\frac{\sum_{i=1}^{n}(B_i-C_i)}{\sum_{j=1}^{n}B_j}.', ['Σᵢ₌₁ⁿ wᵢRᵢ = Σᵢ₌₁ⁿ ',('Bᵢ','Σⱼ₌₁ⁿ Bⱼ'),' × 100 × ',('Bᵢ − Cᵢ','Bᵢ'),' = 100 × ',('Σᵢ₌₁ⁿ (Bᵢ − Cᵢ)','Σⱼ₌₁ⁿ Bⱼ'),'.']),
 'ANGLE':(6,r'\delta=|a-b|\bmod 360^\circ,\qquad d(a,b)=\min\!\left(\delta,360^\circ-\delta\right).', ['δ = |a − b| mod 360°,     d(a, b) = min(δ, 360° − δ).'])
}
# Use \text for a Vietnamese label in math so the text font supplies every accent.
EQUATIONS['WEIGHTED']=(4,EQUATIONS['WEIGHTED'][1].replace(r'\mathrm{t\text{ổ}ng}',r'\text{tổng}'),EQUATIONS['WEIGHTED'][2])
def word_equation(key,p):
 number,_,parts=EQUATIONS[key];m=OxmlElement('m:oMath')
 for part in parts: m.append(frac(*part) if isinstance(part,tuple) else run(part))
 p._p.append(m);p.add_run('     ('+str(number)+')')

# Structured Office Math: indices and summation bounds remain editable components.
def sub(base,index):return ('sub',base,index)
def f(n,d):return ('frac',n,d)
def sm(index,body):return ('sum',index,'n',body)
Bi=sub('B','i');Ci=sub('C','i');Bj=sub('B','j');wi=sub('w','i');Ri=sub('R','i')
Si=sm('i',Bi);Sj=sm('j',Bj);Sdiff=sm('i',['(',Bi,' − ',Ci,')']);prod=[wi,Ri]
parts4=[sub('R',('text','tổng')),' = 100 × ',f(Sdiff,Si),' = ',sm('i',prod),',     ',wi,' = ',f(Bi,Sj),'.']
parts5=[sm('i',prod),' = ',sm('i',[f(Bi,Sj),' × 100 × ',f([Bi,' − ',Ci],Bi)]),' = 100 × ',f(Sdiff,Sj),'.']
for key,parts in [('WEIGHTED',parts4),('WEIGHTED_PROOF',parts5)]:
 num,latex,_=EQUATIONS[key];EQUATIONS[key]=(num,latex,parts)

def append_math(parent,value):
 if isinstance(value,str):parent.append(run(value));return
 if isinstance(value,list):
  for v in value:append_math(parent,v)
  return
 kind=value[0]
 if kind=='text':
  r=run(value[1]);pr=OxmlElement('m:rPr');pr.append(OxmlElement('m:nor'));r.insert(0,pr);parent.append(r);return
 if kind=='sub':
  node=OxmlElement('m:sSub')
  for tag,v in [('e',value[1]),('sub',value[2])]:
   el=OxmlElement('m:'+tag);append_math(el,v);node.append(el)
 elif kind=='sum':
  node=OxmlElement('m:nary');pr=OxmlElement('m:naryPr')
  from docx.oxml.ns import qn
  for tag,v in [('chr','∑'),('limLoc','undOvr')]:
   el=OxmlElement('m:'+tag);el.set(qn('m:val'),v);pr.append(el)
  node.append(pr)
  for tag,v in [('sub',value[1]+' = 1'),('sup',value[2]),('e',value[3])]:
   el=OxmlElement('m:'+tag);append_math(el,v);node.append(el)
 else:
  # Original two-element tuples are also fractions.
  a,b=value[1:] if kind=='frac' else value
  node=OxmlElement('m:f')
  for tag,v in [('num',a),('den',b)]:
   el=OxmlElement('m:'+tag);append_math(el,v);node.append(el)
 parent.append(node)

def word_equation(key,p):
 number,_,parts=EQUATIONS[key];m=OxmlElement('m:oMath');append_math(m,parts)
 p._p.append(m);p.add_run('     ('+str(number)+')')
