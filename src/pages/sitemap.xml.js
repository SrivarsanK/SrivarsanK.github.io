import { projects } from '../data/portfolio.js';

export function GET({ site }) {
  const paths = ['/', '/about.html', ...projects.map(({ slug }) => '/projects/' + slug + '/')];
  const locations = paths.map((path) => '<url><loc>' + new URL(path, site).href + '</loc></url>');
  return new Response('<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">' + locations.join('') + '</urlset>', {
    headers: { 'Content-Type': 'application/xml; charset=utf-8' },
  });
}
