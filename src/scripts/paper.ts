import type { gsap as GSAP } from 'gsap';

// Lays a print down crumpled and smooths it flat. The warp filter is only attached while the tween runs,
// so a settled print costs nothing to scroll past; callers run this inside a no-reduced-motion media query.
export const uncrumple = (gsap: typeof GSAP, shot: HTMLElement, delay = 0) => {
  const warp = shot.querySelector('feDisplacementMap');
  if (!warp) return;
  // The warp is in pixels, so it scales with the print: a thumbnail and the case-study hero crumple alike.
  const strength = Math.max(14, shot.offsetWidth * 0.09);
  gsap.timeline({
    delay,
    onStart: () => shot.classList.add('is-crumpling'),
    onComplete: () => shot.classList.remove('is-crumpling'),
  })
    .fromTo(warp, { attr: { scale: strength } }, { attr: { scale: 0 }, duration: 1.15, ease: 'power3.out' }, 0)
    .fromTo(shot, { '--crumple': 1 }, { '--crumple': 0.5, duration: 1.4, ease: 'power2.out' }, 0);
};
