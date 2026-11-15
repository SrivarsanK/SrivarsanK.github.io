import { profile, projects, socials, ageOn, studyLabel } from '../data/portfolio.js';

// A plain-Markdown summary for AI agents (llmstxt.org), built from the same data as the pages so it never drifts from them.
export function GET({ site }) {
  const url = (path) => new URL(path, site).href;
  const now = new Date();
  const lines = [
    '# ' + profile.name,
    '',
    '> ' + profile.description,
    '',
    `${profile.name} is ${ageOn(now)} and studies ${profile.course} at SRM Institute of Science and Technology, Ramapuram, Chennai (${profile.batch.join('–')} batch, ${studyLabel(now)}). Interested in compiler design and developer tools; open to internships and open-source work.`,
    '',
    '## Projects',
    '',
    ...projects.map((p) => `- [${p.name}](${url('/projects/' + p.slug + '/')}): ${p.summary} ${p.context}. Stack: ${p.stack.join(', ')}.${p.private ? '' : ' Source: ' + p.repository}`),
    '',
    '## Pages',
    '',
    `- [About](${url('/about.html')}): background, studies, and the internships Srivarsan is looking for`,
    `- [Home](${url('/')}): every project ranked by GitHub stars, skills, and a contact form`,
    '',
    '## Contact',
    '',
    ...socials.map(([label, href]) => `- [${label}](${href})`),
  ];
  return new Response(lines.join('\n') + '\n', { headers: { 'Content-Type': 'text/markdown; charset=utf-8' } });
}
