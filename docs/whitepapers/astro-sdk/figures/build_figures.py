from PIL import Image, ImageDraw, ImageFont
from pathlib import Path
import math, html
P=Path(__file__).parent
S=3
FONT='/System/Library/Fonts/Supplemental/Arial.ttf'
BOLD='/System/Library/Fonts/Supplemental/Arial Bold.ttf'
class Figure:
 def __init__(self,h):
  self.h=h;self.im=Image.new('RGB',(1200*S,h*S),'white');self.d=ImageDraw.Draw(self.im);self.svg=[]
 def box(self,x,y,w,h,text,fill='#F0F4F7',size=23):
  self.d.rounded_rectangle((x*S,y*S,(x+w)*S,(y+h)*S),radius=10*S,fill=fill,outline='#40576A',width=2*S)
  self.svg.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="10" fill="{fill}" stroke="#40576A" stroke-width="2"/>')
  self.text(x+w/2,y+h/2,text,size)
 def text(self,x,y,text,size=23,bold=False):
  lines=text.split('\n');step=size*1.35
  for i,t in enumerate(lines):
   cy=y+(i-(len(lines)-1)/2)*step
   self.d.text((int(x*S),int(cy*S)),t,font=ImageFont.truetype(BOLD if bold else FONT,int(size*S)),anchor='mm',fill='#172B3A')
   self.svg.append(f'<text x="{x}" y="{cy}" text-anchor="middle" dominant-baseline="central" font-family="Arial, sans-serif" font-size="{size}" font-weight="{700 if bold else 400}" fill="#172B3A">{html.escape(t)}</text>')
 def arrow(self,x,y,xx,yy):
  self.d.line((x*S,y*S,xx*S,yy*S),fill='#40576A',width=3*S)
  a=math.atan2(yy-y,xx-x);pts=[(xx,yy),(xx-12*math.cos(a-.5),yy-12*math.sin(a-.5)),(xx-12*math.cos(a+.5),yy-12*math.sin(a+.5))]
  self.d.polygon([(int(a*S),int(b*S)) for a,b in pts],fill='#40576A')
  self.svg.append(f'<path d="M{x},{y} L{xx},{yy}" stroke="#40576A" stroke-width="3"/><polygon points="'+ ' '.join(f'{a},{b}' for a,b in pts)+'" fill="#40576A"/>')
 def save(self,name):
  self.im.save(P/(name+'.png'),dpi=(576,576))
  (P/(name+'.svg')).write_text(f'<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="{self.h}" viewBox="0 0 1200 {self.h}">'+''.join(self.svg)+'</svg>')
f=Figure(570)
f.text(600,26,'Nhiều ngôn ngữ ứng dụng, một lõi tính toán dùng chung',27,True)
for x,label in [(30,'JavaScript / TypeScript\nqua Node-API'),(420,'Python\nqua ctypes và C ABI'),(810,'.NET\nqua P/Invoke và C ABI')]:
 f.box(x,65,360,86,label);f.arrow(x+180,151,x+180,208)
f.box(30,210,1140,92,'Lõi Rust\nKiểm tra yêu cầu • tổ chức dữ kiện • tạo báo cáo • nén / giải nén',size=25)
f.arrow(270,302,270,360);f.arrow(930,302,930,360)
f.box(30,362,480,100,'Xử lý thời gian\nMúi giờ → UTC → thang thời gian tính',size=23)
f.box(690,362,480,100,'Provider Swiss Ephemeris\nTính vị trí thiên thể và nhà',size=23)
f.text(600,515,'Ứng dụng nhận kết quả qua chính giao diện đã gọi.\nCác hộp mô tả trách nhiệm; mũi tên chỉ hướng gọi / sử dụng.',22)
f.save('01-kien-truc')
f=Figure(555)
f.text(600,25,'Ví dụ luồng tính một lá số natal',27,True)
for x,label in [(25,'1. Nhận đầu vào\nNgày giờ, nơi sinh,\nmúi giờ, tùy chọn'),(430,'2. Kiểm tra thời gian\nXác định đúng thời điểm;\ntừ chối giờ chưa rõ'),(835,'3. Tính thiên văn\nVị trí và nhà;\nkiểm tra lỗi / cờ trả về')]: f.box(x,65,340,115,label,size=23)
f.arrow(365,122,430,122);f.arrow(770,122,835,122)
f.arrow(1005,180,1005,270)
f.box(835,272,340,105,'4. Tổ chức kết quả\nDữ kiện, phiên bản,\ncảnh báo và lỗi',size=23)
f.arrow(835,324,770,324)
f.box(430,272,340,105,'5. Chọn cách chuyển\nJSON nguồn hoặc\ncontext theo nhu cầu',size=23)
f.arrow(430,324,365,324)
f.box(25,272,340,105,'6. Ứng dụng sử dụng\nHiển thị, vẽ biểu đồ\nhoặc xử lý tiếp',size=23)
f.text(600,435,'compact: thiết kế giữ toàn bộ JSON; cần kiểm tra khôi phục.\nfocused / budgeted: chọn phạm vi → công bố phần bị bỏ.',24)
f.text(600,515,'Khi có lỗi, kiểm tra trạng thái trả về trước khi dùng dữ liệu.',23,True)
f.save('02-luong-xu-ly')
f=Figure(390)
f.text(600,25,'Mức giảm dung lượng là phần đã tiết kiệm so với ban đầu',26,True)
f.box(25,72,360,93,'B: byte nguồn\n20.014.085',size=26)
f.text(410,118,'−',36)
f.box(440,72,360,93,'C: byte sau nén\n11.554.591',size=26)
f.text(825,118,'=',32)
f.box(855,72,320,93,'B − C: byte giảm\n8.459.494',size=26)
f.text(600,215,'R = 100 × (B − C) / B ≈ 42,27',32,True)
f.text(600,275,'Giảm khoảng 42,27%; còn khoảng 57,73% dung lượng nguồn.',24)
f.text(600,338,'C > B → R âm: gói dữ liệu tăng kích thước.\nĐây là byte UTF-8; không phải số token hay phần trăm chi phí AI.',23)
f.save('03-cong-thuc')
