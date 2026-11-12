export const profile = {
  name: 'Srivarsan K',
  github: 'https://github.com/SrivarsanK',
  linkedin: 'https://www.linkedin.com/in/ksrivarsan/',
  email: 'srivarsankannan@gmail.com',
  title: 'AI & Full-Stack Developer',
  description: 'Srivarsan K (K. Srivarsan) is an AI and full-stack developer at SRMIST Ramapuram in Chennai. Explore Glide, Nexus Solo, OraCAN and his work with the Developer Students Club.',
};

export const projects = [
  {
    slug: 'glide',
    name: 'Glide',
    category: 'Open-source developer tool',
    summary: 'A local-first visual editor that writes changes back to source code.',
    repository: 'https://github.com/SrivarsanK/Glide',
    stack: ['TypeScript', 'Node.js', 'Vite', 'Babel', 'Recast'],
    sections: [
      ['What it does', 'Glide lets developers edit a running web interface visually. Moving an element or changing its styles updates the original source file, so the edits can be reviewed in Git.'],
      ['How it works', 'A Node.js proxy connects to a Vite development server and links rendered elements to their source locations. AST and CST transformations update the relevant attributes, styles, or classes while preserving nearby formatting and comments.'],
      ['Framework support', 'Glide supports React JSX and TSX, Vue single-file components, Svelte, Astro, and HTML.'],
    ],
  },
  {
    slug: 'nexus-solo',
    name: 'Nexus Solo',
    category: 'Multi-agent AI',
    summary: 'Five coordinated AI agents for creator management and analytics.',
    repository: 'https://github.com/SrivarsanK/aws_a4b',
    stack: ['Python', 'TypeScript', 'LangGraph', 'PostgreSQL', 'Redis', 'AWS'],
    sections: [
      ['What it does', 'Nexus Solo is an AI-powered creator management project. Five agents handle content, trends, crisis monitoring, brand deals, and analytics.'],
      ['How it works', 'The agents coordinate through a LangGraph message bus. The Creator DNA engine uses a creator’s post history to adapt drafts to their voice and the platform’s format.'],
      ['Language support', 'The project covers ten Indian languages and considers regional platforms alongside Instagram, YouTube, and TikTok.'],
    ],
  },
  {
    slug: 'oracan',
    name: 'OraCAN',
    category: 'Edge AI prototype',
    summary: 'An intraoral imaging prototype running on an NVIDIA Jetson Nano.',
    repository: 'https://github.com/SrivarsanK/OC-Detect',
    stack: ['Jetson Nano', 'Edge Impulse', 'Python', 'Intraoral camera'],
    sections: [
      ['What it does', 'OraCAN is a student prototype that captures intraoral images and experiments with image classification for three oral cancer categories.'],
      ['How it works', 'An intraoral camera feeds images into a Python desktop app on an NVIDIA Jetson Nano. The team trained the model in Edge Impulse and designed the system for on-device inference.'],
      ['Project scope', 'This prototype is for research and demonstration. Clinical use would require validation with representative data and review by qualified clinicians.'],
    ],
  },
];
