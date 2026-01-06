/**
 * Strips HTML tags from a string to generate plain text
 * Used for generating plain text fallback from HTML email content
 *
 * @param html - HTML string to convert
 * @returns Plain text string without HTML tags
 */
export function stripHtml(html: string): string {
  if (!html) return '';

  // Create a temporary div element to leverage browser's HTML parsing
  const temp = document.createElement('div');
  temp.innerHTML = html;

  // Replace common HTML elements with plain text equivalents
  // Convert <br> to newlines
  temp.querySelectorAll('br').forEach(br => {
    br.replaceWith('\n');
  });

  // Convert <p> to newlines
  temp.querySelectorAll('p').forEach(p => {
    const text = p.textContent || '';
    p.replaceWith(`${text}\n\n`);
  });

  // Convert list items to newlines with bullets/numbers
  temp.querySelectorAll('li').forEach(li => {
    const text = li.textContent || '';
    li.replaceWith(`• ${text}\n`);
  });

  // Get text content and clean up extra whitespace
  let text = temp.textContent || '';

  // Normalize whitespace: collapse multiple spaces/tabs to single space
  text = text.replace(/[ \t]+/g, ' ');

  // Normalize newlines: collapse multiple newlines to max 2
  text = text.replace(/\n{3,}/g, '\n\n');

  // Trim leading/trailing whitespace
  text = text.trim();

  return text;
}
