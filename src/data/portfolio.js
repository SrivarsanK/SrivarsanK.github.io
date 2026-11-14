export const profile = {
  name: 'Srivarsan K',
  github: 'https://github.com/SrivarsanK',
  linkedin: 'https://www.linkedin.com/in/ksrivarsan/',
  email: 'srivarsankannan@gmail.com',
  title: 'Open-Source AI & Full-Stack Developer',
  description: 'Srivarsan K is an open-source AI and full-stack developer in Chennai, building developer tools and AI in TypeScript, Python and Rust. Open to internships.',
};

// `started` is the month the repository was created; it orders the path section.
// Each project has a 1200x750 screenshot of it running locally at public/media/projects/<slug>.webp; `shot` is its alt text.
// `stars` is a fallback: the build replaces it with the live count from GitHub.
const catalogue = [
  {
    slug: 'glide',
    stars: 175,
    shot: 'The Glide editor open on a test site, with the layers tree, a live canvas and the properties panel',
    name: 'Glide',
    category: 'Open-source developer tool',
    summary: 'A local-first visual editor that writes changes back to source code.',
    repository: 'https://github.com/SrivarsanK/Glide',
    started: '2026-06',
    context: 'Open source, Apache-2.0',
    stack: ['TypeScript', 'Node.js', 'Vite', 'Babel', 'Recast'],
    sections: [
      ['What it does', 'Glide lets developers edit a running web interface visually. Moving an element or changing its styles updates the original source file, so the edits can be reviewed in Git.'],
      ['How it works', 'A Node.js proxy connects to a Vite development server and links rendered elements to their source locations. AST and CST transformations update the relevant attributes, styles, or classes while preserving nearby formatting and comments.'],
      ['Framework support', 'Glide supports React JSX and TSX, Vue single-file components, Svelte, Astro, and HTML.'],
    ],
  },
  {
    slug: 'ridesuraksha',
    stars: 7,
    shot: 'RideSuraksha’s landing page with the headline “Shield the driven” and a payout velocity of 0.14 seconds',
    name: 'RideSuraksha',
    category: 'Insurtech platform',
    summary: 'Income insurance that pays gig delivery workers automatically when weather stops work.',
    repository: 'https://github.com/SrivarsanK/RideSuraksha',
    live: 'https://ridesuraksha.vercel.app',
    started: '2026-03',
    context: 'Guidewire DEVTrails 2026',
    stack: ['Next.js', 'TypeScript', 'FastAPI', 'Python', 'XGBoost', 'Convex', 'Docker'],
    sections: [
      ['What it does', 'RideSuraksha is a parametric income insurance platform for India’s food delivery workers. When a verified disruption such as heavy rain, severe pollution, or a curfew stops deliveries, the payout starts automatically. There is no claim form and no adjuster.'],
      ['How it works', 'Workers pay a weekly premium priced by the risk of their zone and adjusted with weather forecasts. Before money reaches a worker’s UPI account, fraud checks compare GPS, accelerometer data, nearby workers, and past behaviour. It runs as a progressive web app, so workers don’t need an app store.'],
      ['Where it came from', 'I built RideSuraksha with a team for Guidewire DEVTrails 2026, a university hackathon.'],
    ],
  },
  {
    slug: 'nexus-solo',
    stars: 6,
    shot: 'Nexus Solo’s landing page: “Your AI command center for creators”',
    name: 'Nexus Solo',
    category: 'Multi-agent AI',
    summary: 'Five coordinated AI agents for creator management and analytics.',
    repository: 'https://github.com/SrivarsanK/aws_a4b',
    started: '2026-05',
    context: 'AI for Bharat hackathon',
    stack: ['Python', 'TypeScript', 'LangGraph', 'PostgreSQL', 'Redis', 'AWS'],
    sections: [
      ['What it does', 'Nexus Solo is an AI-powered creator management project. Five agents handle content, trends, crisis monitoring, brand deals, and analytics.'],
      ['How it works', 'The agents coordinate through a LangGraph message bus. The Creator DNA engine uses a creator’s post history to adapt drafts to their voice and the platform’s format.'],
      ['Language support', 'The project covers ten Indian languages and considers regional platforms alongside Instagram, YouTube, and TikTok.'],
    ],
  },
  {
    slug: 'blindspot',
    stars: 0,
    shot: 'Blindspot’s live monitor flagging a high-severity pass-through case, with the transaction network beside it',
    name: 'Blindspot',
    category: 'Fraud investigation console',
    summary: 'A Rust and React console that flags money-mule networks in real-time payment streams.',
    started: '2026-10',
    context: 'Private repository',
    stack: ['Rust', 'React', 'TypeScript', 'SQLite', 'Cytoscape.js', 'Python'],
    sections: [
      ['What it does', 'Blindspot helps fraud analysts find money-mule accounts in instant payment systems such as UPI. It looks for money that arrives from several accounts and leaves again within minutes.'],
      ['How it detects', 'The Rust engine keeps rolling in-memory windows for each account. It raises a candidate when at least three different senders pay in within 15 minutes and at least 80% of that money moves out within the next 10. Accounts with a history of regular merchant settlements get a 30-point risk reduction, which keeps ordinary sellers out of the high-priority queue.'],
      ['Scope', 'Every case carries reason codes and an interactive transaction graph. Blindspot supports an analyst’s decision; it doesn’t freeze funds on its own. Risk scores are hand-tuned rankings, and the demo data is synthetic. The source is in a private repository.'],
    ],
  },
  {
    slug: 'discordbot',
    stars: 126,
    shot: 'The bot’s landing page, “Power up your server”, with cards for music, LeetCode sync, anti-nuke and server stats',
    name: 'DSC Discord Bot',
    category: 'Community tooling',
    summary: 'A Discord bot with a web dashboard and OCR-checked LeetCode challenge tracking.',
    repository: 'https://github.com/SrivarsanK/discordbot',
    started: '2026-06',
    context: 'Developer Students Club',
    stack: ['JavaScript', 'Node.js', 'discord.js', 'Drizzle ORM', 'PostgreSQL', 'Tesseract.js'],
    sections: [
      ['What it does', 'The bot runs the Developer Students Club server at SRMIST Ramapuram. It covers anti-nuke protection, auto-moderation, levelling, custom profiles, and music, and server admins configure all of it from a web dashboard.'],
      ['LeetCode verification', 'Members link their LeetCode account with a one-time token placed in their profile. Each solve they submit passes two checks: Tesseract.js reads a round code from the screenshot, and the LeetCode API confirms an accepted submission within 24 hours. Points feed weekly and monthly leaderboards.'],
      ['How it’s built', 'The bot is written in JavaScript on Node.js, with Drizzle ORM on Neon serverless Postgres behind the dashboard.'],
    ],
  },
  {
    slug: 'yara',
    stars: 3,
    shot: 'YARA’s passenger app on a phone, showing bus routes on a live map and arrival times at the nearest stop',
    name: 'YARA',
    category: 'Transit intelligence, mobile',
    summary: 'Passenger and operator apps for tracking public buses in real time.',
    repository: 'https://github.com/SrivarsanK/YARA-mobile',
    started: '2026-08',
    context: 'Smart India Hackathon 2026',
    stack: ['React Native', 'Expo', 'TypeScript'],
    sections: [
      ['What it does', 'YARA is a public transit intelligence platform. The passenger app shows buses moving on a live map with arrival countdowns and occupancy badges. When a bus is still finishing its previous route, the app says so and shows when it will arrive.'],
      ['How it’s demonstrated', 'The operator app can inject delays, GPS dropouts, and crowding. Each one changes what the passenger sees, and an event log shows the cause next to the effect.'],
      ['How it’s built', 'YARA is a React Native monorepo with two Expo apps and a shared package of types, hooks, components, and theme. I built it for Smart India Hackathon 2026.'],
    ],
  },
  {
    slug: 'mindbridge',
    stars: 7,
    shot: 'MindBridge’s landing page, “Your recovery journey starts here”, with its privacy commitments',
    name: 'MindBridge',
    category: 'Mental health platform',
    summary: 'A privacy-first mental health companion for students, with mood tracking and peer support.',
    repository: 'https://github.com/SrivarsanK/MindBridge',
    started: '2025-10',
    context: 'Student wellbeing',
    stack: ['Next.js', 'TypeScript', 'Convex', 'Clerk', 'Tailwind CSS'],
    sections: [
      ['What it does', 'MindBridge is a mental health support platform for students. It combines an AI companion with mood tracking, dream analysis, and anonymous peer matching.'],
      ['Safety', 'Conversations pass through several levels of crisis detection, and the platform escalates when someone needs more than a chatbot. Peer chats are end-to-end encrypted.'],
      ['Language support', 'The interface supports six Indian languages.'],
    ],
  },
  {
    slug: 'urjabandhu',
    stars: 6,
    shot: 'UrjaBandhu’s landing page, “Smarter energy decisions”, with a question box for electricity advice',
    name: 'UrjaBandhu',
    category: 'Energy analytics',
    summary: 'An electricity bill optimizer that breaks a home’s usage down by appliance.',
    repository: 'https://github.com/SrivarsanK/UrjaBandhu',
    live: 'https://urjabandhu.vercel.app',
    started: '2025-08',
    context: 'Household energy',
    stack: ['Next.js', 'TypeScript', 'Supabase', 'PostgreSQL', 'Python', 'LSTM', 'Tailwind CSS'],
    sections: [
      ['What it does', 'UrjaBandhu helps households lower their electricity bills. It forecasts consumption, works out which appliances use the most power, and suggests changes in plain language.'],
      ['How it works', 'Non-intrusive load monitoring splits a single meter reading into per-appliance loads. Time-series models such as Prophet and LSTM forecast usage, and OCR reads an appliance’s rating label from a photo to estimate its power draw.'],
      ['Language support', 'An assistant gives personalised suggestions and speaks Hindi, Bengali, Tamil, Telugu, and more.'],
    ],
  },
];

// Skill names match project `stack` entries exactly, so each skill can list the projects that use it.
export const skillGroups = [
  { name: 'Languages', skills: ['TypeScript', 'Python', 'Rust', 'JavaScript', 'C / C++', 'Java', 'SQL'] },
  { name: 'AI & ML', skills: ['LangGraph', 'XGBoost', 'Edge Impulse', 'Tesseract.js', 'LSTM'] },
  { name: 'Web & mobile', skills: ['Next.js', 'React', 'React Native', 'Astro', 'Node.js', 'FastAPI', 'discord.js', 'Tailwind CSS'] },
  { name: 'Data & infra', skills: ['PostgreSQL', 'MySQL', 'SQLite', 'Redis', 'Convex', 'Docker', 'AWS'] },
];

const headers = { Accept: 'application/vnd.github+json', ...(process.env.GITHUB_TOKEN && { Authorization: 'Bearer ' + process.env.GITHUB_TOKEN }) };
const liveStars = async (project) => {
  if (!project.repository) return project.stars;
  try {
    const response = await fetch('https://api.github.com/repos' + new URL(project.repository).pathname, { headers });
    return response.ok ? (await response.json()).stargazers_count : project.stars;
  } catch {
    return project.stars;
  }
};

// Ranked by stars, most first. A private repository has no public count and ranks last.
export const projects = (await Promise.all(catalogue.map(async (project) => ({ ...project, stars: await liveStars(project) }))))
  .sort((a, b) => b.stars - a.stars);

export const usedIn = (skill) => projects.filter((project) => project.stack.includes(skill));

export const monthLabel = (started) =>
  new Date(started + '-01T00:00:00Z').toLocaleDateString('en-GB', { month: 'short', year: 'numeric', timeZone: 'UTC' });
