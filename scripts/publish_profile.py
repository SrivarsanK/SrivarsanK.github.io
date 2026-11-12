"""Publish a readable profile and metadata alongside the interactive portfolio."""
import json
import re
import sys
from html import escape
from pathlib import Path

TITLE = "Srivarsan K | AI & Full-Stack Developer"
DESCRIPTION = "Srivarsan K (K Srivarsan), AI and full-stack developer at SRMIST Ramapuram, Chennai. Building Glide. Find projects, GitHub and the official LinkedIn profile."
URL = "https://srivarsank.github.io/"
PERSON = {
    "@type": "Person",
    "@id": URL + "#person",
    "name": "Srivarsan K",
    "alternateName": ["K Srivarsan", "K. Srivarsan"],
    "url": URL + "about.html",
    "description": DESCRIPTION,
    "sameAs": ["https://www.linkedin.com/in/ksrivarsan/", "https://github.com/SrivarsanK"],
}

def publish(directory):
    directory = Path(directory)
    index = directory / "index.html"
    source = index.read_text(encoding="utf-8")
    if "</head>" not in source or "</body>" not in source:
        raise ValueError("Expected a complete built index.html")
    source = re.sub(r"<title\b[^>]*>.*?</title>", "", source, flags=re.I | re.S)
    source = re.sub(r"""<meta\b(?=[^>]*\bname=["']description["'])[^>]*>""", "", source, flags=re.I)
    source = re.sub(r"""<link\b(?=[^>]*\brel=["']canonical["'])[^>]*>""", "", source, flags=re.I)
    # Remove only this script's previous additions so reruns remain safe.
    source = re.sub(r"<!-- profile-seo:start -->.*?<!-- profile-seo:end -->", "", source, flags=re.S)
    head = ('<title>' + escape(TITLE) + '</title>\n'
            '<meta name="description" content="' + escape(DESCRIPTION, quote=True) + '">\n'
            '<link rel="canonical" href="' + URL + '">\n')
    source = source.replace("</head>", head + "</head>", 1)
    link = ('<!-- profile-seo:start --><a href="/about.html" '
            'style="position:fixed;right:12px;top:12px;z-index:9999;padding:8px 12px;'
            'border:1px solid #64748b;border-radius:8px;background:#0f172a;color:#fff;'
            'font:14px system-ui;text-decoration:underline">About Srivarsan K &amp; LinkedIn</a>'
            '<!-- profile-seo:end -->')
    source = source.replace("</body>", link + "</body>", 1)
    index.write_text(source, encoding="utf-8")
    schema = {"@context": "https://schema.org", "@type": "ProfilePage",
              "url": URL + "about.html", "mainEntity": PERSON}
    profile = """<!doctype html>
<html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>About Srivarsan K | LinkedIn, GitHub &amp; Projects</title>
<meta name="description" content="Meet Srivarsan K (K Srivarsan), AI and full-stack developer at SRMIST Ramapuram. Official LinkedIn, GitHub and selected projects.">
<link rel="canonical" href="https://srivarsank.github.io/about.html">
<style>
body{margin:0;background:#0b1120;color:#e2e8f0;font:18px/1.7 system-ui,sans-serif}
main{max-width:760px;margin:auto;padding:64px 24px}
h1{font-size:clamp(2.5rem,8vw,4rem);line-height:1.1;margin:20px 0}
h2{font-size:1.35rem;margin-top:36px}a{color:#93c5fd;text-underline-offset:4px}
nav{display:flex;flex-wrap:wrap;gap:12px 24px}
.subtitle{color:#94a3b8}li{margin:12px 0}
</style>
<script type="application/ld+json">SCHEMA</script></head>
<body><main>
<a href="/">Open interactive portfolio</a>
<h1>Srivarsan K</h1>
<p class="subtitle">AI &amp; Full-Stack Developer · Chennai, India</p>
<p>I'm Srivarsan K, also known as K Srivarsan. I study Artificial Intelligence at
SRM Institute of Science and Technology, Ramapuram, and build developer tools and AI applications.</p>
<nav aria-label="Official profiles">
<a href="https://www.linkedin.com/in/ksrivarsan/" rel="me">Srivarsan K on LinkedIn</a>
<a href="https://github.com/SrivarsanK" rel="me">Srivarsan K on GitHub</a>
</nav>
<h2>Projects</h2>
<ul>
<li><a href="https://github.com/SrivarsanK/Glide">Glide</a> — a local visual editor that writes design changes back to source code.</li>
<li><a href="https://github.com/SrivarsanK/aws_a4b">Nexus Solo</a> — an AI-powered creator management platform using coordinated agents.</li>
</ul>
<h2>Community</h2>
<p>Technical Lead at DSC SRMIST Ramapuram, contributing to student developer activities and technical events.</p>
</main></body></html>
"""
    (directory / "about.html").write_text(profile.replace("SCHEMA", json.dumps(schema, ensure_ascii=False)), encoding="utf-8")

if __name__ == "__main__":
    publish(sys.argv[1])
