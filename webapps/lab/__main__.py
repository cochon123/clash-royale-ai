"""Run with .venv/bin/python -m webapps.lab --host 127.0.0.1 --port 8801."""
from __future__ import annotations

import argparse
import html
import re
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import quote, unquote, urlsplit

import markdown

from webapps.common import paths
from webapps.common.httpkit import App, HTTPError, Raw
from . import scanners as scan

STATIC = Path(__file__).parent / 'static'
app = App('Clash Lab')


def safe_document(value):
    candidate = (paths.REPO / unquote(value)).resolve()
    roots = (paths.DOCS, paths.ENGINE / 'docs', paths.MODELS)
    if candidate.suffix.lower() not in {'.md', '.html'} or not any(root.resolve() in candidate.parents for root in roots):
        raise HTTPError(404, 'Document outside the Lab document catalog')
    if not candidate.is_file():
        raise HTTPError(404, 'Document not found')
    return candidate


class LocalDocument(HTMLParser):
    """Allow readable document markup, without scripts, styles or remote embeds."""
    allowed = {'p', 'div', 'span', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'pre', 'code', 'blockquote',
               'ul', 'ol', 'li', 'table', 'thead', 'tbody', 'tr', 'td', 'th', 'hr', 'br', 'strong', 'em',
               'del', 'a', 'details', 'summary', 'sup', 'sub', 'img'}
    blocked = {'script', 'style', 'iframe', 'object', 'head', 'svg', 'form'}

    def __init__(self, path):
        super().__init__(convert_charrefs=True)
        self.path, self.output, self.suppressed = path, [], []

    def handle_starttag(self, tag, attrs):
        if tag in self.blocked:
            self.suppressed.append(tag)
        if self.suppressed or tag not in self.allowed:
            return
        values = dict(attrs)
        clean = []
        if tag == 'a' and values.get('href'):
            href = values['href']
            parsed = urlsplit(href)
            if parsed.scheme in {'http', 'https'}:
                clean.extend([('href', href), ('rel', 'noreferrer noopener'), ('target', '_blank')])
            elif not parsed.scheme and not parsed.netloc:
                if href.startswith('#'):
                    clean.append(('href', href))
                else:
                    raw_path = unquote(parsed.path)
                    if raw_path.startswith(scan.OLD_ROOT + '/'):
                        target = Path(scan.mapped(raw_path))
                    elif raw_path.startswith('/'):
                        target = paths.REPO / raw_path.lstrip('/')
                    else:
                        target = (self.path.parent / raw_path).resolve()
                    if target.suffix == '.md':
                        try:
                            rel = paths.relative(safe_document(str(target)))
                            clean.append(('href', '/#docs?doc=' + quote(rel, safe='') + ('&anchor=' + quote(parsed.fragment) if parsed.fragment else '')))
                        except HTTPError:
                            pass
                    elif paths.REPORTS.resolve() in target.parents:
                        clean.append(('href', '/reports/' + quote(str(target.relative_to(paths.REPORTS)), safe='/')))
                    elif paths.MODELS.resolve() in target.parents and target.suffix == '.html':
                        clean.append(('href', '/model-card?path=' + quote(paths.relative(target), safe='')))
        if tag == 'img':
            # Documents can contain remote tracking images or huge local captures.
            self.output.append('<span class="muted">[Image: ' + html.escape(values.get('alt', 'see source document')) + ']</span>')
            return
        if values.get('id'):
            clean.append(('id', values['id']))
        attrs_text = ''.join(' ' + k + '="' + html.escape(v, quote=True) + '"' for k, v in clean)
        self.output.append('<' + tag + attrs_text + '>')

    def handle_endtag(self, tag):
        if self.suppressed:
            if tag == self.suppressed[-1]:
                self.suppressed.pop()
            return
        if tag in self.allowed and tag not in {'img', 'hr', 'br'}:
            self.output.append('</' + tag + '>')

    def handle_data(self, data):
        if not self.suppressed:
            self.output.append(html.escape(data))


def document(path):
    try:
        with path.open('rb') as handle:
            raw = handle.read(2 * 1024 * 1024 + 1)
        if len(raw) > 2 * 1024 * 1024:
            raise HTTPError(413, 'Document exceeds the 2 MiB reader limit')
        content = scan.mapped(raw.decode('utf-8', errors='replace'))
    except OSError:
        raise HTTPError(404, 'Document disappeared while reading')
    if path.suffix.lower() == '.md':
        content = markdown.markdown(content, extensions=['fenced_code', 'tables', 'toc'])
    parser = LocalDocument(path)
    parser.feed(content)
    return {'source': paths.relative(path), 'title': path.stem, 'html': ''.join(parser.output),
            'note': 'Readable local copy. Embedded scripts, styles and images are omitted.'}


@app.route('GET', '/api/health')
def health(request):
    return {'status': 'ok', 'name': 'Clash Lab', 'read_only': True, 'sampled_at': scan.live().get('sampled_at')}


@app.route('GET', '/api/overview')
def overview(request):
    return scan.overview()


@app.route('GET', '/api/models')
def models(request):
    return scan.models()


@app.route('GET', '/api/models/{id}')
def model(request):
    model_id = unquote(request.params['id'])
    data = scan.models()
    for kind in ('playable', 'offline'):
        row = next((r for r in data[kind] if r['id'] == model_id), None)
        if row is not None:
            return {'sampled_at': data['sampled_at'], 'kind': kind, 'model': row}
    raise HTTPError(404, 'Unknown model')


@app.route('GET', '/api/runs')
def runs(request):
    return scan.runs()


@app.route('GET', '/api/runs/{id}')
def run(request):
    # The ID must occur in the shallow catalog; never resolve arbitrary input paths.
    run_id = unquote(request.params['id'])
    if run_id not in {r['id'] for r in scan.cached('run-scan', 5, scan.run_scan)}:
        raise HTTPError(404, 'Unknown run')
    value = scan.run_detail(run_id)
    if value is None:
        raise HTTPError(404, 'Run disappeared')
    return value


@app.route('GET', '/api/engine')
def engine(request):
    return scan.cached('engine', 60, scan.engine_scan)


@app.route('GET', '/api/data')
def data(request):
    return {**scan.cached('data', 300, scan.data_scan),
            'collector': next((s for s in scan.live().get('services', []) if s['name'] == 'Replay collector'), None)}


@app.route('GET', '/api/docs')
def docs(request):
    return scan.cached('docs', 60, scan.docs_scan)


@app.route('GET', '/api/document')
def read_document(request):
    return document(safe_document(request.query.get('path', '')))


@app.route('GET', '/model-card')
def model_card(request):
    doc = document(safe_document(request.query.get('path', '')))
    return Raw(('<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">'
                '<title>' + html.escape(doc['title']) + ' · Clash Lab</title><link rel="stylesheet" href="/static/style.css">'
                '<body><main class="standalone"><a href="/#models">← Models</a><p class="source">' + html.escape(doc['source']) +
                '</p><article class="prose">' + doc['html'] + '</article></main></body></html>').encode(), 'text/html; charset=utf-8')


@app.route('GET', '/reports/{file}')
def offline_report(request):
    path = (paths.REPORTS / unquote(request.params['file'])).resolve()
    if paths.REPORTS.resolve() not in path.parents or not path.is_file():
        raise HTTPError(404, 'Report not found')
    if path.suffix.lower() != '.html':
        return path
    def remote_link(match):
        tag = match.group(0)
        return '' if re.search(r'''\bhref\s*=\s*["'](?:https?:)?//''', tag, re.I) else tag
    content = re.sub(r'<link\b[^>]*>', remote_link, path.read_text(errors='replace'), flags=re.I)
    # Legacy reports contain Google Fonts links. Remove them in memory and keep
    # native inline chart scripts, with no network dependencies or file edits.
    return Raw(content.encode(), 'text/html; charset=utf-8', headers={
        'Content-Security-Policy': "default-src 'self' data: blob:; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; connect-src 'self'; font-src 'self'; object-src 'none'; base-uri 'self'",
        'Referrer-Policy': 'no-referrer'})


for page in ('/', '/overview', '/models', '/runs', '/engine', '/data', '/docs'):
    app.route('GET', page)(lambda request: Raw((STATIC / 'index.html').read_bytes(), 'text/html; charset=utf-8', headers={
        'Content-Security-Policy': "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; object-src 'none'; base-uri 'none'",
        'Referrer-Policy': 'no-referrer'}))

app.mount('/static/', STATIC)
app.mount('/reports/', paths.REPORTS)
app.mount('/cards/', paths.CARD_ART)


def main():
    parser = argparse.ArgumentParser(description='Read-only Clash Royale AI research Lab')
    parser.add_argument('--host', default='127.0.0.1')
    parser.add_argument('--port', type=int, default=8801)
    args = parser.parse_args()
    scan.start_sampler()
    try:
        app.serve(args.host, args.port)
    finally:
        scan.STOP.set()


if __name__ == '__main__':
    main()
