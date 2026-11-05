# -*- coding: utf-8 -*-
"""디시 「각종 모드 대응」 글: 초안 md 의 본문 블록을 에디터 HTML 태그 모드용 한 줄 HTML 로 바꾼다.
  python MIG\_dc_post_html.py <초안 .md> <출력 .txt>
규격 = 메모리 feedback-dc-post-html-format. URL 줄마다 그 글의 og 메타를 새로 읽어 링크 카드를 붙인다.
항목 제목과 카드 제목이 맞는지는 눈으로 볼 수 있게 표로 찍고, URL 중복·카드 주소 불일치는 실패로 끝낸다.
"""
import html, re, sys, time, urllib.request
sys.stdout.reconfigure(encoding='utf-8')
GALL = ' - 팀파이트매니저 마이너 갤러리'
CARD = ('<div class="og-div" contenteditable="false"><a href="{URL}" target="_blank" class="og-wrap" style="display:inline-block;width:70%;height:105px;background:#fff;border:1px solid #dfe1ee;box-sizing:border-box;letter-spacing:-1px;overflow:hidden;line-height:1.5">'
        '<div class="og-img" style="float:left;width:104px;height:100%;box-sizing: border-box;border-right: 1px solid #dfe1ee;" data-img="{IMG}"><img class="ogimg" referrerpolicy="no-referrer" src="{IMG}" alt="" style="width:100%!important;height:100%!important;object-fit: cover;object-position: center center"></div>'
        '<div class="og-info" style="float:right;width:calc(100% - 104px);height:100%;box-sizing:border-box;padding:12px"><div class="og-inr" style="display: inline-block;max-width: 100%;text-overflow: ellipsis;white-space: nowrap;overflow: hidden;font-size: 14px;font-weight:bold">'
        '<strong class="og-tit" style="display: inline-block;max-width: 70%;font-size: 14px;vertical-align: top;padding-right: 5px;white-space: nowrap;overflow: hidden;text-overflow: ellipsis">{TITLE} </strong>'
        '<strong class="og-gallname" style="font-size: 14px;vertical-align: top;margin-left: -3px"> - 팀파이트매니저 마이너 갤러리</strong></div>'
        '<p class="og-txt" style="width:100%;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;color:#000;margin:2px 0 11px">{DESC}</p>'
        '<p class="og-lnk" style="width:100%;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;color:#3b4890">gall.dcinside.com</p></div></a></div>')


def esc(s):
    return s.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;').replace('\xa0', '&nbsp;')


def og(url):
    req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/130.0 Safari/537.36'})
    t = urllib.request.urlopen(req, timeout=20).read().decode('utf-8', 'replace')

    def m(p):
        x = re.search(r'<meta[^>]*property="og:%s"[^>]*content="([^"]*)"' % p, t)
        return html.unescape(x.group(1)) if x else ''
    return m('title'), m('description'), m('image')


src, dst = sys.argv[1], sys.argv[2]
body = open(src, encoding='utf-8').read().split('**본문**')[1].split('```')[1].strip('\n')
lines = body.split('\n')
out, rows, head = [], [], ''
for i, l in enumerate(lines):
    if l.startswith('http'):
        title, desc, img = og(l)
        title = title.replace(GALL, '').strip()
        assert title and img, l
        rows.append((head, title, l))
        out.append('<p>%s</p>' % esc(l))
        out.append(CARD.replace('{URL}', esc(l)).replace('{IMG}', esc(img)).replace('{TITLE}', esc(title)).replace('{DESC}', esc(desc)))
        out.append('<p><br></p>' if i == len(lines) - 1 else '<p><br></p><p><br></p>')
        time.sleep(0.4)
    elif not l:
        if not out[-1].endswith('<p><br></p>'):
            out.append('<p><br></p>')
    else:
        if re.match(r'\d+\. ', l):
            head = l
        out.append('<p>%s</p>' % esc(l))
s = ''.join(out)
assert '\n' not in s
urls = [r[2] for r in rows]
assert len(set(urls)) == len(urls), '같은 URL'
pairs = re.findall(r'<p>(https://[^<]*)</p><div class="og-div" contenteditable="false"><a href="([^"]*)"', s)
assert len(pairs) == len(urls) and all(a == b for a, b in pairs), '카드 주소 불일치'
open(dst, 'w', encoding='utf-8', newline='').write(s)
for h, t, u in rows:
    print('%-34s %-34s no=%s' % (h, t, re.search(r'no=(\d+)', u).group(1)))
print('항목', len(rows), '· 고유 URL', len(set(urls)), '·', len(s.encode('utf-8')), 'B')
