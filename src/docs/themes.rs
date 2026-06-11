pub fn stylesheet(theme: &str) -> &'static str {
    match theme {
        "paper" => PAPER_THEME,
        _ => MARINA_THEME,
    }
}

const MARINA_THEME: &str = r#"@page {
  size: A4;
  margin: 22mm;
}

:root {
  --bg: #f8f7f2;
  --panel: #fffdf8;
  --ink: #17212b;
  --muted: #546271;
  --line: #d6d7d9;
  --accent: #005b7f;
  --accent-soft: #d7edf7;
  --code: #173042;
}

* {
  box-sizing: border-box;
}

body {
  margin: 0 auto;
  max-width: 880px;
  padding: 48px 36px 80px;
  background:
    radial-gradient(circle at top left, rgba(0, 91, 127, 0.09), transparent 26%),
    linear-gradient(180deg, #f3f4ef 0%, var(--bg) 100%);
  color: var(--ink);
  font-family: Georgia, "Times New Roman", serif;
  line-height: 1.65;
}

main {
  background: var(--panel);
  border: 1px solid rgba(0, 0, 0, 0.06);
  box-shadow: 0 18px 50px rgba(23, 33, 43, 0.08);
  padding: 56px;
}

h1, h2, h3, h4, h5, h6 {
  color: var(--accent);
  font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
  line-height: 1.2;
  margin-top: 1.8em;
  margin-bottom: 0.55em;
}

h1 {
  font-size: 2.4rem;
  margin-top: 0;
  border-bottom: 3px solid var(--accent-soft);
  padding-bottom: 0.35em;
}

h2 {
  font-size: 1.7rem;
}

h3 {
  font-size: 1.3rem;
}

p, li, blockquote {
  font-size: 1rem;
}

ul, ol {
  padding-left: 1.5rem;
}

code, pre {
  font-family: "SFMono-Regular", Consolas, "Liberation Mono", monospace;
}

pre {
  overflow-x: auto;
  padding: 16px 18px;
  background: var(--code);
  color: #f4f7fa;
  border-radius: 10px;
}

blockquote {
  margin: 1.4rem 0;
  padding: 0.1rem 0 0.1rem 1rem;
  border-left: 4px solid var(--accent);
  color: var(--muted);
}

hr {
  border: 0;
  border-top: 1px solid var(--line);
  margin: 2rem 0;
}
"#;

const PAPER_THEME: &str = r#"@page {
  size: A4;
  margin: 20mm;
}

body {
  margin: 0 auto;
  max-width: 820px;
  padding: 32px;
  background: #ffffff;
  color: #111111;
  font-family: Georgia, serif;
  line-height: 1.6;
}

main {
  padding: 24px 18px 48px;
}

h1, h2, h3, h4, h5, h6 {
  font-family: Arial, Helvetica, sans-serif;
}

pre {
  padding: 12px;
  background: #f1f1f1;
  overflow-x: auto;
}

blockquote {
  margin-left: 0;
  padding-left: 14px;
  border-left: 3px solid #777777;
}
"#;
